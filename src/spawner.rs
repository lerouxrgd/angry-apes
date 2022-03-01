use crate::prelude::*;

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

pub fn spawn_player(
    commands: &mut Commands,
    asset_server: &AssetServer,
    texture_atlases: &mut Assets<TextureAtlas>,
) {
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

    let unit_anims = UnitAnimations {
        stand_h,
        stand_timer,
        move_h,
        move_timer,
        attack_h,
        attack_timer,
        attack_count,
        wound_h,
        wound_timer,
        wound_count,
    };
    let unit_state = UnitState::Stand;
    let orientation = Orientation::Right;
    let unit_sprite = spawn_unit_sprite(commands, &unit_anims, &unit_state, &orientation);

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
        .insert(UnitSprite(unit_sprite))
        .push_children(&[unit_sprite])
        .insert(orientation);
}

pub fn spawn_unit_sprite(
    commands: &mut Commands,
    anims: &UnitAnimations,
    state: &UnitState,
    orientation: &Orientation,
) -> Entity {
    commands
        .spawn_bundle(SpriteSheetBundle {
            texture_atlas: anims.atlas_for(state),
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

pub fn spawn_coins(
    commands: &mut Commands,
    asset_server: &AssetServer,
    texture_atlases: &mut Assets<TextureAtlas>,
) {
    let eth_handle = asset_server.load("eth.png");
    let eth_atlas = TextureAtlas::from_grid(eth_handle, Vec2::new(50.0, 50.0), 1, 11);
    let eth_atlas_h = texture_atlases.add(eth_atlas);

    commands
        .spawn_bundle(SpriteSheetBundle {
            texture_atlas: eth_atlas_h,
            transform: Transform {
                translation: Vec3::new(-300., -222., 10.),
                scale: Vec3::splat(1.2),
                ..Default::default()
            },
            ..Default::default()
        })
        .insert(Animation {
            timer: Timer::from_seconds(0.12, true),
            count: None,
        })
        .insert(Coin);
}

pub fn spawn_ape(
    commands: &mut Commands,
    asset_server: &AssetServer,
    texture_atlases: &mut Assets<TextureAtlas>,
) {
    let ape = commands
        .spawn()
        .insert(Ape)
        .insert_bundle(SpriteBundle {
            texture: asset_server.load("bored_ape_king.png"),
            transform: Transform {
                scale: Vec3::splat(0.8),
                translation: Vec3::new(0., 0., 5.),
                ..Default::default()
            },
            ..Default::default()
        })
        .id();

    let laser_init_image = asset_server.load("ape_eyes.png");
    let laser_init_atlas = TextureAtlas::from_grid(laser_init_image, Vec2::new(900.0, 600.0), 2, 1);
    let laser_on_image = asset_server.load("ape_lasers.png");
    let laser_on_atlas = TextureAtlas::from_grid(laser_on_image, Vec2::new(900.0, 600.0), 3, 1);
    let ape_attack_spec = ApeAttackSpec {
        ape,
        init_h: texture_atlases.add(laser_init_atlas),
        init_duration: DurationTimer::from_seconds(0.6),
        init_timer: Timer::from_seconds(0.1, true),
        on_h: texture_atlases.add(laser_on_atlas),
        on_duration: DurationTimer::from_seconds(1.0),
        on_timer: Timer::from_seconds(0.1, true),
    };

    commands.entity(ape).insert(ape_attack_spec);
}

pub fn spawn_attack_init(commands: &mut Commands, attack_spec: &ApeAttackSpec) {
    let ApeAttackSpec {
        ape,
        init_duration,
        init_timer,
        ..
    } = attack_spec;

    let animation = commands
        .spawn_bundle(SpriteSheetBundle {
            texture_atlas: attack_spec.init_h.clone(),
            transform: Transform::from_xyz(150., 0., 10.),
            ..Default::default()
        })
        .insert(ApeEntity(*ape))
        .insert(StagedAnimation::init(
            init_duration.clone(),
            init_timer.clone(),
        ))
        .id();

    commands.entity(*ape).push_children(&[animation]);
}

pub fn spawn_attack_on(commands: &mut Commands, attack_spec: &ApeAttackSpec) {
    let ApeAttackSpec {
        ape,
        on_duration,
        on_timer,
        ..
    } = attack_spec;

    let animation = commands
        .spawn_bundle(SpriteSheetBundle {
            texture_atlas: attack_spec.on_h.clone(),
            transform: Transform::from_xyz(150., 0., 10.),
            ..Default::default()
        })
        .insert(ApeEntity(*ape))
        .insert(StagedAnimation::on(on_duration.clone(), on_timer.clone()))
        .id();

    commands.entity(*ape).push_children(&[animation]);
}
