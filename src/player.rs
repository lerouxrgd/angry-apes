use crate::prelude::*;

/////////////////////////////////////// Spawners ///////////////////////////////////////

pub fn spawn_player(
    commands: &mut Commands,
    asset_server: &AssetServer,
    texture_atlases: &mut Assets<TextureAtlas>,
) {
    // Sprites for UnitCondition::Normal

    let stand_image = asset_server.load("Paladin__STAND.png");
    let stand_atlas = TextureAtlas::from_grid(stand_image, Vec2::new(57.0, 107.0), 11, 1);
    let stand_h = texture_atlases.add(stand_atlas);
    let stand_timer = Timer::from_seconds(0.2, true);

    let move_image = asset_server.load("Paladin__MOVE.png");
    let move_atlas = TextureAtlas::from_grid(move_image, Vec2::new(65.0, 107.0), 8, 1);
    let move_h = texture_atlases.add(move_atlas);
    let move_timer = Timer::from_seconds(0.1, true);

    let attack_image = asset_server.load("Paladin__ATTACK_1.png");
    let attack_atlas = TextureAtlas::from_grid(attack_image, Vec2::new(105.0, 107.0), 5, 1);
    let attack_h = texture_atlases.add(attack_atlas);
    let attack_timer = Timer::from_seconds(0.11, true);
    let attack_count = 5;

    let wound_image = asset_server.load("Paladin__WOUND.png");
    let wound_atlas = TextureAtlas::from_grid(wound_image, Vec2::new(110.0, 127.0), 3, 1);
    let wound_h = texture_atlases.add(wound_atlas);
    let wound_timer = Timer::from_seconds(0.13, true);
    let wound_count = 3;

    let jump_image = asset_server.load("Paladin__JUMP.png");
    let jump_atlas = TextureAtlas::from_grid(jump_image, Vec2::new(65.0, 107.0), 1, 1);
    let jump_h = texture_atlases.add(jump_atlas);
    let jump_timer = Timer::from_seconds(0.1, true);

    let fall_image = asset_server.load("Paladin__FALL.png");
    let fall_atlas = TextureAtlas::from_grid(fall_image, Vec2::new(65.0, 107.0), 1, 1);
    let fall_h = texture_atlases.add(fall_atlas);
    let fall_timer = Timer::from_seconds(0.1, true);

    let dash_image = asset_server.load("Paladin__DASH.png");
    let dash_atlas = TextureAtlas::from_grid(dash_image, Vec2::new(65.0, 107.0), 1, 1);
    let dash_h = texture_atlases.add(dash_atlas);
    let dash_timer = Timer::from_seconds(0.15, true);

    // Sprites for UnitCondition::Upgraded

    let stand_upgraded_image = asset_server.load("Crusader__STAND.png");
    let stand_upgraded_atlas =
        TextureAtlas::from_grid(stand_upgraded_image, Vec2::new(57.0, 107.0), 11, 1);
    let stand_upgraded_h = texture_atlases.add(stand_upgraded_atlas);

    let move_upgraded_image = asset_server.load("Crusader__MOVE.png");
    let move_upgraded_atlas =
        TextureAtlas::from_grid(move_upgraded_image, Vec2::new(65.0, 107.0), 8, 1);
    let move_upgraded_h = texture_atlases.add(move_upgraded_atlas);

    let attack_upgraded_image = asset_server.load("Crusader__ATTACK_1.png");
    let attack_upgraded_atlas =
        TextureAtlas::from_grid(attack_upgraded_image, Vec2::new(105.0, 107.0), 5, 1);
    let attack_upgraded_h = texture_atlases.add(attack_upgraded_atlas);

    let wound_upgraded_image = asset_server.load("Crusader__WOUND.png");
    let wound_upgraded_atlas =
        TextureAtlas::from_grid(wound_upgraded_image, Vec2::new(110.0, 127.0), 3, 1);
    let wound_upgraded_h = texture_atlases.add(wound_upgraded_atlas);

    let jump_upgraded_image = asset_server.load("Crusader__JUMP.png");
    let jump_upgraded_atlas =
        TextureAtlas::from_grid(jump_upgraded_image, Vec2::new(65.0, 107.0), 1, 1);
    let jump_upgraded_h = texture_atlases.add(jump_upgraded_atlas);

    let fall_upgraded_image = asset_server.load("Crusader__FALL.png");
    let fall_upgraded_atlas =
        TextureAtlas::from_grid(fall_upgraded_image, Vec2::new(65.0, 107.0), 1, 1);
    let fall_upgraded_h = texture_atlases.add(fall_upgraded_atlas);

    let dash_upgraded_image = asset_server.load("Crusader__DASH.png");
    let dash_upgraded_atlas =
        TextureAtlas::from_grid(dash_upgraded_image, Vec2::new(65.0, 107.0), 1, 1);
    let dash_upgraded_h = texture_atlases.add(dash_upgraded_atlas);

    // Spawn player initial sprite

    let unit_anims = UnitAnimations {
        stand_h,
        stand_upgraded_h,
        stand_timer,
        move_h,
        move_upgraded_h,
        move_timer,
        attack_h,
        attack_upgraded_h,
        attack_timer,
        attack_count,
        wound_h,
        wound_upgraded_h,
        wound_timer,
        wound_count,
        jump_h,
        jump_upgraded_h,
        jump_timer,
        fall_h,
        fall_upgraded_h,
        fall_timer,
        dash_h,
        dash_upgraded_h,
        dash_timer,
    };
    let unit_state = UnitState::Stand;
    let unit_condition = UnitCondition::Normal;
    let orientation = Orientation::Right;
    let unit_sprite = spawn_unit_sprite(
        commands,
        &unit_anims,
        &unit_state,
        &unit_condition,
        &orientation,
    );

    // Spawn player unit

    commands
        .spawn()
        .insert(Player)
        .insert(GlobalTransform::default())
        .insert(Transform {
            scale: Vec3::splat(1.5),
            translation: Vec3::new(0., -170., 999.),
            ..Default::default()
        })
        .insert(Gravity { vy: 0. })
        .insert(unit_anims)
        .insert(unit_state)
        .insert(unit_condition)
        .insert(UnitSprite(unit_sprite))
        .push_children(&[unit_sprite])
        .insert(orientation)
        .insert(unit_condition)
        .insert(EthOwned::default());
}

pub fn spawn_unit_sprite(
    commands: &mut Commands,
    anims: &UnitAnimations,
    state: &UnitState,
    condition: &UnitCondition,
    orientation: &Orientation,
) -> Entity {
    commands
        .spawn_bundle(SpriteSheetBundle {
            texture_atlas: anims.atlas_for(state, condition),
            sprite: TextureAtlasSprite {
                flip_x: orientation.flip_x(),
                ..Default::default()
            },
            ..Default::default()
        })
        .insert(Animation {
            timer: anims.timer_for(state),
            count: anims.count_for(state),
        })
        .id()
}

pub fn spawn_life_hud(commands: &mut Commands, asset_server: &AssetServer) {
    let life_hud = commands
        .spawn()
        .insert(LifeHud)
        .insert_bundle(SpriteBundle {
            texture: asset_server.load("heart_icon.png"),
            transform: Transform {
                scale: Vec3::splat(0.13),
                translation: Vec3::new(-557., 244., 999.),
                ..Default::default()
            },
            ..Default::default()
        })
        .id();

    let mut chunks = vec![];
    let mut offset = 200.;
    for _ in 0..3 {
        let chunk = commands
            .spawn_bundle(SpriteBundle {
                texture: asset_server.load("life_chunk.png"),
                transform: Transform {
                    scale: Vec3::splat(1.2),
                    translation: Vec3::new(offset, 5., 0.),
                    ..Default::default()
                },
                ..Default::default()
            })
            .id();
        commands.entity(life_hud).push_children(&[chunk]);
        offset += 120.;
        chunks.push(chunk);
    }

    commands.entity(life_hud).insert(LifeChunks(chunks));
}

////////////////////////////////////// Components //////////////////////////////////////

#[derive(Component)]
pub struct Player;

#[derive(Clone, Component)]
pub struct UnitAnimations {
    pub stand_h: Handle<TextureAtlas>,
    pub stand_upgraded_h: Handle<TextureAtlas>,
    pub stand_timer: Timer,
    pub move_h: Handle<TextureAtlas>,
    pub move_upgraded_h: Handle<TextureAtlas>,
    pub move_timer: Timer,
    pub attack_h: Handle<TextureAtlas>,
    pub attack_upgraded_h: Handle<TextureAtlas>,
    pub attack_timer: Timer,
    pub attack_count: usize,
    pub wound_h: Handle<TextureAtlas>,
    pub wound_upgraded_h: Handle<TextureAtlas>,
    pub wound_timer: Timer,
    pub wound_count: usize,
    pub jump_h: Handle<TextureAtlas>,
    pub jump_upgraded_h: Handle<TextureAtlas>,
    pub jump_timer: Timer,
    pub fall_h: Handle<TextureAtlas>,
    pub fall_upgraded_h: Handle<TextureAtlas>,
    pub fall_timer: Timer,
    pub dash_h: Handle<TextureAtlas>,
    pub dash_upgraded_h: Handle<TextureAtlas>,
    pub dash_timer: Timer,
}

impl UnitAnimations {
    pub fn atlas_for(
        &self,
        u_state: &UnitState,
        u_condition: &UnitCondition,
    ) -> Handle<TextureAtlas> {
        match (u_state, u_condition) {
            // Normal
            (UnitState::Stand, UnitCondition::Normal) => self.stand_h.clone(),
            (UnitState::Move, UnitCondition::Normal) => self.move_h.clone(),
            (UnitState::Attack, UnitCondition::Normal) => self.attack_h.clone(),
            (UnitState::Wound, UnitCondition::Normal) => self.wound_h.clone(),
            (UnitState::Jump, UnitCondition::Normal) => self.jump_h.clone(),
            (UnitState::Fall, UnitCondition::Normal) => self.fall_h.clone(),
            (UnitState::Dash, UnitCondition::Normal) => self.dash_h.clone(),
            // Upgraded
            (UnitState::Stand, UnitCondition::Upgraded) => self.stand_upgraded_h.clone(),
            (UnitState::Move, UnitCondition::Upgraded) => self.move_upgraded_h.clone(),
            (UnitState::Attack, UnitCondition::Upgraded) => self.attack_upgraded_h.clone(),
            (UnitState::Wound, UnitCondition::Upgraded) => self.wound_upgraded_h.clone(),
            (UnitState::Jump, UnitCondition::Upgraded) => self.jump_upgraded_h.clone(),
            (UnitState::Fall, UnitCondition::Upgraded) => self.fall_upgraded_h.clone(),
            (UnitState::Dash, UnitCondition::Upgraded) => self.dash_upgraded_h.clone(),
        }
    }

    pub fn timer_for(&self, u_state: &UnitState) -> Timer {
        match u_state {
            UnitState::Stand => self.stand_timer.clone(),
            UnitState::Move => self.move_timer.clone(),
            UnitState::Attack => self.attack_timer.clone(),
            UnitState::Wound => self.wound_timer.clone(),
            UnitState::Jump => self.jump_timer.clone(),
            UnitState::Fall => self.fall_timer.clone(),
            UnitState::Dash => self.dash_timer.clone(),
        }
    }

    pub fn count_for(&self, u_state: &UnitState) -> Option<usize> {
        match u_state {
            UnitState::Stand | UnitState::Move | UnitState::Jump | UnitState::Fall => None,
            UnitState::Attack => Some(self.attack_count),
            UnitState::Wound => Some(self.wound_count),
            UnitState::Dash => Some(1),
        }
    }
}

#[derive(Clone, Copy, Component)]
pub enum UnitState {
    Stand,
    Move,
    Attack,
    Wound,
    Jump,
    Fall,
    Dash,
}

#[derive(Component)]
pub struct UnitChanged {
    pub unit: Entity,
    pub unit_sprite: Entity,
    pub unit_anims: UnitAnimations,
    pub new_state: UnitState,
    pub new_condition: UnitCondition,
    pub orientation: Orientation,
}

#[derive(Clone, Copy, Component)]
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

#[derive(Component)]
pub struct UnitSprite(pub Entity);

#[derive(Component)]
pub struct Gravity {
    pub vy: f32,
}

pub struct UnitAttack(pub Entity);

#[derive(Component)]
pub struct LifeHud;

#[derive(Component)]
pub struct LifeChunks(pub Vec<Entity>);

/////////////////////////////////////// Systems ////////////////////////////////////////

pub fn animate_unit_sprites(
    time: Res<Time>,
    mut commands: Commands,
    texture_atlases: Res<Assets<TextureAtlas>>,
    mut ev_unit_changed: EventWriter<UnitChanged>,
    mut ev_unit_attack: EventWriter<UnitAttack>,
    units_q: Query<(
        Entity,
        &UnitSprite,
        &UnitState,
        &UnitCondition,
        &UnitAnimations,
        &Orientation,
        Option<&Player>,
    )>,
    mut sprites_q: Query<(&mut Animation, &mut TextureAtlasSprite)>,
) {
    for (unit, unit_sprite, unit_state, &unit_condition, unit_anims, &orientation, is_player) in
        units_q.iter()
    {
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
                        .get(unit_anims.atlas_for(unit_state, &unit_condition))
                        .unwrap();
                    sprite.index = (sprite.index + 1) % texture_atlas.textures.len();

                    if *count == 1 && matches!(unit_state, UnitState::Attack) && is_player.is_some()
                    {
                        ev_unit_attack.send(UnitAttack(unit));
                    }
                }
                // Animation is finished
                else {
                    let new_state = match *unit_state {
                        UnitState::Dash => UnitState::Fall,
                        _ => UnitState::Stand,
                    };
                    commands.entity(unit).remove::<Movements>();
                    ev_unit_changed.send(UnitChanged {
                        unit,
                        unit_sprite: unit_sprite.0,
                        unit_anims: unit_anims.clone(),
                        new_state: new_state,
                        new_condition: unit_condition,
                        orientation,
                    });
                    continue;
                }
            }
            // This is an infinite animation
            None => {
                let texture_atlas = texture_atlases
                    .get(unit_anims.atlas_for(unit_state, &unit_condition))
                    .unwrap();
                sprite.index = (sprite.index + 1) % texture_atlas.textures.len();
                sprite.flip_x = orientation.flip_x();
            }
        }
    }
}

pub fn update_units(mut commands: Commands, mut ev_unit_changed: ResMut<Events<UnitChanged>>) {
    for UnitChanged {
        unit,
        unit_sprite,
        unit_anims,
        new_state,
        new_condition,
        orientation,
    } in ev_unit_changed.drain()
    {
        commands.entity(unit_sprite).despawn();
        let unit_sprite = spawn_unit_sprite(
            &mut commands,
            &unit_anims,
            &new_state,
            &new_condition,
            &orientation,
        );

        commands
            .entity(unit)
            .push_children(&[unit_sprite])
            .insert(UnitSprite(unit_sprite))
            .insert(new_state);

        match new_state {
            UnitState::Jump => {
                commands.entity(unit).insert(Gravity { vy: 500. });
            }
            _ => (),
        }
    }
}

pub fn move_units(time: Res<Time>, mut units_q: Query<(&UnitState, &mut Transform, &Movements)>) {
    for (unit_state, mut transform, movements) in units_q.iter_mut() {
        for moving in movements.0.iter() {
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
            if transform.translation.x < -wall {
                transform.translation.x = -wall;
            }
            if transform.translation.x > wall {
                transform.translation.x = wall;
            }
        }
    }
}

pub fn unit_attacks_ape(
    mut commands: Commands,
    mut ev_unit_attack: ResMut<Events<UnitAttack>>,
    ape_icon: Res<ApeIconHandle>,
    units_q: Query<(&Transform, &UnitCondition)>,
    mut apes_q: Query<
        (
            Entity,
            &Transform,
            &mut ApeLife,
            &ApeWoundWidth,
            &ApeWoundHandle,
        ),
        With<Ape>,
    >,
) {
    for UnitAttack(unit) in ev_unit_attack.drain() {
        let (unit_transform, unit_condition) = match units_q.get(unit) {
            Ok(q_res) => q_res,
            Err(_) => return,
        };

        let unit_x = unit_transform.translation.x;
        for (ape, ape_transform, mut ape_life, ape_wound_width, ape_wound_h) in apes_q.iter_mut() {
            let ape_x = ape_transform.translation.x;

            let close_enough = (unit_x - ape_x).abs() < ape_wound_width.0;
            if close_enough {
                ape_life.decrease_by(unit_condition.damages());
                let wound_anim =
                    spawn_ape_damaged_anim(&mut commands, &ape_life, ape_wound_h, &ape_icon);
                commands.entity(ape).push_children(&[wound_anim]);
            }
        }
    }
}

pub fn fall_units(
    time: Res<Time>,
    mut commands: Commands,
    mut ev_unit_changed: EventWriter<UnitChanged>,
    mut units_q: Query<(
        Entity,
        &UnitState,
        &UnitCondition,
        &UnitAnimations,
        &UnitSprite,
        &mut Transform,
        &mut Gravity,
        &Orientation,
    )>,
) {
    for (
        unit,
        unit_state,
        &unit_condition,
        unit_anims,
        unit_sprite,
        mut transform,
        mut gravity,
        &orientation,
    ) in units_q.iter_mut()
    {
        gravity.vy -= 1000. * time.delta_seconds();
        transform.translation.y += gravity.vy * time.delta_seconds();

        let floor = -170.;
        if transform.translation.y < floor {
            transform.translation.y = floor;
            gravity.vy = 0.;

            match *unit_state {
                UnitState::Jump | UnitState::Fall => {
                    commands.entity(unit).remove::<Movements>();
                    ev_unit_changed.send(UnitChanged {
                        unit: unit,
                        unit_sprite: unit_sprite.0,
                        unit_anims: unit_anims.clone(),
                        new_state: UnitState::Stand,
                        new_condition: unit_condition,
                        orientation,
                    });
                }
                _ => (),
            }
        }
    }
}
