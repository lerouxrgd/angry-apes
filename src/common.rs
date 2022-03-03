use crate::prelude::*;

/////////////////////////////////////// Spawners ///////////////////////////////////////

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

pub fn spawn_font(asset_server: &AssetServer) -> Handle<Font> {
    let font_handle: Handle<Font> = asset_server.load("FontsFree-Net-Monkey.ttf");
    font_handle
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

// TODO: make this random somehow
#[derive(Component)]
pub struct TriggerTimer(pub Timer);

impl Default for TriggerTimer {
    fn default() -> Self {
        Self(Timer::from_seconds(3., true))
    }
}

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

pub fn respawn_game_state() {
    // TODO: respawn what has been despawned
}

pub fn gameover_screen() {
    // TODO: display some text and change AppState to exit gameover screen
}
