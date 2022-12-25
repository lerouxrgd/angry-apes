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

#[derive(Debug, Clone, PartialEq, Eq, Hash, Component)]
pub enum Moving {
    Left,
    Up,
    Down,
    Right,
}

#[derive(Debug, Clone, Component, Deref)]
pub struct Movements(pub HashSet<Moving>);

impl Movements {
    pub fn from_input(input: &PlayerInput<'_>) -> Self {
        match input {
            PlayerInput::Keyboard { keys } => Self::from_keyboard(keys),
            PlayerInput::Gamepad {
                gamepad,
                buttons,
                axes,
            } => Self::from_gamepad(*gamepad, buttons, axes),
        }
    }

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

    pub fn from_gamepad(
        gamepad: Gamepad,
        buttons: &Input<GamepadButton>,
        axes: &Axis<GamepadAxis>,
    ) -> Self {
        let mut movements = HashSet::with_capacity(4);

        let dpad_left = buttons.pressed(GamepadButton {
            gamepad,
            button_type: GamepadButtonType::DPadLeft,
        });
        let dpad_right = buttons.pressed(GamepadButton {
            gamepad,
            button_type: GamepadButtonType::DPadRight,
        });
        let dpad_up = buttons.pressed(GamepadButton {
            gamepad,
            button_type: GamepadButtonType::DPadUp,
        });
        let dpad_down = buttons.pressed(GamepadButton {
            gamepad,
            button_type: GamepadButtonType::DPadDown,
        });

        let left_stick_x = axes
            .get(GamepadAxis {
                gamepad,
                axis_type: GamepadAxisType::LeftStickX,
            })
            .unwrap();
        let left_stick_y = axes
            .get(GamepadAxis {
                gamepad,
                axis_type: GamepadAxisType::LeftStickY,
            })
            .unwrap();

        if dpad_left {
            movements.insert(Moving::Left);
        }
        if dpad_right {
            movements.insert(Moving::Right);
        }
        if dpad_up {
            movements.insert(Moving::Up);
        }
        if dpad_down {
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

    pub fn from_orientation(orientation: Orientation) -> Self {
        let mut movements = HashSet::with_capacity(1);

        match orientation {
            Orientation::Left => movements.insert(Moving::Left),
            Orientation::Right => movements.insert(Moving::Right),
        };

        Self(movements)
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Component)]
pub enum Orientation {
    Left,
    Right,
}

impl Orientation {
    pub fn from_input(input: &PlayerInput<'_>) -> Option<Self> {
        match input {
            PlayerInput::Keyboard { keys } => Self::from_keyboard(keys),
            PlayerInput::Gamepad {
                gamepad,
                buttons,
                axes,
            } => Self::from_gamepad(*gamepad, buttons, axes),
        }
    }

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

    pub fn from_gamepad(
        gamepad: Gamepad,
        buttons: &Input<GamepadButton>,
        axes: &Axis<GamepadAxis>,
    ) -> Option<Self> {
        let dpad_left = buttons.pressed(GamepadButton {
            gamepad,
            button_type: GamepadButtonType::DPadLeft,
        });
        let dpad_right = buttons.pressed(GamepadButton {
            gamepad,
            button_type: GamepadButtonType::DPadRight,
        });

        let left_stick_x = axes
            .get(GamepadAxis {
                gamepad,
                axis_type: GamepadAxisType::LeftStickX,
            })
            .unwrap();

        if left_stick_x > 0.01 {
            Some(Self::Right)
        } else if left_stick_x < -0.01 {
            Some(Self::Left)
        } else if dpad_left {
            Some(Self::Left)
        } else if dpad_right {
            Some(Self::Right)
        } else {
            None
        }
    }

    pub fn from_movements(movements: &Movements) -> Option<Self> {
        if movements.contains(&Moving::Left) && !movements.contains(&Moving::Right) {
            Some(Orientation::Left)
        } else if movements.contains(&Moving::Right) && !movements.contains(&Moving::Left) {
            Some(Orientation::Right)
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

pub enum PlayerInput<'a> {
    Keyboard {
        keys: &'a Input<KeyCode>,
    },
    Gamepad {
        gamepad: Gamepad,
        buttons: &'a Input<GamepadButton>,
        axes: &'a Axis<GamepadAxis>,
    },
}

impl<'a> PlayerInput<'a> {
    pub fn jump_detected(&self) -> bool {
        match self {
            Self::Keyboard { keys } => keys.just_pressed(KeyCode::Space),
            Self::Gamepad {
                gamepad, buttons, ..
            } => buttons.just_pressed(GamepadButton {
                gamepad: *gamepad,
                button_type: GamepadButtonType::South,
            }),
        }
    }

    pub fn dash_detected(&self) -> bool {
        match &self {
            Self::Keyboard { keys } => {
                keys.just_pressed(KeyCode::RControl) || keys.just_pressed(KeyCode::Tab)
            }
            Self::Gamepad {
                gamepad, buttons, ..
            } => buttons.just_pressed(GamepadButton {
                gamepad: *gamepad,
                button_type: GamepadButtonType::East,
            }),
        }
    }

    pub fn attack_detected(&self) -> bool {
        match self {
            Self::Keyboard { keys } => {
                keys.just_pressed(KeyCode::Return) || keys.just_pressed(KeyCode::Key1)
            }
            Self::Gamepad {
                gamepad, buttons, ..
            } => {
                buttons.just_pressed(GamepadButton {
                    gamepad: *gamepad,
                    button_type: GamepadButtonType::West,
                }) || buttons.just_released(GamepadButton {
                    gamepad: *gamepad,
                    button_type: GamepadButtonType::West,
                })
            }
        }
    }

    pub fn direction_pressed(&self) -> bool {
        match self {
            Self::Keyboard { keys } => {
                keys.pressed(KeyCode::Left)
                    || keys.pressed(KeyCode::Up)
                    || keys.pressed(KeyCode::Down)
                    || keys.pressed(KeyCode::Right)
            }
            Self::Gamepad {
                gamepad,
                buttons,
                axes,
            } => {
                let dpad_left = buttons.pressed(GamepadButton {
                    gamepad: *gamepad,
                    button_type: GamepadButtonType::DPadLeft,
                });
                let dpad_right = buttons.pressed(GamepadButton {
                    gamepad: *gamepad,
                    button_type: GamepadButtonType::DPadRight,
                });

                let left_stick_x = axes
                    .get(GamepadAxis {
                        gamepad: *gamepad,
                        axis_type: GamepadAxisType::LeftStickX,
                    })
                    .unwrap();
                let left_stick_y = axes
                    .get(GamepadAxis {
                        gamepad: *gamepad,
                        axis_type: GamepadAxisType::LeftStickY,
                    })
                    .unwrap();

                dpad_left || dpad_right || left_stick_x != 0. || left_stick_y != 0.
            }
        }
    }

    pub fn direction_just_released(&self) -> bool {
        match self {
            Self::Keyboard { keys } => {
                keys.just_released(KeyCode::Left)
                    || keys.just_released(KeyCode::Up)
                    || keys.just_released(KeyCode::Down)
                    || keys.just_released(KeyCode::Right)
            }
            Self::Gamepad {
                gamepad,
                buttons,
                axes,
            } => {
                let dpad_left = buttons.just_released(GamepadButton {
                    gamepad: *gamepad,
                    button_type: GamepadButtonType::DPadLeft,
                });
                let dpad_right = buttons.just_released(GamepadButton {
                    gamepad: *gamepad,
                    button_type: GamepadButtonType::DPadRight,
                });

                let left_stick_x = axes
                    .get(GamepadAxis {
                        gamepad: *gamepad,
                        axis_type: GamepadAxisType::LeftStickX,
                    })
                    .unwrap();
                let left_stick_y = axes
                    .get(GamepadAxis {
                        gamepad: *gamepad,
                        axis_type: GamepadAxisType::LeftStickY,
                    })
                    .unwrap();

                dpad_left || dpad_right || left_stick_x == 0. && left_stick_y == 0.
            }
        }
    }
}

pub fn handle_input(
    input_kind: Res<InputKind>,
    keys: Res<Input<KeyCode>>,
    buttons: Res<Input<GamepadButton>>,
    axes: Res<Axis<GamepadAxis>>,
    mut commands: Commands,
    mut ev_unit_changed: EventWriter<UnitChanged>,
    player_q: Query<(Entity, &UnitState, &Orientation, &DashCooldown), With<Player>>,
) {
    let input = match *input_kind {
        InputKind::Keyboard => PlayerInput::Keyboard { keys: &keys },
        InputKind::Gamepad => PlayerInput::Gamepad {
            gamepad: Gamepad { id: 0 },
            buttons: &buttons,
            axes: &axes,
        },
    };

    let (player, unit_state, &orientation, cooldown) = player_q.single();

    let new_orientation = Orientation::from_input(&input);

    if input.jump_detected() {
        match *unit_state {
            UnitState::Stand | UnitState::Move => (),
            _ => return,
        }

        ev_unit_changed.send(
            UnitChanged::entity(player)
                .new_state(UnitState::Jump)
                .new_orientation(new_orientation),
        );
    } else if input.dash_detected() {
        match *unit_state {
            UnitState::Stand | UnitState::Move | UnitState::Jump if cooldown.finished() => (),
            _ => return,
        }

        commands
            .entity(player)
            .insert(Movements::from_orientation(orientation));

        ev_unit_changed.send(
            UnitChanged::entity(player)
                .new_state(UnitState::Dash)
                .new_orientation(new_orientation),
        );
    } else if input.direction_pressed() && !input.attack_detected() {
        match *unit_state {
            UnitState::Attack | UnitState::Wound | UnitState::Die | UnitState::Dash => {
                return;
            }
            UnitState::Move | UnitState::Jump | UnitState::Fall => {
                let movements = Movements::from_input(&input);
                let new_orientation = Orientation::from_movements(&movements);

                commands.entity(player).insert(movements);

                ev_unit_changed.send(UnitChanged::entity(player).new_orientation(new_orientation));
            }
            UnitState::Stand => {
                commands
                    .entity(player)
                    .insert(Movements::from_input(&input));

                ev_unit_changed.send(
                    UnitChanged::entity(player)
                        .new_state(UnitState::Move)
                        .new_orientation(new_orientation),
                );
            }
        }
    } else if input.direction_just_released() && !input.attack_detected() {
        match *unit_state {
            UnitState::Move => (),
            _ => return,
        }

        commands.entity(player).remove::<Movements>();
        ev_unit_changed.send(
            UnitChanged::entity(player)
                .new_state(UnitState::Stand)
                .new_orientation(new_orientation),
        );
    } else if input.attack_detected() {
        match *unit_state {
            UnitState::Attack | UnitState::Wound => return,
            _ => (),
        }

        commands.entity(player).remove::<Movements>();
        ev_unit_changed.send(
            UnitChanged::entity(player)
                .new_state(UnitState::Attack)
                .new_orientation(new_orientation),
        );
    }
}

pub fn gamepad_connection_events(
    mut input_kind: ResMut<InputKind>,
    mut gamepad_event: EventReader<GamepadEvent>,
) {
    for event in gamepad_event.iter() {
        match &event {
            GamepadEvent {
                event_type: GamepadEventType::Connected,
                ..
            } => {
                *input_kind = InputKind::Gamepad;
            }
            GamepadEvent {
                event_type: GamepadEventType::Disconnected,
                ..
            } => {
                *input_kind = InputKind::Keyboard;
            }
            _ => (),
        }
    }
}
