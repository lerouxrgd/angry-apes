use crate::prelude::*;

/////////////////////////////////////// Spawners ///////////////////////////////////////

pub fn spawn_player(
    commands: &mut Commands,
    aseprite_handles: &AsepriteHandles,
    aseprites: &Assets<Aseprite>,
) {
    let aseprite_handle = aseprite_handles.get(sprites::Paladin::PATH).unwrap();
    let aseprite = aseprites.get(aseprite_handle).unwrap();
    let animation = AsepriteAnimation::new(aseprite.info(), sprites::Paladin::tags::STAND);

    commands
        .spawn(Player)
        .insert(UnitKind::Player)
        .insert(VisibilityBundle::default())
        .insert(Gravity::default())
        .insert(DashCooldown::default())
        .insert(AsepriteBundle {
            texture_atlas: aseprite.atlas().clone(),
            sprite: TextureAtlasSprite::new(animation.current_frame()),
            aseprite: aseprite_handle.clone(),
            animation,
            ..default()
        })
        .insert(TransformBundle::from_transform(Transform {
            scale: Vec3::splat(1.5),
            translation: Vec3::new(0., -170., 999.),
            ..default()
        }))
        .insert(UnitState::Stand)
        .insert(UnitCondition::Normal)
        .insert(Orientation::Right)
        .insert(EthOwned::default());
}

pub fn spawn_life_hud(commands: &mut Commands, asset_server: &AssetServer) {
    let (life_hud_x, life_hud_y) = (-557., 244.);
    let life_hud = commands
        .spawn(LifeHud)
        .insert(SpriteBundle {
            texture: asset_server.load("heart_icon.png"),
            transform: Transform {
                translation: Vec3::new(life_hud_x, life_hud_y, 999.),
                scale: Vec3::splat(0.13),
                ..default()
            },
            ..default()
        })
        .id();

    let mut chunks = vec![];
    let mut offset = life_hud_x + 25.;
    for _ in 0..5 {
        let chunk = commands
            .spawn(SpriteBundle {
                texture: asset_server.load("life_chunk.png"),
                transform: Transform {
                    translation: Vec3::new(offset, life_hud_y, 999.),
                    scale: Vec3::splat(0.15),
                    ..default()
                },
                ..default()
            })
            .id();
        offset += 15.;
        chunks.push(chunk);
    }

    commands.entity(life_hud).insert(LifeChunks(chunks));
}

////////////////////////////////////// Components //////////////////////////////////////

#[derive(Component)]
pub struct Player;

#[derive(Component)]
pub enum UnitKind {
    Player,
}

impl UnitKind {
    pub fn anim_tag(&self, unit_state: UnitState, unit_condition: UnitCondition) -> AsepriteTag {
        use UnitCondition as Cond;
        use UnitState::*;
        match (self, unit_condition, unit_state) {
            (Self::Player, Cond::Normal, Stand) => sprites::Paladin::tags::STAND,
            (Self::Player, Cond::Normal, Move) => sprites::Paladin::tags::MOVE,
            (Self::Player, Cond::Normal, Attack) => sprites::Paladin::tags::ATTACK,
            (Self::Player, Cond::Normal, Wound) => sprites::Paladin::tags::WOUND,
            (Self::Player, Cond::Normal, Die) => sprites::Paladin::tags::DIE,
            (Self::Player, Cond::Normal, Jump) => sprites::Paladin::tags::JUMP,
            (Self::Player, Cond::Normal, Fall) => sprites::Paladin::tags::FALL,
            (Self::Player, Cond::Normal, Dash) => sprites::Paladin::tags::DASH,
            (Self::Player, Cond::Upgraded, Stand) => sprites::Crusader::tags::STAND,
            (Self::Player, Cond::Upgraded, Move) => sprites::Crusader::tags::MOVE,
            (Self::Player, Cond::Upgraded, Attack) => sprites::Crusader::tags::ATTACK,
            (Self::Player, Cond::Upgraded, Wound) => sprites::Crusader::tags::WOUND,
            (Self::Player, Cond::Upgraded, Die) => unreachable!(),
            (Self::Player, Cond::Upgraded, Jump) => sprites::Crusader::tags::JUMP,
            (Self::Player, Cond::Upgraded, Fall) => sprites::Crusader::tags::FALL,
            (Self::Player, Cond::Upgraded, Dash) => sprites::Crusader::tags::DASH,
        }
        .into()
    }

    pub fn asperite_handle(
        &self,
        aseprite_handles: &AsepriteHandles,
        unit_condition: UnitCondition,
    ) -> Handle<Aseprite> {
        use UnitCondition as Cond;
        match (self, unit_condition) {
            (Self::Player, Cond::Normal) => aseprite_handles.get(sprites::Paladin::PATH),
            (Self::Player, Cond::Upgraded) => aseprite_handles.get(sprites::Crusader::PATH),
        }
        .unwrap()
        .clone()
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Component)]
pub enum UnitState {
    Stand,
    Move,
    Attack,
    Wound,
    Die,
    Jump,
    Fall,
    Dash,
}

#[derive(Component, Event)]
pub struct UnitChanged {
    unit: Entity,
    new_state: Option<UnitState>,
    new_condition: Option<UnitCondition>,
    new_orientation: Option<Orientation>,
}

impl UnitChanged {
    pub fn entity(unit: Entity) -> Self {
        Self {
            unit,
            new_state: None,
            new_condition: None,
            new_orientation: None,
        }
    }

    pub fn new_state<N: Into<Option<UnitState>>>(mut self, new_state: N) -> Self {
        self.new_state = new_state.into();
        self
    }

    pub fn new_condition<N: Into<Option<UnitCondition>>>(mut self, new_condition: N) -> Self {
        self.new_condition = new_condition.into();
        self
    }

    pub fn new_orientation<N: Into<Option<Orientation>>>(mut self, new_orientation: N) -> Self {
        self.new_orientation = new_orientation.into();
        self
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Component)]
pub enum UnitCondition {
    Normal,
    Upgraded,
}

impl UnitCondition {
    pub fn damages(&self) -> f32 {
        match self {
            Self::Normal => 30.,
            Self::Upgraded => 300.,
        }
    }
}

#[derive(Component, Default)]
pub struct Gravity {
    pub vy: f32,
}

#[derive(Component, Deref, DerefMut)]
pub struct DashTimer(pub Timer);

impl Default for DashTimer {
    fn default() -> Self {
        DashTimer(Timer::from_seconds(0.3, TimerMode::Once))
    }
}

#[derive(Component, Deref, DerefMut)]
pub struct DashCooldown(pub Timer);

impl Default for DashCooldown {
    fn default() -> Self {
        DashCooldown(Timer::from_seconds(0.25, TimerMode::Once))
    }
}

#[derive(Event)]
pub struct UnitAttack(pub Entity);

#[derive(Component)]
pub struct LifeHud;

#[derive(Component, Deref, DerefMut)]
pub struct LifeChunks(pub Vec<Entity>);

/////////////////////////////////////// Systems ////////////////////////////////////////

pub fn update_units(
    mut commands: Commands,
    mut ev_unit_changed: EventReader<UnitChanged>,
    mut units_q: Query<(
        &UnitKind,
        &mut UnitCondition,
        &mut UnitState,
        &mut Handle<TextureAtlas>,
        &mut TextureAtlasSprite,
        &mut Handle<Aseprite>,
        &mut AsepriteAnimation,
        &mut Orientation,
    )>,
    aseprite_handles: Res<AsepriteHandles>,
    aseprites: Res<Assets<Aseprite>>,
) {
    for &UnitChanged {
        unit,
        new_state,
        new_condition,
        new_orientation,
    } in ev_unit_changed.iter()
    {
        let Ok((
            unit_kind,
            mut unit_condition,
            mut unit_state,
            mut atlas,
            mut sprite,
            mut aseprite_h,
            mut animation,
            mut orientation,

        )) = units_q.get_mut(unit) else {
            continue;
        };

        if let Some(new_state) = new_state {
            let new_aseprite_h = unit_kind.asperite_handle(&aseprite_handles, *unit_condition);
            let new_aseprite = aseprites.get(&new_aseprite_h).unwrap();
            *animation = AsepriteAnimation::new(
                new_aseprite.info(),
                unit_kind.anim_tag(new_state, *unit_condition),
            );

            match new_state {
                UnitState::Stand
                | UnitState::Attack
                | UnitState::Fall
                | UnitState::Wound
                | UnitState::Die => {
                    commands.entity(unit).remove::<Movements>();
                    if let UnitState::Dash = *unit_state {
                        commands
                            .entity(unit)
                            .insert(DashCooldown::default())
                            .remove::<DashTimer>();
                    }
                }

                UnitState::Jump => {
                    commands.entity(unit).insert(Gravity { vy: 500. });
                }

                UnitState::Dash => {
                    commands.entity(unit).insert(DashTimer::default());
                }

                _ => (),
            }

            *unit_state = new_state;
        }

        if let Some(new_condition) = new_condition {
            let new_aseprite_h = unit_kind.asperite_handle(&aseprite_handles, new_condition);
            let new_aseprite = aseprites.get(&new_aseprite_h).unwrap();
            let new_animation = AsepriteAnimation::new(
                new_aseprite.info(),
                unit_kind.anim_tag(*unit_state, new_condition),
            );

            *atlas = new_aseprite.atlas().clone();
            *aseprite_h = new_aseprite_h.clone();
            *animation = new_animation;

            *unit_condition = new_condition;
        }

        if let Some(new_orientation) = new_orientation {
            sprite.flip_x = new_orientation.flip_x();
            *orientation = new_orientation;
        }
    }
}

pub fn reorient_units_on_sprite_change(
    mut units_q: Query<
        (&mut TextureAtlasSprite, &Orientation),
        (With<UnitKind>, Changed<TextureAtlasSprite>),
    >,
) {
    for (mut sprite_atlas, orientation) in &mut units_q {
        sprite_atlas.flip_x = orientation.flip_x();
    }
}

pub fn transition_units(
    time: Res<Time>,
    units_q: Query<(
        Entity,
        &UnitState,
        Option<&Player>,
        Option<&DashTimer>,
        &Handle<Aseprite>,
        &AsepriteAnimation,
    )>,
    aseprites: Res<Assets<Aseprite>>,
    mut ev_unit_changed: EventWriter<UnitChanged>,
    mut ev_unit_attack: EventWriter<UnitAttack>,
    mut app_state: ResMut<NextState<AppState>>,
) {
    for (unit, &unit_state, is_player, dash, handle, anim) in &units_q {
        let Some(aseprite) = aseprites.get(handle) else { continue };
        match unit_state {
            UnitState::Attack => {
                let remaining_frames = anim.remaining_tag_frames(aseprite.info()).unwrap();
                let frame_finished = anim.frame_finished(time.delta());
                if remaining_frames == 1 && frame_finished {
                    ev_unit_attack.send(UnitAttack(unit));
                }
                if remaining_frames == 0 && frame_finished {
                    ev_unit_changed.send(UnitChanged::entity(unit).new_state(UnitState::Stand));
                }
            }

            UnitState::Dash => match dash {
                Some(dash) if dash.just_finished() => {
                    ev_unit_changed.send(UnitChanged::entity(unit).new_state(UnitState::Fall))
                }
                _ => (),
            },

            UnitState::Wound => {
                let remaining_frames = anim.remaining_tag_frames(aseprite.info()).unwrap();
                let frame_finished = anim.frame_finished(time.delta());
                if remaining_frames == 0 && frame_finished {
                    ev_unit_changed.send(UnitChanged::entity(unit).new_state(UnitState::Stand));
                }
            }

            UnitState::Die if is_player.is_some() => {
                let remaining_frames = anim.remaining_tag_frames(aseprite.info()).unwrap();
                let frame_finished = anim.frame_finished(time.delta());
                if remaining_frames == 0 && frame_finished {
                    app_state.set(AppState::GameOver);
                }
            }

            _ => (),
        }
    }
}

pub fn move_units(time: Res<Time>, mut units_q: Query<(&UnitState, &mut Transform, &Movements)>) {
    for (unit_state, mut transform, movements) in units_q.iter_mut() {
        for moving in movements.iter() {
            let velocity = match *unit_state {
                UnitState::Dash => 600.,
                _ => 150.,
            };

            match moving {
                Moving::Left => transform.translation.x -= velocity * time.delta_seconds(),
                Moving::Right => transform.translation.x += velocity * time.delta_seconds(),
                Moving::Up => (),
                Moving::Down => (),
            }

            let wall = 540.;
            transform.translation.x = transform.translation.x.clamp(-wall, wall);
        }
    }
}

pub fn unit_attacks_ape(
    mut commands: Commands,
    mut ev_unit_attack: EventReader<UnitAttack>,
    ape_icon: Res<ApeIconHandle>,
    units_q: Query<(&Transform, &UnitCondition)>,
    mut apes_q: Query<
        (
            Entity,
            &Transform,
            &mut ApeLife,
            &ApeWoundWidth,
            &ApeWoundHandle,
            &Flank,
        ),
        With<Ape>,
    >,
) {
    for &UnitAttack(unit) in ev_unit_attack.iter() {
        let (unit_transform, unit_condition) = match units_q.get(unit) {
            Ok(q_res) => q_res,
            Err(_) => return,
        };

        let unit_x = unit_transform.translation.x;
        for (ape, ape_transform, mut ape_life, ape_wound_width, ape_wound_h, flank) in
            apes_q.iter_mut()
        {
            let ape_x = ape_transform.translation.x;

            let close_enough = (unit_x - ape_x).abs() < ape_wound_width.0;
            if close_enough {
                ape_life.decrease_by(unit_condition.damages());
                let wound_anim =
                    spawn_ape_damaged_anim(&mut commands, &ape_life, ape_wound_h, &ape_icon, flank);
                commands.entity(ape).push_children(&[wound_anim]);
            }
        }
    }
}

pub fn fall_units(
    time: Res<Time>,
    mut ev_unit_changed: EventWriter<UnitChanged>,
    mut units_q: Query<(Entity, &UnitState, &mut Transform, &mut Gravity)>,
) {
    for (unit, unit_state, mut transform, mut gravity) in units_q.iter_mut() {
        gravity.vy -= 1000. * time.delta_seconds();
        transform.translation.y += gravity.vy * time.delta_seconds();

        let floor = -170.;
        if transform.translation.y < floor {
            transform.translation.y = floor;
            gravity.vy = 0.;

            match *unit_state {
                UnitState::Jump | UnitState::Fall => {
                    ev_unit_changed.send(UnitChanged::entity(unit).new_state(UnitState::Stand));
                }
                _ => (),
            }
        }
    }
}

pub fn tick_dashes(time: Res<Time>, mut units_q: Query<&mut DashTimer>) {
    for mut timer in units_q.iter_mut() {
        timer.tick(time.delta());
    }
}

pub fn cooldown_dashes(time: Res<Time>, mut units_q: Query<&mut DashCooldown>) {
    for mut cooldown in units_q.iter_mut() {
        cooldown.tick(time.delta());
    }
}
