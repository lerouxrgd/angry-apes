use crate::prelude::*;

/////////////////////////////////////// Spawners ///////////////////////////////////////

pub fn init_ape_icon(commands: &mut Commands, asset_server: &AssetServer) -> ApeIconHandle {
    let ape_icon_h = ApeIconHandle(asset_server.load("ape_icon_ok.png"));
    commands.insert_resource(ape_icon_h.clone());
    ape_icon_h
}

pub fn spawn_ape(
    commands: &mut Commands,
    asset_server: &AssetServer,
    texture_atlases: &mut Assets<TextureAtlas>,
    flank: Flank,
) {
    let ape_name = ["ape_king", "ape_yacht", "ape_terminator"]
        .choose(&mut rand::thread_rng())
        .unwrap();

    let ape_wound_image = asset_server.load(&format!("{ape_name}_wound.png"));
    let ape_wound_atlas = TextureAtlas::from_grid(ape_wound_image, Vec2::new(600., 600.), 3, 1);

    let ape = commands
        .spawn()
        .insert(Ape)
        .insert_bundle(SpriteBundle {
            texture: asset_server.load(&format!("{ape_name}.png")),
            transform: Transform {
                scale: Vec3::splat(0.8),
                translation: Vec3::new(flank.start_pos(), 0., 5.),
                ..Default::default()
            },
            sprite: Sprite {
                flip_x: flank.flip_x(),
                ..Default::default()
            },
            ..Default::default()
        })
        .insert(ApeWoundHandle(texture_atlases.add(ape_wound_atlas)))
        .insert(ApeWoundWidth(170. * 0.8))
        .insert(ApeLife::new(1000.))
        .insert(flank.initial_move())
        .insert(flank)
        .id();

    let attack_range = ApeAttackRange::new(350., 155.)
        .scaled_by(0.8)
        .with_offset(PROJECTION_SCALE / 2.);

    let laser_init_image = asset_server.load(&format!("{ape_name}_blinking_eyes.png"));
    let laser_init_atlas = TextureAtlas::from_grid(laser_init_image, Vec2::new(900.0, 600.0), 2, 1);

    let laser_on_image = asset_server.load(&format!("{ape_name}_lasers.png"));
    let laser_on_atlas = TextureAtlas::from_grid(laser_on_image, Vec2::new(900.0, 600.0), 3, 1);

    let ape_attack_spec = ApeAttackSpec {
        attack_range,
        init_h: texture_atlases.add(laser_init_atlas),
        init_duration: DurationTimer::from_seconds(0.6),
        init_timer: Timer::from_seconds(0.1, true),
        on_h: texture_atlases.add(laser_on_atlas),
        on_duration: DurationTimer::from_seconds(1.0),
        on_timer: Timer::from_seconds(0.1, true),
        flank,
    };

    commands.entity(ape).insert(ape_attack_spec);
}

pub fn spawn_ape_attack_init(commands: &mut Commands, ape: Entity, attack_spec: &ApeAttackSpec) {
    let ApeAttackSpec {
        init_duration,
        init_timer,
        ..
    } = attack_spec;

    let offset_x = match attack_spec.flank {
        Flank::Left => 150.,
        Flank::Right => -150.,
    };

    let animation = commands
        .spawn_bundle(SpriteSheetBundle {
            texture_atlas: attack_spec.init_h.clone(),
            transform: Transform::from_xyz(offset_x, 0., 10.),
            sprite: TextureAtlasSprite {
                flip_x: attack_spec.flank.flip_x(),
                ..Default::default()
            },
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

    let offset_x = match attack_spec.flank {
        Flank::Left => 150.,
        Flank::Right => -150.,
    };

    let animation = commands
        .spawn_bundle(SpriteSheetBundle {
            texture_atlas: attack_spec.on_h.clone(),
            transform: Transform::from_xyz(offset_x, 0., 10.),
            sprite: TextureAtlasSprite {
                flip_x: attack_spec.flank.flip_x(),
                ..Default::default()
            },

            ..Default::default()
        })
        .insert(StagedAnimation::on(on_duration.clone(), on_timer.clone()))
        .insert(*attack_range)
        .insert(attack_spec.flank)
        .id();

    commands.entity(ape).push_children(&[animation]);
}

pub fn spawn_ape_damaged_anim(
    commands: &mut Commands,
    ape_life: &ApeLife,
    wound_h: &ApeWoundHandle,
    ape_icon_h: &ApeIconHandle,
    flank: &Flank,
) -> Entity {
    let anim = commands
        .spawn_bundle(SpriteSheetBundle {
            texture_atlas: wound_h.0.clone(),
            transform: Transform::from_xyz(0., 0., 9.),
            sprite: TextureAtlasSprite {
                flip_x: flank.flip_x(),
                ..Default::default()
            },
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
                Transform::from_xyz(10., 300., 15.),
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

pub fn spawn_dead_apes_hud(
    commands: &mut Commands,
    asset_server: &AssetServer,
    font_handle: &Handle<Font>,
) {
    let dead_apes_hud = commands
        .spawn()
        .insert(DeadApesHud)
        .insert(DeadApesCounter(0))
        .insert_bundle(SpriteBundle {
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
            text: Text::from_section(
                "0",
                TextStyle {
                    font: font_handle.clone(),
                    font_size: 280.0,
                    color: Color::WHITE,
                },
            )
            .with_alignment(TextAlignment {
                vertical: VerticalAlign::Center,
                horizontal: HorizontalAlign::Left,
            }),
            transform: Transform::from_xyz(165., -25., 0.),
            ..Default::default()
        })
        .insert(DeadApesText)
        .id();
    commands.entity(dead_apes_hud).push_children(&[count]);
}

////////////////////////////////////// Components //////////////////////////////////////

#[derive(Component)]
pub struct Ape;

#[derive(Clone)]
pub struct ApeIconHandle(pub Handle<Image>);

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
    pub flank: Flank,
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
pub struct DeadApesHud;

#[derive(Component)]
pub struct DeadApesText;

#[derive(Component)]
pub struct DeadApesCounter(usize);

pub struct ApesAliveAt(Instant);

impl Default for ApesAliveAt {
    fn default() -> Self {
        Self(Instant::now())
    }
}

#[derive(Copy, Clone, PartialEq, Eq, Hash, Component)]
pub enum Flank {
    Left,
    Right,
}

impl Flank {
    pub fn start_pos(&self) -> f32 {
        match self {
            Self::Left => self.min(),
            Self::Right => self.max(),
        }
    }

    pub fn initial_move(&self) -> Moving {
        match self {
            Self::Left => Moving::Left,
            Self::Right => Moving::Right,
        }
    }

    pub fn min(&self) -> f32 {
        match self {
            Self::Left => -(GLOBAL_WIDTH / 2. - 210. * 0.8),
            Self::Right => 145. * 0.8,
        }
    }

    pub fn max(&self) -> f32 {
        match self {
            Self::Left => -145. * 0.8,
            Self::Right => GLOBAL_WIDTH / 2. - 210. * 0.8,
        }
    }

    pub fn flip_x(&self) -> bool {
        match self {
            Self::Left => false,
            Self::Right => true,
        }
    }
}

/////////////////////////////////////// Systems ////////////////////////////////////////

pub fn make_ape(
    mut commands: Commands,
    apes_q: Query<&Flank, With<Ape>>,
    asset_server: Res<AssetServer>,
    mut texture_atlases: ResMut<Assets<TextureAtlas>>,
    mut apes_alive_at: Local<ApesAliveAt>,
) {
    let apes_flanks = apes_q.iter().cloned().collect::<HashSet<_>>();

    if apes_flanks.len() == 2 {
        *apes_alive_at = ApesAliveAt::default();
    }

    if apes_alive_at.0.elapsed() > Duration::from_secs(3) {
        for flank in [Flank::Left, Flank::Right] {
            if !apes_flanks.contains(&flank) {
                spawn_ape(&mut commands, &asset_server, &mut texture_atlases, flank);
            }
        }
    }
}

pub fn move_apes(
    time: Res<Time>,
    mut apes_q: Query<(&mut Transform, &mut Moving, &Flank), With<Ape>>,
) {
    for (mut transform, mut moving, flank) in apes_q.iter_mut() {
        let inc = 60. * time.delta_seconds();
        let old_x = transform.translation.x;
        match &*moving {
            Moving::Left => {
                if old_x - inc > flank.min() {
                    transform.translation.x = old_x - inc;
                } else {
                    transform.translation.x = old_x + inc;
                    *moving = Moving::Right;
                }
            }
            Moving::Right => {
                if old_x + inc < flank.max() {
                    transform.translation.x = old_x + inc;
                } else {
                    transform.translation.x = old_x - inc;
                    *moving = Moving::Left;
                }
            }
            _ => unreachable!(),
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
            spawn_ape_attack_init(&mut commands, ape, attack_spec);
        }
    }
}

pub fn ape_attacks_player_collision(
    mut commands: Commands,
    mut ev_unit_changed: EventWriter<UnitChanged>,
    attacks_q: Query<(&GlobalTransform, &ApeAttackRange, &Flank)>,
    player_q: Query<(Entity, &Transform, &UnitState, &UnitCondition), With<Player>>,
    mut health_q: Query<&mut LifeChunks, With<LifeHud>>,
) {
    let (player, player_transform, player_state, &player_condition) = player_q.single();

    if matches!(player_condition, UnitCondition::Upgraded) {
        return;
    }

    if matches!(
        player_state,
        UnitState::Dash | UnitState::Jump | UnitState::Fall
    ) {
        return;
    }

    let player_x = player_transform.translation.x;
    for (attack_transform, &ApeAttackRange { offset_x, range_x }, flank) in attacks_q.iter() {
        let attack_x = attack_transform.to_scale_rotation_translation().2.x;

        let player_in_range = match flank {
            Flank::Left => {
                attack_x + offset_x < player_x && player_x < attack_x + offset_x + range_x
            }
            Flank::Right => {
                attack_x - offset_x - range_x < player_x && player_x < attack_x - offset_x
            }
        };

        if player_in_range {
            commands.entity(player).remove::<Movements>();

            if !matches!(player_state, UnitState::Wound | UnitState::Die) {
                let mut health_chunks = health_q.single_mut();
                if let Some(chunk) = health_chunks.0.pop() {
                    commands.entity(chunk).despawn();
                }

                if health_chunks.0.is_empty() {
                    ev_unit_changed.send(UnitChanged {
                        unit: player,
                        new_state: UnitState::Die,
                        new_condition: player_condition,
                    });
                } else {
                    ev_unit_changed.send(UnitChanged {
                        unit: player,
                        new_state: UnitState::Wound,
                        new_condition: player_condition,
                    });
                }
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
        let attack_spec = match apes_q.get(ape.get()) {
            Ok(attack_spec) => attack_spec,
            Err(_) => continue,
        };

        match &mut *anim {
            StagedAnimation::Init { duration, timer } => {
                duration.tick(time.delta());
                timer.tick(time.delta());

                if duration.finished() {
                    commands.entity(id).despawn();
                    spawn_ape_attack_on(&mut commands, ape.get(), attack_spec);
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
    mut score: ResMut<Score>,
    mut dead_counter: Query<&mut DeadApesCounter, With<DeadApesHud>>,
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
                    let texture_atlas = texture_atlases.get(&atlas_h.0).unwrap();
                    sprite.index = (sprite.index + 1) % texture_atlas.textures.len();
                }
                // Animation is finished
                else {
                    commands.entity(anim_id).despawn_recursive();
                    if life.current == 0. {
                        dead_counter.single_mut().0 += 1;
                        score.0 += 1;
                        commands.entity(ape.get()).despawn_recursive();
                    }
                }
            }
            None => unreachable!(),
        }
    }
}

pub fn display_dead_apes_hud(
    counter: Query<&DeadApesCounter>,
    mut text_q: Query<&mut Text, With<DeadApesText>>,
) {
    let mut text = text_q.single_mut();
    text.sections[0].value = counter.single().0.to_string();
}
