use crate::prelude::*;

////////////////////////////////////// Components //////////////////////////////////////

pub enum InputKind {
    Keyboard,
    Gamepad,
}

impl Default for InputKind {
    fn default() -> Self {
        Self::Keyboard
    }
}

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

    pub fn from_orientation(orientation: &Orientation) -> Self {
        let mut movements = HashSet::with_capacity(1);

        match orientation {
            Orientation::Left => movements.insert(Moving::Left),
            Orientation::Right => movements.insert(Moving::Right),
        };

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

/////////////////////////////////////// Systems ////////////////////////////////////////

pub fn keyboard_input(
    input_kind: Res<InputKind>,
    keys: Res<Input<KeyCode>>,
    mut commands: Commands,
    mut ev_unit_changed: EventWriter<UnitChanged>,
    player_q: Query<(Entity, &UnitState, &UnitCondition, &Orientation, &Cooldown), With<Player>>,
) {
    if !matches!(&*input_kind, InputKind::Keyboard) {
        return;
    }

    let (player, unit_state, &unit_condition, orientation, cooldown) = player_q.single();

    if let Some(orientation) = Orientation::from_keyboard(&keys) {
        commands.entity(player).insert(orientation);
    }

    if keyboard_jump_detected(&keys) {
        match *unit_state {
            UnitState::Stand | UnitState::Move => (),
            _ => return,
        }

        ev_unit_changed.send(UnitChanged {
            unit: player,
            new_state: UnitState::Jump,
            new_condition: unit_condition,
        });
    } else if keyboard_dash_detected(&keys) {
        match *unit_state {
            UnitState::Stand | UnitState::Move | UnitState::Jump if cooldown.0.finished() => (),
            _ => return,
        }

        commands
            .entity(player)
            .insert(Movements::from_orientation(orientation));

        ev_unit_changed.send(UnitChanged {
            unit: player,
            new_state: UnitState::Dash,
            new_condition: unit_condition,
        });
    } else if keyboard_direction_pressed(&keys) && !keys.just_pressed(KeyCode::Return) && !keys.just_pressed(KeyCode::Key1) {
        match *unit_state {
            UnitState::Attack | UnitState::Wound | UnitState::Die | UnitState::Dash => {
                return;
            }
            UnitState::Move | UnitState::Jump | UnitState::Fall => {
                if let Some(orientation) = Orientation::from_keyboard(&keys) {
                    commands.entity(player).insert(orientation);
                }
                commands
                    .entity(player)
                    .insert(Movements::from_keyboard(&keys));
                return;
            }
            UnitState::Stand => {
                commands
                    .entity(player)
                    .insert(Movements::from_keyboard(&keys));

                ev_unit_changed.send(UnitChanged {
                    unit: player,
                    new_state: UnitState::Move,
                    new_condition: unit_condition,
                });
            }
        }
    } else if keyboard_direction_just_released(&keys) {
        match *unit_state {
            UnitState::Move => (),
            _ => return,
        }

        commands.entity(player).remove::<Movements>();
        ev_unit_changed.send(UnitChanged {
            unit: player,
            new_state: UnitState::Stand,
            new_condition: unit_condition,
        });
    } else if keys.just_pressed(KeyCode::Return) || keys.just_released(KeyCode::Return) || keys.just_pressed(KeyCode::Key1) || keys.just_released(KeyCode::Key1)  {
        match *unit_state {
            UnitState::Attack | UnitState::Wound => return,
            _ => (),
        }

        commands.entity(player).remove::<Movements>();
        ev_unit_changed.send(UnitChanged {
            unit: player,
            new_state: UnitState::Attack,
            new_condition: unit_condition,
        });
    }
}

pub fn keyboard_jump_detected(keys: &Input<KeyCode>) -> bool {
    keys.just_pressed(KeyCode::Space)
}

pub fn keyboard_dash_detected(keys: &Input<KeyCode>) -> bool {
    keys.pressed(KeyCode::RControl) || keys.pressed(KeyCode::Tab)
}

pub fn keyboard_direction_pressed(keys: &Input<KeyCode>) -> bool {
    keys.pressed(KeyCode::Left)
        || keys.pressed(KeyCode::Up)
        || keys.pressed(KeyCode::Down)
        || keys.pressed(KeyCode::Right)
}

pub fn keyboard_direction_just_released(keys: &Input<KeyCode>) -> bool {
    keys.just_released(KeyCode::Left)
        || keys.just_released(KeyCode::Up)
        || keys.just_released(KeyCode::Down)
        || keys.just_released(KeyCode::Right)
}

pub fn gamepad_input(
    input_kind: Res<InputKind>,
    gamepads: Res<Gamepads>,
    buttons: Res<Input<GamepadButton>>,
    axes: Res<Axis<GamepadAxis>>,
    mut commands: Commands,
    mut ev_unit_changed: EventWriter<UnitChanged>,
    player_q: Query<(Entity, &UnitState, &UnitCondition, &Orientation, &Cooldown), With<Player>>,
) {
    if !matches!(&*input_kind, InputKind::Gamepad) {
        return;
    }

    let gamepad = Gamepad(0);
    if !gamepads.contains(&gamepad) {
        return;
    }

    let (player, unit_state, &unit_condition, orientation, cooldown) = player_q.single();

    if let Some(orientation) = Orientation::from_gamepad(gamepad, &axes) {
        commands.entity(player).insert(orientation);
    }

    if gamepad_jump_detected(gamepad, &buttons) {
        match *unit_state {
            UnitState::Stand | UnitState::Move => (),
            _ => return,
        }

        ev_unit_changed.send(UnitChanged {
            unit: player,
            new_state: UnitState::Jump,
            new_condition: unit_condition,
        });
    } else if gamepad_dash_detected(gamepad, &buttons) {
        match *unit_state {
            UnitState::Stand | UnitState::Move | UnitState::Jump if cooldown.0.finished() => (),
            _ => return,
        }

        commands
            .entity(player)
            .insert(Movements::from_orientation(orientation));

        ev_unit_changed.send(UnitChanged {
            unit: player,
            new_state: UnitState::Dash,
            new_condition: unit_condition,
        });
    } else if gamepad_direction_pressed(gamepad, &axes)
        && !buttons.just_pressed(GamepadButton(gamepad, GamepadButtonType::West))
    {
        match *unit_state {
            UnitState::Attack | UnitState::Wound | UnitState::Die | UnitState::Dash => {
                return;
            }
            UnitState::Move | UnitState::Jump | UnitState::Fall => {
                commands
                    .entity(player)
                    .insert(Movements::from_gamepad(gamepad, &axes));
                return;
            }
            UnitState::Stand => {
                commands
                    .entity(player)
                    .insert(Movements::from_gamepad(gamepad, &axes));
                ev_unit_changed.send(UnitChanged {
                    unit: player,
                    new_state: UnitState::Move,
                    new_condition: unit_condition,
                });
            }
        }
    } else if gamepad_direction_just_released(gamepad, &axes)
        && !gamepad_attack_detected(gamepad, &buttons)
    {
        match *unit_state {
            UnitState::Move => (),
            _ => return,
        }

        commands.entity(player).remove::<Movements>();
        ev_unit_changed.send(UnitChanged {
            unit: player,
            new_state: UnitState::Stand,
            new_condition: unit_condition,
        });
    } else if gamepad_attack_detected(gamepad, &buttons) {
        match *unit_state {
            UnitState::Attack | UnitState::Wound => return,
            _ => (),
        }

        commands.entity(player).remove::<Movements>();
        ev_unit_changed.send(UnitChanged {
            unit: player,
            new_state: UnitState::Attack,
            new_condition: unit_condition,
        });
    }
}

pub fn gamepad_jump_detected(gamepad: Gamepad, buttons: &Input<GamepadButton>) -> bool {
    buttons.just_pressed(GamepadButton(gamepad, GamepadButtonType::South))
}

pub fn gamepad_dash_detected(gamepad: Gamepad, buttons: &Input<GamepadButton>) -> bool {
    buttons.pressed(GamepadButton(gamepad, GamepadButtonType::East))
}

pub fn gamepad_attack_detected(gamepad: Gamepad, buttons: &Input<GamepadButton>) -> bool {
    buttons.just_pressed(GamepadButton(gamepad, GamepadButtonType::West))
        || buttons.just_released(GamepadButton(gamepad, GamepadButtonType::West))
}

pub fn gamepad_direction_pressed(gamepad: Gamepad, axes: &Axis<GamepadAxis>) -> bool {
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

    left_dpad_x == 1.
        || left_dpad_y == 1.
        || left_dpad_x == -1.
        || left_dpad_y == -1.
        || left_stick_x != 0.
        || left_stick_y != 0.
}

pub fn gamepad_direction_just_released(gamepad: Gamepad, axes: &Axis<GamepadAxis>) -> bool {
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

    left_dpad_x == 0. && left_dpad_y == 0. && left_stick_x == 0. && left_stick_y == 0.
}

pub fn gamepad_connection_events(
    mut input_kind: ResMut<InputKind>,
    mut gamepad_event: EventReader<GamepadEvent>,
) {
    for event in gamepad_event.iter() {
        match &event {
            GamepadEvent(_gamepad, GamepadEventType::Connected) => {
                *input_kind = InputKind::Gamepad;
            }
            GamepadEvent(_gamepad, GamepadEventType::Disconnected) => {
                *input_kind = InputKind::Keyboard;
            }
            _ => (),
        }
    }
}
