use crate::prelude::*;

/////////////////////////////////////// Spawners ///////////////////////////////////////

pub fn spawn_ape(
    commands: &mut Commands,
    asset_server: &AssetServer,
    texture_atlases: &mut Assets<TextureAtlas>,
) {
    let ape = commands
        .spawn()
        .insert(Ape)
        .insert_bundle(SpriteBundle {
            texture: asset_server.load("ape_king.png"),
            transform: Transform {
                scale: Vec3::splat(0.8),
                translation: Vec3::new(0., 0., 5.),
                ..Default::default()
            },
            ..Default::default()
        })
        .id();

    // TODO: add some kind of attack_offset{x} (to be able to locate attack impact pos)

    let laser_init_image = asset_server.load("ape_king_blinking_eyes.png");
    let laser_init_atlas = TextureAtlas::from_grid(laser_init_image, Vec2::new(900.0, 600.0), 2, 1);
    let laser_on_image = asset_server.load("ape_king_lasers.png");
    let laser_on_atlas = TextureAtlas::from_grid(laser_on_image, Vec2::new(900.0, 600.0), 3, 1);
    let ape_attack_spec = ApeAttackSpec {
        ape_entity: ApeEntity(ape),
        init_h: texture_atlases.add(laser_init_atlas),
        init_duration: DurationTimer::from_seconds(0.6),
        init_timer: Timer::from_seconds(0.1, true),
        on_h: texture_atlases.add(laser_on_atlas),
        on_duration: DurationTimer::from_seconds(1.0),
        on_timer: Timer::from_seconds(0.1, true),
    };

    commands.entity(ape).insert(ape_attack_spec);
}

pub fn spawn_attack_init(commands: &mut Commands, attack_spec: &ApeAttackSpec) {
    let ApeAttackSpec {
        ape_entity,
        init_duration,
        init_timer,
        ..
    } = attack_spec;

    let animation = commands
        .spawn_bundle(SpriteSheetBundle {
            texture_atlas: attack_spec.init_h.clone(),
            transform: Transform::from_xyz(150., 0., 10.),
            ..Default::default()
        })
        .insert(*ape_entity)
        .insert(StagedAnimation::init(
            init_duration.clone(),
            init_timer.clone(),
        ))
        .id();

    commands.entity(ape_entity.0).push_children(&[animation]);
}

pub fn spawn_attack_on(commands: &mut Commands, attack_spec: &ApeAttackSpec) {
    let ApeAttackSpec {
        ape_entity,
        on_duration,
        on_timer,
        ..
    } = attack_spec;

    let animation = commands
        .spawn_bundle(SpriteSheetBundle {
            texture_atlas: attack_spec.on_h.clone(),
            transform: Transform::from_xyz(150., 0., 10.),
            ..Default::default()
        })
        .insert(*ape_entity)
        .insert(StagedAnimation::on(on_duration.clone(), on_timer.clone()))
        .id();

    commands.entity(ape_entity.0).push_children(&[animation]);
}

////////////////////////////////////// Components //////////////////////////////////////

#[derive(Component)]
pub struct Ape;

#[derive(Clone, Copy, Component)]
pub struct ApeEntity(pub Entity);

#[derive(Component)]
pub struct ApeAttackSpec {
    pub ape_entity: ApeEntity,
    pub init_h: Handle<TextureAtlas>,
    pub init_duration: DurationTimer,
    pub init_timer: Timer,
    pub on_h: Handle<TextureAtlas>,
    pub on_duration: DurationTimer,
    pub on_timer: Timer,
}

/////////////////////////////////////// Systems ////////////////////////////////////////

pub fn move_ape(time: Res<Time>, mut ape_q: Query<&mut Transform, With<Ape>>) {
    for mut transform in ape_q.iter_mut() {
        if (time.time_since_startup().as_secs() / 5) % 2 == 0 {
            transform.translation.x -= 60. * time.delta_seconds();
        } else {
            transform.translation.x += 60. * time.delta_seconds();
        }
    }
}

pub fn trigger_ape_attack(
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

pub fn animate_ape_attack(
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
