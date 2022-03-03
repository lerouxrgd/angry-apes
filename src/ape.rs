use crate::prelude::*;

/////////////////////////////////////// Spawners ///////////////////////////////////////

pub fn init_ape_icon(commands: &mut Commands, asset_server: &AssetServer) {
    let ape_icon_h = asset_server.load("ape_icon_ok.png");
    commands.insert_resource(ApeIconHandle(ape_icon_h));
}

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
        .insert(ApeLife::new(1000.))
        .id();

    let attack_range = ApeAttackRange::new(350., 155.)
        .scaled_by(0.8)
        .with_offset(PROJECTION_SCALE / 2.);

    let laser_init_image = asset_server.load("ape_king_blinking_eyes.png");
    let laser_init_atlas = TextureAtlas::from_grid(laser_init_image, Vec2::new(900.0, 600.0), 2, 1);

    let laser_on_image = asset_server.load("ape_king_lasers.png");
    let laser_on_atlas = TextureAtlas::from_grid(laser_on_image, Vec2::new(900.0, 600.0), 3, 1);

    let ape_attack_spec = ApeAttackSpec {
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

pub fn spawn_ape_attack_init(commands: &mut Commands, ape: Entity, attack_spec: &ApeAttackSpec) {
    let ApeAttackSpec {
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
        .insert(StagedAnimation::init(
            init_duration.clone(),
            init_timer.clone(),
        ))
        .id();

    commands.entity(ape).push_children(&[animation]);
}

pub fn spawn_ape_attack_on(commands: &mut Commands, ape: Entity, attack_spec: &ApeAttackSpec) {
    let ApeAttackSpec {
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
        .insert(StagedAnimation::on(on_duration.clone(), on_timer.clone()))
        .insert(*attack_range)
        .id();

    commands.entity(ape).push_children(&[animation]);
}

pub fn spawn_ape_damaged_anim(
    commands: &mut Commands,
    ape_life: &ApeLife,
    wound_h: &ApeWoundHandle,
    ape_icon_h: &ApeIconHandle,
) -> Entity {
    let anim = commands
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
        .insert(*ape_life)
        .id();

    if ape_life.current > 0. {
        let rect_x = ape_life.current * 200. / ape_life.max;
        let rect = shapes::Rectangle {
            extents: Vec2::new(rect_x, 10.),
            origin: shapes::RectangleOrigin::default(),
        };
        let builder = GeometryBuilder::new().add(&rect);
        let healthbar = commands
            .spawn_bundle(builder.build(
                DrawMode::Fill(FillMode::color(Color::PINK)),
                Transform::from_xyz(10., 300., 15.), // TODO: make some ApeDims
            ))
            .id();
        commands.entity(anim).push_children(&[healthbar]);

        let icon = commands
            .spawn_bundle(SpriteBundle {
                texture: ape_icon_h.0.clone(),
                transform: Transform {
                    scale: Vec3::splat(0.4),
                    translation: Vec3::new(-rect_x / 2. - 20., 0., 0.),
                    ..Default::default()
                },
                ..Default::default()
            })
            .id();
        commands.entity(healthbar).push_children(&[icon]);
    }
    anim
}

pub fn spawn_dead_apes_counter(
    commands: &mut Commands,
    asset_server: &AssetServer,
    font_handle: Handle<Font>,
) {
    commands.insert_resource(DeadApesCounter(0));

    let icon = commands
        .spawn_bundle(SpriteBundle {
            texture: asset_server.load("ape_icon_dead.png"),
            transform: Transform {
                scale: Vec3::splat(0.15),
                translation: Vec3::new(505., 275., 999.),
                ..Default::default()
            },
            ..Default::default()
        })
        .id();

    let count = commands
        .spawn_bundle(Text2dBundle {
            text: Text::with_section(
                "0",
                TextStyle {
                    font: font_handle.clone(),
                    font_size: 280.0,
                    color: Color::WHITE,
                },
                TextAlignment {
                    vertical: VerticalAlign::Center,
                    horizontal: HorizontalAlign::Left,
                },
            ),
            transform: Transform::from_xyz(165., -25., 0.),
            ..Default::default()
        })
        .insert(DeadApesText)
        .id();
    commands.entity(icon).push_children(&[count]);
}

////////////////////////////////////// Components //////////////////////////////////////

#[derive(Component)]
pub struct Ape;

pub struct ApeIconHandle(Handle<Image>);

#[derive(Clone, Component)]
pub struct ApeWoundHandle(pub Handle<TextureAtlas>);

#[derive(Component)]
pub struct ApeWoundWidth(pub f32);

#[derive(Component)]
pub struct ApeAttackSpec {
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

#[derive(Clone, Copy, Component)]
pub struct ApeLife {
    pub current: f32,
    pub max: f32,
}

impl ApeLife {
    pub fn new(amount: f32) -> Self {
        Self {
            current: amount,
            max: amount,
        }
    }

    pub fn decrease_by(&mut self, amount: f32) {
        self.current = (self.current - amount).max(0.);
    }
}

#[derive(Component)]
pub struct DeadApesText;

pub struct DeadApesCounter(usize);

pub struct ApeAliveAt(Instant);

impl Default for ApeAliveAt {
    fn default() -> Self {
        Self(Instant::now())
    }
}

/////////////////////////////////////// Systems ////////////////////////////////////////

// TODO: A better ape spawning strategy
pub fn make_ape(
    mut commands: Commands,
    apes_q: Query<Entity, With<Ape>>,
    asset_server: Res<AssetServer>,
    mut texture_atlases: ResMut<Assets<TextureAtlas>>,
    mut ape_alive_at: Local<ApeAliveAt>,
) {
    let apes_count = apes_q.iter().count();
    if apes_count != 0 {
        *ape_alive_at = ApeAliveAt::default();
    }
    if ape_alive_at.0.elapsed() > Duration::from_secs(3) {
        spawn_ape(&mut commands, &asset_server, &mut texture_atlases);
    }
}

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
    apes_q: Query<(Entity, &ApeAttackSpec), With<Ape>>,
    mut trigger: Local<TriggerTimer>,
) {
    trigger.0.tick(time.delta());
    if trigger.0.just_finished() {
        for (ape, attack_spec) in apes_q.iter() {
            spawn_ape_attack_init(&mut commands, ape, &attack_spec);
        }
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
    mut health_q: Query<&mut LifeChunks, With<LifeHud>>,
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
                let mut health_chunks = health_q.single_mut();
                if let Some(chunk) = health_chunks.0.pop() {
                    commands.entity(chunk).despawn();
                }

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
        &Parent,
        &mut StagedAnimation,
        &mut TextureAtlasSprite,
        &Handle<TextureAtlas>,
    )>,
) {
    for (id, ape, mut anim, mut sprite, texture_atlas_h) in attacks_anim_q.iter_mut() {
        let attack_spec = match apes_q.get(ape.0) {
            Ok(attack_spec) => attack_spec,
            Err(_) => continue,
        };

        match &mut *anim {
            StagedAnimation::Init { duration, timer } => {
                duration.tick(time.delta());
                timer.tick(time.delta());

                if duration.finished() {
                    commands.entity(id).despawn();
                    spawn_ape_attack_on(&mut commands, ape.0, &attack_spec);
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
    mut dead_counter: ResMut<DeadApesCounter>,
    mut commands: Commands,
    mut wounds_q: Query<(
        Entity,
        &Parent,
        &mut Animation,
        &mut TextureAtlasSprite,
        &ApeLife,
        &ApeWoundHandle,
    )>,
) {
    for (anim_id, ape, mut anim, mut sprite, life, atlas_h) in wounds_q.iter_mut() {
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
                    commands.entity(anim_id).despawn_recursive();
                    if life.current == 0. {
                        dead_counter.0 += 1;
                        commands.entity(ape.0).despawn_recursive();
                    }
                }
            }
            None => unreachable!(),
        }
    }
}

pub fn display_dead_apes_counter(
    counter: Res<DeadApesCounter>,
    mut text_q: Query<&mut Text, With<DeadApesText>>,
) {
    let mut text = text_q.single_mut();
    text.sections[0].value = counter.0.to_string();
}
