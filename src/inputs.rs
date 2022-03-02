use crate::prelude::*;

pub enum InputKind {
    Keyboard,
    Gamepad,
}

impl Default for InputKind {
    fn default() -> Self {
        Self::Keyboard
    }
}

pub fn keyboard_input(
    input_kind: Res<InputKind>,
    keys: Res<Input<KeyCode>>,
    mut commands: Commands,
    mut ev_unit_changed: EventWriter<UnitChanged>,
    player_q: Query<
        (
            Entity,
            &UnitState,
            &UnitCondition,
            &UnitAnimations,
            &UnitSprite,
            &Orientation,
        ),
        With<Player>,
    >,
) {
    if !matches!(&*input_kind, InputKind::Keyboard) {
        return;
    }

    if keyboard_direction_pressed(&keys) && !keys.just_pressed(KeyCode::Key1) {
        let (player, unit_state, &unit_condition, unit_anims, sprite, &orientation) =
            player_q.single();

        match *unit_state {
            UnitState::Attack | UnitState::Wound => {
                if let Some(orientation) = Orientation::from_keyboard(&keys) {
                    commands.entity(player).insert(orientation);
                }
                return;
            }
            UnitState::Move => {
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
                let orientation = if let Some(orientation) = Orientation::from_keyboard(&keys) {
                    commands.entity(player).insert(orientation);
                    orientation
                } else {
                    orientation
                };
                ev_unit_changed.send(UnitChanged {
                    unit: player,
                    unit_sprite: sprite.0,
                    unit_anims: unit_anims.clone(),
                    new_state: UnitState::Move,
                    new_condition: unit_condition,
                    orientation,
                });
            }
        }
    } else if keyboard_direction_just_released(&keys) {
        let (player, unit_state, &unit_condition, unit_anims, sprite, &orientation) =
            player_q.single();

        match *unit_state {
            UnitState::Move => (),
            _ => return,
        }

        commands.entity(player).remove::<Movements>();
        ev_unit_changed.send(UnitChanged {
            unit: player,
            unit_sprite: sprite.0,
            unit_anims: unit_anims.clone(),
            new_state: UnitState::Stand,
            new_condition: unit_condition,
            orientation,
        });
    } else if keys.just_pressed(KeyCode::Key1) || keys.just_released(KeyCode::Key1) {
        let (player, unit_state, &unit_condition, unit_anims, sprite, &orientation) =
            player_q.single();

        match *unit_state {
            UnitState::Attack | UnitState::Wound => return,
            UnitState::Stand | UnitState::Move => (),
        }

        commands.entity(player).remove::<Movements>();
        ev_unit_changed.send(UnitChanged {
            unit: player,
            unit_sprite: sprite.0,
            unit_anims: unit_anims.clone(),
            new_state: UnitState::Attack,
            new_condition: unit_condition,
            orientation,
        });
    }
    // TODO: remove this later, it is just to test wound anim
    else if keys.just_pressed(KeyCode::Key2) || keys.just_released(KeyCode::Key2) {
        let (player, unit_state, &unit_condition, unit_anims, sprite, &orientation) =
            player_q.single();

        match *unit_state {
            UnitState::Attack | UnitState::Wound => return,
            UnitState::Stand | UnitState::Move => (),
        }

        commands.entity(player).remove::<Movements>();
        ev_unit_changed.send(UnitChanged {
            unit: player,
            unit_sprite: sprite.0,
            unit_anims: unit_anims.clone(),
            new_state: UnitState::Wound,
            new_condition: unit_condition,
            orientation,
        });
    }
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
    player_q: Query<
        (
            Entity,
            &UnitState,
            &UnitCondition,
            &UnitAnimations,
            &UnitSprite,
            &Orientation,
        ),
        With<Player>,
    >,
) {
    if !matches!(&*input_kind, InputKind::Gamepad) {
        return;
    }

    let gamepad = Gamepad(0);
    if !gamepads.contains(&gamepad) {
        return;
    }

    if gamepad_direction_pressed(gamepad, &axes)
        && !buttons.just_pressed(GamepadButton(gamepad, GamepadButtonType::West))
    {
        let (player, unit_state, &unit_condition, unit_anims, sprite, &orientation) =
            player_q.single();

        match *unit_state {
            UnitState::Attack | UnitState::Wound => {
                if let Some(orientation) = Orientation::from_gamepad(gamepad, &axes) {
                    commands.entity(player).insert(orientation);
                }
                return;
            }
            UnitState::Move => {
                if let Some(orientation) = Orientation::from_gamepad(gamepad, &axes) {
                    commands.entity(player).insert(orientation);
                }
                commands
                    .entity(player)
                    .insert(Movements::from_gamepad(gamepad, &axes));
                return;
            }
            UnitState::Stand => {
                commands
                    .entity(player)
                    .insert(Movements::from_gamepad(gamepad, &axes));
                let orientation =
                    if let Some(orientation) = Orientation::from_gamepad(gamepad, &axes) {
                        commands.entity(player).insert(orientation);
                        orientation
                    } else {
                        orientation
                    };
                ev_unit_changed.send(UnitChanged {
                    unit: player,
                    unit_sprite: sprite.0,
                    unit_anims: unit_anims.clone(),
                    new_state: UnitState::Move,
                    new_condition: unit_condition,
                    orientation,
                });
            }
        }
    } else if gamepad_direction_just_released(gamepad, &axes)
        && !gamepad_attack_detected(gamepad, &buttons)
    {
        let (player, unit_state, &unit_condition, unit_anims, sprite, &orientation) =
            player_q.single();

        match *unit_state {
            UnitState::Attack | UnitState::Stand | UnitState::Wound => return,
            UnitState::Move => {
                commands.entity(player).remove::<Movements>();
                ev_unit_changed.send(UnitChanged {
                    unit: player,
                    unit_sprite: sprite.0,
                    unit_anims: unit_anims.clone(),
                    new_state: UnitState::Stand,
                    new_condition: unit_condition,
                    orientation,
                });
            }
        }
    } else if gamepad_attack_detected(gamepad, &buttons) {
        let (player, unit_state, &unit_condition, unit_anims, sprite, &orientation) =
            player_q.single();

        match *unit_state {
            UnitState::Attack | UnitState::Wound => return,
            UnitState::Stand | UnitState::Move => (),
        }

        commands.entity(player).remove::<Movements>();
        ev_unit_changed.send(UnitChanged {
            unit: player,
            unit_sprite: sprite.0,
            unit_anims: unit_anims.clone(),
            new_state: UnitState::Attack,
            new_condition: unit_condition,
            orientation,
        });
    }
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
