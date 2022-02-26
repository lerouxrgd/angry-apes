use bevy::prelude::*;
use bevy::render::camera::ScalingMode;

fn main() {
    App::new()
        .insert_resource(WindowDescriptor {
            title: "Angry Apes".to_string(),
            ..Default::default()
        })
        .insert_resource(Msaa { samples: 1 })
        .add_plugins(DefaultPlugins)
        .add_startup_system(setup)
        .add_system(switch)
        // .add_system(player_input)
        // .add_system(sprite_movement)
        .run();
}

fn setup(
    windows: Res<Windows>,
    mut commands: Commands,
    asset_server: Res<AssetServer>,
    mut texture_atlases: ResMut<Assets<TextureAtlas>>,
) {
    let window = windows.get_primary().unwrap();
    // window.height()

    commands.spawn_bundle(SpriteBundle {
        texture: asset_server.load("background.png"),
        transform: Transform::from_xyz(0., 0., 1.),
        ..Default::default()
    });

    ////////////////////

    let texture_handle = asset_server.load("bored_ape_king.png");
    let texture_atlas = TextureAtlas::from_grid(texture_handle, Vec2::new(600.0, 600.0), 1, 1);
    let texture_atlas_h = texture_atlases.add(texture_atlas);

    commands.spawn_bundle(SpriteSheetBundle {
        texture_atlas: texture_atlas_h,
        transform: Transform::from_xyz(0., 0., 5.),
        ..Default::default()
    });

    let texture_handle = asset_server.load("bored_ape_king_lasers.png");
    let texture_atlas = TextureAtlas::from_grid(texture_handle, Vec2::new(900.0, 600.0), 1, 1);
    let texture_atlas_h = texture_atlases.add(texture_atlas);

    commands
        .spawn_bundle(SpriteSheetBundle {
            texture_atlas: texture_atlas_h,
            transform: Transform::from_xyz(150., 0., 10.),
            ..Default::default()
        })
        .insert(AttackAnimation::Off(Timer::from_seconds(3., false)));

    let mut camera = OrthographicCameraBundle::new_2d();
    let projection = bevy::render::camera::OrthographicProjection {
        scaling_mode: ScalingMode::FixedVertical,
        scale: 300.,
        ..Default::default()
    };
    camera.orthographic_projection = projection;
    commands.spawn_bundle(camera);
}

fn switch(time: Res<Time>, mut attack_q: Query<(&mut AttackAnimation, &mut Visibility)>) {
    for (mut anim, mut visibility) in attack_q.iter_mut() {
        match &mut *anim {
            AttackAnimation::On(timer) => {
                timer.tick(time.delta());
                if timer.just_finished() {
                    *anim = AttackAnimation::Off(Timer::from_seconds(3., false));
                    visibility.is_visible = false;
                }
            }
            AttackAnimation::Off(timer) => {
                timer.tick(time.delta());
                if timer.just_finished() {
                    *anim = AttackAnimation::On(Timer::from_seconds(1., false));
                    visibility.is_visible = true;
                }
            }
        }
    }
}

#[derive(Component)]
pub enum AttackAnimation {
    On(Timer),
    Off(Timer),
}
