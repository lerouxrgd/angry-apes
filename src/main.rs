mod components;
mod inputs;
mod spawner;

mod prelude {
    pub use std::collections::HashSet;
    pub use std::time::Duration;

    pub use bevy::app::Events;
    pub use bevy::input::gamepad::{Gamepad, GamepadAxisType, GamepadButton};
    pub use bevy::input::keyboard::KeyboardInput;
    pub use bevy::input::ElementState;
    pub use bevy::prelude::*;
    pub use bevy::render::camera::OrthographicProjection;
    pub use bevy::render::camera::ScalingMode;

    pub use crate::components::*;
    pub use crate::inputs::*;
    pub use crate::spawner::*;
}

use prelude::*;

fn main() {
    App::new()
        .insert_resource(WindowDescriptor {
            title: "Angry Apes".to_string(),
            ..Default::default()
        })
        .insert_resource(Msaa { samples: 1 })
        .add_plugins(DefaultPlugins)
        .add_startup_system(setup)
        .add_system(gamepad_connection_events.before("input"))
        .add_system(gamepad_input.label("input"))
        .add_system(keyboard_input.label("input"))
        .add_system(move_unit)
        .add_system(animate_unit_sprites)
        .add_system(animate_coins)
        .add_system(move_ape)
        .add_system(trigger_ape_attack)
        .add_system(animate_ape_attack)
        .add_stage_before(
            CoreStage::PostUpdate,
            "update_unit_states",
            SystemStage::parallel(),
        )
        .add_system_to_stage("update_unit_states", update_unit_states)
        .init_resource::<Events<UnitStateChanged>>()
        .init_resource::<InputKind>()
        .run();
}

fn setup(
    mut commands: Commands,
    asset_server: Res<AssetServer>,
    mut texture_atlases: ResMut<Assets<TextureAtlas>>,
) {
    spawn_background(&mut commands, &asset_server);
    spawn_platform(&mut commands, &asset_server);
    spawn_player(&mut commands, &asset_server, &mut texture_atlases);
    spawn_camera(&mut commands);

    spawn_coins(&mut commands, &asset_server, &mut texture_atlases);

    spawn_ape(&mut commands, &asset_server, &mut texture_atlases);
}

////////////////////////////////////////////////////////////////////////////////////////

fn move_ape(time: Res<Time>, mut ape_q: Query<&mut Transform, With<Ape>>) {
    for mut transform in ape_q.iter_mut() {
        if (time.time_since_startup().as_secs() / 5) % 2 == 0 {
            transform.translation.x -= 60. * time.delta_seconds();
        } else {
            transform.translation.x += 60. * time.delta_seconds();
        }
    }
}

fn trigger_ape_attack(
    time: Res<Time>,
    mut commands: Commands,
    apes_q: Query<&ApeAttackSpec, With<Ape>>,
    mut trigger: Local<TriggerTimer>,
) {
    trigger.0.tick(time.delta());
    if trigger.0.just_finished() {
        let attack_spec = apes_q.single();
        spawn_attack_init(&mut commands, &attack_spec);
    }
}

fn animate_ape_attack(
    time: Res<Time>,
    mut commands: Commands,
    texture_atlases: Res<Assets<TextureAtlas>>,
    apes_q: Query<&ApeAttackSpec, With<Ape>>,
    mut attack_anim_q: Query<(
        Entity,
        &ApeEntity,
        &mut StagedAnimation,
        &mut TextureAtlasSprite,
        &Handle<TextureAtlas>,
    )>,
) {
    for (id, ape, mut anim, mut sprite, texture_atlas_h) in attack_anim_q.iter_mut() {
        let attack_spec = apes_q.get(ape.0).unwrap();

        match &mut *anim {
            StagedAnimation::Init { duration, timer } => {
                duration.tick(time.delta());
                timer.tick(time.delta());

                if duration.finished() {
                    commands.entity(id).despawn();
                    spawn_attack_on(&mut commands, &attack_spec);
                } else if timer.just_finished() {
                    let texture_atlas = texture_atlases.get(texture_atlas_h).unwrap();
                    sprite.index = (sprite.index + 1) % texture_atlas.textures.len();
                }
            }
            StagedAnimation::On { duration, timer } => {
                timer.tick(time.delta());
                duration.tick(time.delta());

                if duration.finished() {
                    commands.entity(id).despawn();
                } else if timer.just_finished() {
                    let texture_atlas = texture_atlases.get(texture_atlas_h).unwrap();
                    sprite.index = (sprite.index + 1) % texture_atlas.textures.len();
                }
            }
        }
    }
}

fn animate_unit_sprites(
    time: Res<Time>,
    texture_atlases: Res<Assets<TextureAtlas>>,
    mut ev_unit_states: EventWriter<UnitStateChanged>,
    units_q: Query<(
        Entity,
        &UnitSprite,
        &UnitState,
        &UnitAnimations,
        &Orientation,
    )>,
    mut sprites_q: Query<(&mut Animation, &mut TextureAtlasSprite)>,
) {
    for (unit, unit_sprite, unit_state, unit_anims, &orientation) in units_q.iter() {
        let (mut anim, mut sprite) = sprites_q.get_mut(unit_sprite.0).unwrap();

        anim.timer.tick(time.delta());
        if !anim.timer.just_finished() {
            continue;
        }

        match anim.count.as_mut() {
            // This is a finite animation
            Some(count) => {
                if *count != 0 {
                    *count -= 1;
                    let texture_atlas = texture_atlases
                        .get(unit_anims.atlas_for(unit_state))
                        .unwrap();
                    sprite.index = (sprite.index + 1) % texture_atlas.textures.len();
                }
                // Animation is finished
                else {
                    ev_unit_states.send(UnitStateChanged {
                        unit,
                        unit_sprite: unit_sprite.0,
                        unit_anims: unit_anims.clone(),
                        new_state: UnitState::Stand, // TODO: make some state transistion logic
                        orientation,
                    });
                    continue;
                }
            }
            // This is an infinite animation
            None => {
                let texture_atlas = texture_atlases
                    .get(unit_anims.atlas_for(unit_state))
                    .unwrap();
                sprite.index = (sprite.index + 1) % texture_atlas.textures.len();
                sprite.flip_x = orientation.flip_x();
            }
        }
    }
}

fn update_unit_states(
    mut commands: Commands,
    mut ev_unit_states: ResMut<Events<UnitStateChanged>>,
) {
    for UnitStateChanged {
        unit,
        unit_sprite,
        unit_anims,
        new_state,
        orientation,
    } in ev_unit_states.drain()
    {
        commands.entity(unit_sprite).despawn();
        let unit_sprite = spawn_unit_sprite(&mut commands, &unit_anims, &new_state, &orientation);
        commands
            .entity(unit)
            .push_children(&[unit_sprite])
            .insert(UnitSprite(unit_sprite))
            .insert(new_state);
    }
}

fn move_unit(time: Res<Time>, mut sprite_q: Query<(&mut Transform, &Movements)>) {
    for (mut transform, movements) in sprite_q.iter_mut() {
        for moving in movements.0.iter() {
            match moving {
                Moving::Left => transform.translation.x -= 150. * time.delta_seconds(),
                Moving::Up => transform.translation.y += 150. * time.delta_seconds(),
                Moving::Down => transform.translation.y -= 150. * time.delta_seconds(),
                Moving::Right => transform.translation.x += 150. * time.delta_seconds(),
            }
        }
    }
}

fn animate_coins(
    time: Res<Time>,
    texture_atlases: Res<Assets<TextureAtlas>>,
    mut query: Query<
        (
            &mut Animation,
            &mut TextureAtlasSprite,
            &Handle<TextureAtlas>,
        ),
        With<Coin>,
    >,
) {
    for (mut anim, mut sprite, texture_atlas_h) in query.iter_mut() {
        anim.timer.tick(time.delta());
        if anim.timer.just_finished() {
            let texture_atlas = texture_atlases.get(texture_atlas_h).unwrap();
            sprite.index = (sprite.index + 1) % texture_atlas.textures.len();
        }
    }
}
