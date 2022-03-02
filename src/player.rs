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
            // Upgraded
            (UnitState::Stand, UnitCondition::Upgraded) => self.stand_upgraded_h.clone(),
            (UnitState::Move, UnitCondition::Upgraded) => self.move_upgraded_h.clone(),
            (UnitState::Attack, UnitCondition::Upgraded) => self.attack_upgraded_h.clone(),
            (UnitState::Wound, UnitCondition::Upgraded) => self.wound_upgraded_h.clone(),
        }
    }

    pub fn timer_for(&self, u_state: &UnitState) -> Timer {
        match u_state {
            UnitState::Stand => self.stand_timer.clone(),
            UnitState::Move => self.move_timer.clone(),
            UnitState::Attack => self.attack_timer.clone(),
            UnitState::Wound => self.wound_timer.clone(),
        }
    }

    pub fn count_for(&self, u_state: &UnitState) -> Option<usize> {
        match u_state {
            UnitState::Stand | UnitState::Move => None,
            UnitState::Attack => Some(self.attack_count),
            UnitState::Wound => Some(self.wound_count),
        }
    }
}

#[derive(Clone, Copy, Component)]
pub enum UnitState {
    Stand,
    Move,
    Attack,
    Wound,
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

#[derive(Component)]
pub struct UnitSprite(pub Entity);

/////////////////////////////////////// Systems ////////////////////////////////////////

pub fn animate_unit_sprites(
    time: Res<Time>,
    texture_atlases: Res<Assets<TextureAtlas>>,
    mut ev_unit_changed: EventWriter<UnitChanged>,
    units_q: Query<(
        Entity,
        &UnitSprite,
        &UnitState,
        &UnitCondition,
        &UnitAnimations,
        &Orientation,
    )>,
    mut sprites_q: Query<(&mut Animation, &mut TextureAtlasSprite)>,
) {
    for (unit, unit_sprite, unit_state, &unit_condition, unit_anims, &orientation) in units_q.iter()
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
                }
                // Animation is finished
                else {
                    ev_unit_changed.send(UnitChanged {
                        unit,
                        unit_sprite: unit_sprite.0,
                        unit_anims: unit_anims.clone(),
                        new_state: UnitState::Stand, // TODO: make some state transistion logic
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
    }
}

pub fn move_unit(time: Res<Time>, mut sprite_q: Query<(&mut Transform, &Movements)>) {
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
