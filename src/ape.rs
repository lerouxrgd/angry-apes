use crate::prelude::*;

/////////////////////////////////////// Spawners ///////////////////////////////////////

pub fn spawn_ape(
    commands: &mut Commands,
    asset_server: &AssetServer,
    texture_atlases: &mut Assets<TextureAtlas>,
) {
    let ape_wound_image = asset_server.load("ape_king_wound.png");
    let ape_wound_atlas = TextureAtlas::from_grid(ape_wound_image, Vec2::new(600., 600.), 3, 1);

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
        .insert(ApeWoundHandle(texture_atlases.add(ape_wound_atlas)))
        .insert(ApeWoundWidth(170. * 0.8))
        .id();

    let attack_range = ApeAttackRange::new(350., 155.)
        .scaled_by(0.8)
        .with_offset(PROJECTION_SCALE / 2.);

    let laser_init_image = asset_server.load("ape_king_blinking_eyes.png");
    let laser_init_atlas = TextureAtlas::from_grid(laser_init_image, Vec2::new(900.0, 600.0), 2, 1);

    let laser_on_image = asset_server.load("ape_king_lasers.png");
    let laser_on_atlas = TextureAtlas::from_grid(laser_on_image, Vec2::new(900.0, 600.0), 3, 1);

    let ape_attack_spec = ApeAttackSpec {
        ape_entity: ApeEntity(ape),
        attack_range: attack_range,
        init_h: texture_atlases.add(laser_init_atlas),
        init_duration: DurationTimer::from_seconds(0.6),
        init_timer: Timer::from_seconds(0.1, true),
        on_h: texture_atlases.add(laser_on_atlas),
        on_duration: DurationTimer::from_seconds(1.0),
        on_timer: Timer::from_seconds(0.1, true),
    };

    commands.entity(ape).insert(ape_attack_spec);
}

pub fn spawn_ape_attack_init(commands: &mut Commands, attack_spec: &ApeAttackSpec) {
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

pub fn spawn_ape_attack_on(commands: &mut Commands, attack_spec: &ApeAttackSpec) {
    let ApeAttackSpec {
        ape_entity,
        on_duration,
        on_timer,
        attack_range,
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
        .insert(*attack_range)
        .id();

    commands.entity(ape_entity.0).push_children(&[animation]);
}

pub fn spawn_ape_damaged_anim(commands: &mut Commands, wound_h: &ApeWoundHandle) -> Entity {
    commands
        .spawn_bundle(SpriteSheetBundle {
            texture_atlas: wound_h.0.clone(),
            transform: Transform::from_xyz(0., 0., 9.),
            ..Default::default()
        })
        .insert(Animation {
            timer: Timer::from_seconds(0.08, true),
            count: Some(4),
        })
        .insert(wound_h.clone())
        .id()
}

////////////////////////////////////// Components //////////////////////////////////////

#[derive(Component)]
pub struct Ape;

#[derive(Clone, Copy, Component)]
pub struct ApeEntity(pub Entity);

#[derive(Clone, Component)]
pub struct ApeWoundHandle(pub Handle<TextureAtlas>);

#[derive(Component)]
pub struct ApeWoundWidth(pub f32);

#[derive(Component)]
pub struct ApeAttackSpec {
    pub ape_entity: ApeEntity,
    pub attack_range: ApeAttackRange,
    pub init_h: Handle<TextureAtlas>,
    pub init_duration: DurationTimer,
    pub init_timer: Timer,
    pub on_h: Handle<TextureAtlas>,
    pub on_duration: DurationTimer,
    pub on_timer: Timer,
}

#[derive(Clone, Copy, Component)]
pub struct ApeAttackRange {
    offset_x: f32,
    range_x: f32,
}

impl ApeAttackRange {
    pub fn new(offset_x: f32, range_x: f32) -> Self {
        Self { offset_x, range_x }
    }

    pub fn scaled_by(self, scale: f32) -> Self {
        Self {
            offset_x: self.offset_x * scale,
            range_x: self.range_x * scale,
        }
    }

    pub fn with_offset(self, offset: f32) -> Self {
        Self {
            offset_x: self.offset_x - offset,
            range_x: self.range_x,
        }
    }
}

/////////////////////////////////////// Systems ////////////////////////////////////////

pub fn move_apes(time: Res<Time>, mut apes_q: Query<&mut Transform, With<Ape>>) {
    for mut transform in apes_q.iter_mut() {
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
        spawn_ape_attack_init(&mut commands, &attack_spec);
    }
}

pub fn ape_attacks_player_collision(
    mut commands: Commands,
    mut ev_unit_changed: EventWriter<UnitChanged>,
    attacks_q: Query<(&GlobalTransform, &ApeAttackRange)>,
    player_q: Query<
        (
            Entity,
            &Transform,
            &UnitSprite,
            &UnitState,
            &UnitCondition,
            &UnitAnimations,
            &Orientation,
        ),
        With<Player>,
    >,
) {
    let (
        player,
        player_transform,
        player_sprite,
        player_state,
        &player_condition,
        player_anims,
        &orientation,
    ) = player_q.single();

    if matches!(player_condition, UnitCondition::Upgraded) {
        return;
    }

    let player_x = player_transform.translation.x;
    for (attack_transform, &ApeAttackRange { offset_x, range_x }) in attacks_q.iter() {
        let attack_x = attack_transform.translation.x;
        if attack_x + offset_x < player_x && player_x < attack_x + offset_x + range_x {
            commands.entity(player).remove::<Movements>();
            if !matches!(player_state, UnitState::Wound) {
                ev_unit_changed.send(UnitChanged {
                    unit: player,
                    unit_sprite: player_sprite.0,
                    unit_anims: player_anims.clone(),
                    new_state: UnitState::Wound,
                    new_condition: player_condition,
                    orientation,
                });
            }
        }
    }
}

pub fn animate_apes_attacks(
    time: Res<Time>,
    mut commands: Commands,
    texture_atlases: Res<Assets<TextureAtlas>>,
    apes_q: Query<&ApeAttackSpec, With<Ape>>,
    mut attacks_anim_q: Query<(
        Entity,
        &ApeEntity,
        &mut StagedAnimation,
        &mut TextureAtlasSprite,
        &Handle<TextureAtlas>,
    )>,
) {
    for (id, ape, mut anim, mut sprite, texture_atlas_h) in attacks_anim_q.iter_mut() {
        let attack_spec = apes_q.get(ape.0).unwrap();

        match &mut *anim {
            StagedAnimation::Init { duration, timer } => {
                duration.tick(time.delta());
                timer.tick(time.delta());

                if duration.finished() {
                    commands.entity(id).despawn();
                    spawn_ape_attack_on(&mut commands, &attack_spec);
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

pub fn animate_apes_wounds(
    time: Res<Time>,
    texture_atlases: Res<Assets<TextureAtlas>>,
    mut commands: Commands,
    mut wounds_q: Query<(
        Entity,
        &mut Animation,
        &mut TextureAtlasSprite,
        &ApeWoundHandle,
    )>,
) {
    for (anim_id, mut anim, mut sprite, atlas_h) in wounds_q.iter_mut() {
        anim.timer.tick(time.delta());
        if !anim.timer.just_finished() {
            continue;
        }

        match anim.count.as_mut() {
            Some(count) => {
                if *count != 0 {
                    *count -= 1;
                    let texture_atlas = texture_atlases.get(atlas_h.0.clone()).unwrap();
                    sprite.index = (sprite.index + 1) % texture_atlas.textures.len();
                }
                // Animation is finished
                else {
                    commands.entity(anim_id).despawn();
                }
            }
            None => unreachable!(),
        }
    }
}
