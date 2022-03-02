use crate::prelude::*;

/////////////////////////////////////// Spawners ///////////////////////////////////////

pub fn spawn_background(commands: &mut Commands, asset_server: &AssetServer) {
    commands.spawn_bundle(SpriteBundle {
        texture: asset_server.load("background.png"),
        transform: Transform::from_xyz(0., 0., 0.),
        ..Default::default()
    });
}

pub fn spawn_platform(commands: &mut Commands, asset_server: &AssetServer) {
    commands.spawn_bundle(SpriteBundle {
        texture: asset_server.load("platform.png"),
        transform: Transform::from_xyz(0., -270., 1.),
        ..Default::default()
    });
}

pub fn spawn_camera(commands: &mut Commands) {
    let projection = OrthographicProjection {
        scaling_mode: ScalingMode::FixedVertical,
        scale: 300.,
        ..Default::default()
    };

    let mut camera = OrthographicCameraBundle::new_2d();
    camera.orthographic_projection = projection;

    commands.spawn_bundle(camera);
}

////////////////////////////////////// Components //////////////////////////////////////

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
