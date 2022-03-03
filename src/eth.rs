use crate::prelude::*;

/////////////////////////////////////// Spawners ///////////////////////////////////////

pub fn init_eth(
    commands: &mut Commands,
    asset_server: &AssetServer,
    texture_atlases: &mut Assets<TextureAtlas>,
) {
    let eth_image = asset_server.load("eth.png");
    let eth_atlas = TextureAtlas::from_grid(eth_image, Vec2::new(50.0, 50.0), 1, 11);
    let eth_atlas_h = texture_atlases.add(eth_atlas);

    commands.insert_resource(EthHandle(eth_atlas_h));

    commands.insert_resource(EthPicked(Instant::now()))
}

pub fn spawn_eth(commands: &mut Commands, position: Vec3, eth_handle: &EthHandle) {
    commands
        .spawn_bundle(SpriteSheetBundle {
            texture_atlas: eth_handle.0.clone(),
            transform: Transform {
                translation: position,
                scale: Vec3::splat(1.2),
                ..Default::default()
            },
            ..Default::default()
        })
        .insert(Animation {
            timer: Timer::from_seconds(0.12, true),
            count: None,
        })
        .insert(Eth::default());
}

pub fn spawn_eth_hud(commands: &mut Commands, asset_server: &AssetServer) {
    let eth_hud = commands.spawn().insert(EthHud).id();

    let outer_rect = shapes::Rectangle {
        extents: Vec2::new(250., 16.),
        origin: shapes::RectangleOrigin::TopLeft,
    };
    let builder = GeometryBuilder::new().add(&outer_rect);
    let outer = commands
        .entity(eth_hud)
        .insert_bundle(builder.build(
            DrawMode::Outlined {
                fill_mode: FillMode::color(Color::NONE),
                outline_mode: StrokeMode::new(Color::rgb_u8(168, 231, 242), 3.),
            },
            Transform::from_xyz(-540., 280., 999.),
        ))
        .id();

    let icon = commands
        .spawn_bundle(SpriteBundle {
            texture: asset_server.load("eth_icon.png"),
            transform: Transform {
                scale: Vec3::splat(0.5),
                translation: Vec3::new(-18., -7., 0.),
                ..Default::default()
            },
            ..Default::default()
        })
        .id();
    commands.entity(outer).push_children(&[icon]);

    let inner_rect = shapes::Rectangle {
        extents: Vec2::new(250., 16. - 3.),
        origin: shapes::RectangleOrigin::TopLeft,
    };
    let builder = GeometryBuilder::new().add(&inner_rect);
    let inner = commands
        .spawn_bundle(builder.build(
            DrawMode::Fill(FillMode::color(Color::rgb_u8(132, 132, 132))),
            Transform::from_xyz(3. / 2., -3. / 2., 0.),
        ))
        .insert(EthGauge)
        .id();
    commands.entity(outer).push_children(&[inner]);
}

////////////////////////////////////// Components //////////////////////////////////////

pub struct EthHandle(pub Handle<TextureAtlas>);

#[derive(Component)]
pub struct Eth {
    pub quantity: f32,
}

impl Default for Eth {
    fn default() -> Self {
        Self { quantity: 10. }
    }
}

#[derive(Debug, Component)]
pub struct EthOwned {
    pub current: f32,
    pub max: f32,
}

impl Default for EthOwned {
    fn default() -> Self {
        Self {
            current: 0.,
            max: 30.,
        }
    }
}

impl EthOwned {
    pub fn add(&mut self, eth: &Eth) {
        self.current = self.max.min(self.current + eth.quantity);
    }

    pub fn remove(&mut self, decayed: f32) {
        self.current = (self.current - decayed).max(0.);
    }

    pub fn is_full(&self) -> bool {
        self.current == self.max
    }

    pub fn is_empty(&self) -> bool {
        self.current == 0.
    }
}

#[derive(Component)]
pub struct EthHud;

#[derive(Component)]
pub struct EthGauge;

pub struct EthPicked(pub Instant);

/////////////////////////////////////// Systems ////////////////////////////////////////

pub fn make_eth(
    eth_handle: Res<EthHandle>,
    picked_eth_at: Res<EthPicked>,
    mut commands: Commands,
    eth_q: Query<Entity, With<Eth>>,
) {
    let eth_count = eth_q.iter().count();

    if eth_count == 0 && picked_eth_at.0.elapsed() > Duration::from_secs(3) {
        spawn_eth(&mut commands, Vec3::new(-300., -222., 20.), &eth_handle);
    }
}

pub fn player_collects_eth(
    mut picked_eth_at: ResMut<EthPicked>,
    mut commands: Commands,
    mut ev_unit_changed: EventWriter<UnitChanged>,
    mut player_q: Query<
        (
            Entity,
            &Transform,
            &mut EthOwned,
            &UnitSprite,
            &UnitState,
            &mut UnitCondition,
            &UnitAnimations,
            &Orientation,
        ),
        With<Player>,
    >,
    eth_q: Query<(Entity, &Eth, &Transform)>,
) {
    let (
        player,
        player_transform,
        mut player_eth,
        player_sprite,
        &player_state,
        mut player_condition,
        player_anims,
        &orientation,
    ) = player_q.single_mut();
    let player_x = player_transform.translation.x;

    for (eth_id, eth, eth_transform) in eth_q.iter() {
        let eth_x = eth_transform.translation.x;
        if (player_x - eth_x).abs() < 30. {
            player_eth.add(eth);
            commands.entity(eth_id).despawn();
            picked_eth_at.0 = Instant::now();

            if player_eth.is_full() {
                let new_condition = UnitCondition::Upgraded;
                *player_condition = new_condition;
                ev_unit_changed.send(UnitChanged {
                    unit: player,
                    unit_sprite: player_sprite.0,
                    unit_anims: player_anims.clone(),
                    new_state: player_state,
                    new_condition,
                    orientation,
                });
            }
        }
    }
}

pub fn decay_player_eth(
    time: Res<Time>,
    mut ev_unit_changed: EventWriter<UnitChanged>,
    mut player_q: Query<
        (
            Entity,
            &mut EthOwned,
            &UnitSprite,
            &UnitState,
            &mut UnitCondition,
            &UnitAnimations,
            &Orientation,
        ),
        With<Player>,
    >,
) {
    let (
        player,
        mut player_eth,
        player_sprite,
        &player_state,
        mut player_condition,
        player_anims,
        &orientation,
    ) = player_q.single_mut();

    if let UnitCondition::Upgraded = &*player_condition {
        player_eth.remove(2. * time.delta_seconds());
        if player_eth.is_empty() {
            let new_condition = UnitCondition::Normal;
            *player_condition = new_condition;
            ev_unit_changed.send(UnitChanged {
                unit: player,
                unit_sprite: player_sprite.0,
                unit_anims: player_anims.clone(),
                new_state: player_state,
                new_condition,
                orientation,
            });
        }
    }
}

pub fn player_eth_gauge(
    player_q: Query<(&EthOwned, &UnitCondition), With<Player>>,
    mut gauge_q: Query<(&mut TessPath, &mut DrawMode), With<EthGauge>>,
) {
    let (player_eth, player_condition) = player_q.single();

    let rect_x = player_eth.current / player_eth.max * (250. - 3.);
    let mut path_builder = tess::path::Path::builder();
    let rect = shapes::Rectangle {
        extents: Vec2::new(rect_x, 16. - 3.),
        origin: shapes::RectangleOrigin::TopLeft,
    };
    rect.add_geometry(&mut path_builder);

    let (mut gauge_path, mut draw) = gauge_q.single_mut();

    *gauge_path = TessPath(path_builder.build());
    *draw = match player_condition {
        UnitCondition::Normal => DrawMode::Fill(FillMode::color(Color::rgb_u8(132, 132, 132))),
        UnitCondition::Upgraded => DrawMode::Fill(FillMode::color(Color::rgb_u8(200, 160, 24))),
    }
}

pub fn animate_eth(
    time: Res<Time>,
    texture_atlases: Res<Assets<TextureAtlas>>,
    mut query: Query<
        (
            &mut Animation,
            &mut TextureAtlasSprite,
            &Handle<TextureAtlas>,
        ),
        With<Eth>,
    >,
) {
    for (mut anim, mut sprite, texture_atlas_h) in query.iter_mut() {
        anim.timer.tick(time.delta());
        if anim.timer.just_finished() {
            let texture_atlas = texture_atlases.get(texture_atlas_h).unwrap();
            sprite.index = (sprite.index + 1) % texture_atlas.textures.len();
        }
    }
}
