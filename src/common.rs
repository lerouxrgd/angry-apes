use crate::prelude::*;

/////////////////////////////////////// Spawners ///////////////////////////////////////

pub fn spawn_game_state(
    commands: &mut Commands,
    asset_server: &AssetServer,
    texture_atlases: &mut Assets<TextureAtlas>,
    font_handle: &Handle<Font>,
) {
    spawn_background(commands, asset_server);
    spawn_platform(commands, asset_server);

    spawn_player(commands, asset_server, texture_atlases);
    spawn_life_hud(commands, asset_server);

    spawn_eth_hud(commands, asset_server);

    spawn_ape(commands, asset_server, texture_atlases, Flank::Left);
    spawn_ape(commands, asset_server, texture_atlases, Flank::Right);
    spawn_dead_apes_hud(commands, asset_server, font_handle);
}

pub fn spawn_background(commands: &mut Commands, asset_server: &AssetServer) {
    commands
        .spawn_bundle(SpriteBundle {
            texture: asset_server.load("background.png"),
            transform: Transform::from_xyz(0., 0., 0.),
            ..Default::default()
        })
        .insert(Scenary);
}

pub fn spawn_platform(commands: &mut Commands, asset_server: &AssetServer) {
    commands
        .spawn_bundle(SpriteBundle {
            texture: asset_server.load("platform.png"),
            transform: Transform::from_xyz(0., -270., 1.),
            ..Default::default()
        })
        .insert(Scenary);
}

pub fn spawn_camera(commands: &mut Commands) {
    let projection = OrthographicProjection {
        scaling_mode: ScalingMode::FixedVertical,
        scale: PROJECTION_SCALE,
        ..Default::default()
    };

    let mut camera = OrthographicCameraBundle::new_2d();
    camera.orthographic_projection = projection;

    commands.spawn_bundle(camera);
}

pub fn spawn_font(commands: &mut Commands, asset_server: &AssetServer) -> Handle<Font> {
    let font_handle: Handle<Font> = asset_server.load("FontsFree-Net-Monkey.ttf");
    commands.insert_resource(font_handle.clone());
    font_handle
}

pub fn spawn_gameover_screen(
    commands: &mut Commands,
    asset_server: &AssetServer,
    font_handle: &Handle<Font>,
    ape_icon: &ApeIconHandle,
) {
    let alignment = TextAlignment {
        vertical: VerticalAlign::Bottom,
        horizontal: HorizontalAlign::Center,
    };
    let visibility = Visibility { is_visible: false };

    commands
        .spawn_bundle(Text2dBundle {
            text: Text::with_section(
                "You   have   been   funged",
                TextStyle {
                    font: font_handle.clone(),
                    font_size: 60.0,
                    color: Color::WHITE,
                },
                alignment,
            ),
            visibility: visibility.clone(),
            transform: Transform::from_xyz(0., 180., 10.),
            ..Default::default()
        })
        .insert(GameoverElement)
        .with_children(|parent| {
            parent
                .spawn_bundle(SpriteBundle {
                    texture: asset_server.load("toilet.png"),
                    transform: Transform {
                        scale: Vec3::splat(0.4),
                        translation: Vec3::new(0., -130., 0.),
                        ..Default::default()
                    },
                    visibility: visibility.clone(),
                    ..Default::default()
                })
                .insert(GameoverElement);

            parent
                .spawn_bundle(Text2dBundle {
                    text: Text::with_section(
                        "You   managed   to   kill   [ 0 ]",
                        TextStyle {
                            font: font_handle.clone(),
                            font_size: 30.0,
                            color: Color::WHITE,
                        },
                        alignment,
                    ),
                    visibility: visibility.clone(),
                    transform: Transform::from_xyz(-80., -300., 0.),
                    ..Default::default()
                })
                .insert(ScoreText)
                .insert(GameoverElement);

            parent
                .spawn_bundle(SpriteBundle {
                    texture: ape_icon.0.clone(),
                    transform: Transform {
                        scale: Vec3::splat(0.6),
                        translation: Vec3::new(0., -283., 0.),
                        ..Default::default()
                    },
                    visibility: visibility.clone(),
                    ..Default::default()
                })
                .insert(ScoreTextIcon)
                .insert(GameoverElement);

            parent
                .spawn_bundle(Text2dBundle {
                    text: Text::with_section(
                        "Press   << attack >>   to   take  your  revenge   on   the   Apes",
                        TextStyle {
                            font: font_handle.clone(),
                            font_size: 30.0,
                            color: Color::WHITE,
                        },
                        alignment,
                    ),
                    visibility: visibility.clone(),
                    transform: Transform::from_xyz(0., -380., 0.),
                    ..Default::default()
                })
                .insert(GameoverElement);
        });
}

////////////////////////////////////// Components //////////////////////////////////////

#[derive(Debug, Clone, Eq, PartialEq, Hash)]
pub enum AppState {
    InGame,
    GameOver,
}

#[derive(Component)]
pub struct Scenary;

#[derive(Component)]
pub struct GameoverElement;

#[derive(Component)]
pub struct Animation {
    pub timer: Timer,
    pub count: Option<usize>,
}

#[derive(Component)]
pub enum StagedAnimation {
    Init {
        duration: DurationTimer,
        timer: Timer,
    },
    On {
        duration: DurationTimer,
        timer: Timer,
    },
}

impl StagedAnimation {
    pub fn init(duration: DurationTimer, timer: Timer) -> Self {
        Self::Init { duration, timer }
    }

    pub fn on(duration: DurationTimer, timer: Timer) -> Self {
        Self::On { duration, timer }
    }
}

#[derive(Clone, Component)]
pub struct DurationTimer(pub Timer);

impl DurationTimer {
    pub fn from_seconds(duration: f32) -> Self {
        Self(Timer::from_seconds(duration, false))
    }

    pub fn finished(&self) -> bool {
        self.0.finished()
    }

    pub fn tick(&mut self, delta: Duration) {
        self.0.tick(delta);
    }
}

#[derive(Component)]
pub struct TriggerTimer(pub Timer);

impl Default for TriggerTimer {
    fn default() -> Self {
        Self(Timer::from_seconds(3., true))
    }
}

#[derive(Default)]
pub struct Score(pub usize);

#[derive(Component)]
pub struct ScoreText;

#[derive(Component)]
pub struct ScoreTextIcon;

/////////////////////////////////////// Systems ////////////////////////////////////////

pub fn despawn_game_state(
    mut commands: Commands,
    entities_query: Query<
        Entity,
        Or<(
            With<Player>,
            With<LifeHud>,
            With<Ape>,
            With<DeadApesHud>,
            With<Eth>,
            With<EthHud>,
            With<Scenary>,
        )>,
    >,
) {
    for e in entities_query.iter() {
        commands.entity(e).despawn_recursive();
    }
}

pub fn respawn_game_state(
    mut commands: Commands,
    font_handle: Res<Handle<Font>>,
    asset_server: Res<AssetServer>,
    mut texture_atlases: ResMut<Assets<TextureAtlas>>,
    mut text_q: Query<&mut Visibility, With<GameoverElement>>,
    mut score: ResMut<Score>,
) {
    for mut visibility in text_q.iter_mut() {
        visibility.is_visible = false;
    }

    score.0 = 0;

    spawn_game_state(
        &mut commands,
        &asset_server,
        &mut texture_atlases,
        &font_handle,
    );
}

pub fn gameover_screen(
    input_kind: Res<InputKind>,
    keys: Res<Input<KeyCode>>,
    gamepads: Res<Gamepads>,
    buttons: Res<Input<GamepadButton>>,
    score: Res<Score>,
    mut app_state: ResMut<State<AppState>>,
    mut elements_q: Query<&mut Visibility, With<GameoverElement>>,
    mut text_q: Query<(&mut Text, &Text2dSize), With<ScoreText>>,
    mut icon_q: Query<&mut Transform, With<ScoreTextIcon>>,
) {
    let (mut text, text_size) = text_q.single_mut();
    text.sections[0].value = format!("You   managed   to   kill   [ {} ]", score.0);
    let icon_offset = text_size.size.width / 2.;
    icon_q.single_mut().translation.x = icon_offset - 50.;

    for mut visibility in elements_q.iter_mut() {
        visibility.is_visible = true;
    }

    match &*input_kind {
        InputKind::Keyboard => {
            if keys.just_released(KeyCode::Key1) {
                app_state.set(AppState::InGame).unwrap();
            }
        }
        InputKind::Gamepad => {
            let gamepad = Gamepad(0);
            if !gamepads.contains(&gamepad) {
                return;
            }
            if buttons.just_released(GamepadButton(gamepad, GamepadButtonType::West)) {
                app_state.set(AppState::InGame).unwrap();
            }
        }
    }
}
