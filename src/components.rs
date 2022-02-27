use crate::prelude::*;

////////////////////////////////////////////////////////////////////////////////////////

#[derive(Component)]
pub struct Player;

#[derive(Clone, Component)]
pub struct UnitAnimations {
    pub stand_h: Handle<TextureAtlas>,
    pub stand_timer: Timer,
    pub move_h: Handle<TextureAtlas>,
    pub move_timer: Timer,
    pub attack_h: Handle<TextureAtlas>,
    pub attack_timer: Timer,
    pub attack_count: usize,
}

impl UnitAnimations {
    pub fn atlas_for(&self, u_state: &UnitState) -> Handle<TextureAtlas> {
        match u_state {
            UnitState::Stand => self.stand_h.clone(),
            UnitState::Move => self.move_h.clone(),
            UnitState::Attack => self.attack_h.clone(),
        }
    }

    pub fn timer_for(&self, u_state: &UnitState) -> Timer {
        match u_state {
            UnitState::Stand => self.stand_timer.clone(),
            UnitState::Move => self.move_timer.clone(),
            UnitState::Attack => self.attack_timer.clone(),
        }
    }

    pub fn count_for(&self, u_state: &UnitState) -> Option<usize> {
        match u_state {
            UnitState::Stand | UnitState::Move => None,
            UnitState::Attack => Some(self.attack_count),
        }
    }
}

#[derive(Clone, Copy, Component)]
pub enum UnitState {
    Stand,
    Move,
    Attack,
}

#[derive(Component)]
pub struct UnitStateChanged {
    pub unit: Entity,
    pub unit_sprite: Entity,
    pub unit_anims: UnitAnimations,
    pub new_state: UnitState,
    pub orientation: Orientation,
}

#[derive(Component)]
pub struct UnitSprite(pub Entity);

#[derive(Component)]
pub struct Animation {
    pub timer: Timer,
    pub count: Option<usize>,
}

////////////////////////////////////////////////////////////////////////////////////////

#[derive(PartialEq, Eq, Hash, Component)]
pub enum Moving {
    Left,
    Up,
    Down,
    Right,
}

#[derive(Component)]
pub struct Movements(pub HashSet<Moving>);

impl Movements {
    pub fn from_keyboard(keys: &Input<KeyCode>) -> Self {
        let mut movements = HashSet::with_capacity(4);

        if keys.pressed(KeyCode::Left) {
            movements.insert(Moving::Left);
        }
        if keys.pressed(KeyCode::Up) {
            movements.insert(Moving::Up);
        }
        if keys.pressed(KeyCode::Down) {
            movements.insert(Moving::Down);
        }
        if keys.pressed(KeyCode::Right) {
            movements.insert(Moving::Right);
        }

        Self(movements)
    }

    pub fn from_gamepad(gamepad: Gamepad, axes: &Axis<GamepadAxis>) -> Self {
        let mut movements = HashSet::with_capacity(4);

        let left_dpad_x = axes
            .get(GamepadAxis(gamepad, GamepadAxisType::DPadX))
            .unwrap();
        let left_dpad_y = axes
            .get(GamepadAxis(gamepad, GamepadAxisType::DPadY))
            .unwrap();
        let left_stick_x = axes
            .get(GamepadAxis(gamepad, GamepadAxisType::LeftStickX))
            .unwrap();
        let left_stick_y = axes
            .get(GamepadAxis(gamepad, GamepadAxisType::LeftStickY))
            .unwrap();

        if left_dpad_x == 1. {
            movements.insert(Moving::Right);
        }
        if left_dpad_x == -1. {
            movements.insert(Moving::Left);
        }
        if left_dpad_y == 1. {
            movements.insert(Moving::Up);
        }
        if left_dpad_y == -1. {
            movements.insert(Moving::Down);
        }

        if left_stick_x > 0.01 {
            movements.insert(Moving::Right);
        }
        if left_stick_x < -0.01 {
            movements.insert(Moving::Left);
        }
        if left_stick_y > 0.01 {
            movements.insert(Moving::Up);
        }
        if left_stick_y < -0.01 {
            movements.insert(Moving::Down);
        }

        Self(movements)
    }
}

#[derive(Clone, Copy, Component)]
pub enum Orientation {
    Left,
    Right,
}

impl Orientation {
    pub fn from_keyboard(keys: &Input<KeyCode>) -> Option<Self> {
        if keys.just_pressed(KeyCode::Left) && !keys.pressed(KeyCode::Right) {
            Some(Self::Left)
        } else if keys.just_pressed(KeyCode::Right) && !keys.pressed(KeyCode::Left) {
            Some(Self::Right)
        } else if keys.pressed(KeyCode::Left) && keys.just_released(KeyCode::Right) {
            Some(Self::Left)
        } else if keys.pressed(KeyCode::Right) && keys.just_released(KeyCode::Left) {
            Some(Self::Right)
        } else {
            None
        }
    }

    pub fn from_gamepad(gamepad: Gamepad, axes: &Axis<GamepadAxis>) -> Option<Self> {
        let left_dpad_x = axes
            .get(GamepadAxis(gamepad, GamepadAxisType::DPadX))
            .unwrap();
        let left_stick_x = axes
            .get(GamepadAxis(gamepad, GamepadAxisType::LeftStickX))
            .unwrap();

        if left_dpad_x == 1. {
            Some(Self::Right)
        } else if left_dpad_x == -1. {
            Some(Self::Left)
        } else if left_stick_x > 0.01 {
            Some(Self::Right)
        } else if left_stick_x < -0.01 {
            Some(Self::Left)
        } else {
            None
        }
    }

    pub fn flip_x(&self) -> bool {
        match self {
            Self::Right => false,
            Self::Left => true,
        }
    }
}

////////////////////////////////////////////////////////////////////////////////////////

// TODO: Add all Ape specific components here

#[derive(Component)]
pub enum AttackAnimation {
    On(Timer),
    Off(Timer),
}
