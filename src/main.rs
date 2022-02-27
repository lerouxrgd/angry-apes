mod components;
mod inputs;
mod spawner;

mod prelude {
    pub use std::collections::HashSet;

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
        .add_system(switch_ape_attack)
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
    spawn_player(&mut commands, &asset_server, &mut texture_atlases);
    spawn_camera(&mut commands);

    // TODO: refactor this block into a proper entity
    {
        let texture_handle = asset_server.load("bored_ape_king.png");
        let texture_atlas = TextureAtlas::from_grid(texture_handle, Vec2::new(600.0, 600.0), 1, 1);
        let texture_atlas_h = texture_atlases.add(texture_atlas);

        commands.spawn_bundle(SpriteSheetBundle {
            texture_atlas: texture_atlas_h,
            transform: Transform::from_xyz(0., 0., 5.),
            ..Default::default()
        });

        let texture_handle = asset_server.load("bored_ape_king_lasers.png");
        let texture_atlas = TextureAtlas::from_grid(texture_handle, Vec2::new(900.0, 600.0), 1, 1);
        let texture_atlas_h = texture_atlases.add(texture_atlas);

        commands
            .spawn_bundle(SpriteSheetBundle {
                texture_atlas: texture_atlas_h,
                transform: Transform::from_xyz(150., 0., 10.),
                ..Default::default()
            })
            .insert(AttackAnimation::Off(Timer::from_seconds(3., false)));
    }
}

////////////////////////////////////////////////////////////////////////////////////////

fn switch_ape_attack(
    time: Res<Time>,
    mut attack_q: Query<(&mut AttackAnimation, &mut Visibility)>,
) {
    for (mut anim, mut visibility) in attack_q.iter_mut() {
        match &mut *anim {
            AttackAnimation::On(timer) => {
                timer.tick(time.delta());
                if timer.just_finished() {
                    *anim = AttackAnimation::Off(Timer::from_seconds(3., false));
                    visibility.is_visible = false;
                }
            }
            AttackAnimation::Off(timer) => {
                timer.tick(time.delta());
                if timer.just_finished() {
                    *anim = AttackAnimation::On(Timer::from_seconds(1., false));
                    visibility.is_visible = true;
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

        // This is a limited animation
        if let Some(count) = anim.count.as_mut() {
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
        // This is a looping animation
        else {
            let texture_atlas = texture_atlases
                .get(unit_anims.atlas_for(unit_state))
                .unwrap();
            sprite.index = (sprite.index + 1) % texture_atlas.textures.len();
            sprite.flip_x = orientation.flip_x();
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
                Moving::Left => transform.translation.x -= 140. * time.delta_seconds(),
                Moving::Up => transform.translation.y += 140. * time.delta_seconds(),
                Moving::Down => transform.translation.y -= 140. * time.delta_seconds(),
                Moving::Right => transform.translation.x += 140. * time.delta_seconds(),
            }
        }
    }
}
