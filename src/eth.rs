use crate::prelude::*;

/////////////////////////////////////// Spawners ///////////////////////////////////////

pub fn init_eth(
    commands: &mut Commands,
    asset_server: &AssetServer,
    texture_atlases: &mut Assets<TextureAtlas>,
) {
    let eth_image = asset_server.load("eth.png");
    let eth_atlas = TextureAtlas::from_grid(eth_image, Vec2::new(50.0, 50.0), 1, 11, None, None);
    let eth_atlas_h = texture_atlases.add(eth_atlas);

    commands.insert_resource(EthHandle(eth_atlas_h));

    commands.insert_resource(EthPicked(Instant::now()))
}

pub fn spawn_eth(commands: &mut Commands, position: Vec3, eth_handle: &EthHandle) {
    commands
        .spawn(SpriteSheetBundle {
            texture_atlas: eth_handle.0.clone(),
            transform: Transform {
                translation: position,
                scale: Vec3::splat(1.2),
                ..default()
            },
            ..default()
        })
        .insert(Animation {
            timer: Timer::from_seconds(0.12, TimerMode::Repeating),
            count: None,
        })
        .insert(Eth::default());
}

pub fn spawn_eth_hud(commands: &mut Commands, asset_server: &AssetServer) {
    let eth_hud = commands.spawn(EthHud).id();

    let outer_rect = shapes::Rectangle {
        extents: Vec2::new(250., 16.),
        origin: shapes::RectangleOrigin::TopLeft,
    };
    let builder = GeometryBuilder::new().add(&outer_rect);
    let outer = commands
        .entity(eth_hud)
        .insert((
            ShapeBundle {
                path: builder.build(),
                transform: Transform::from_xyz(-540., 280., 999.),
                ..default()
            },
            Fill::color(Color::NONE),
            Stroke::new(Color::rgb_u8(168, 231, 242), 3.),
        ))
        .id();

    let icon = commands
        .spawn(SpriteBundle {
            texture: asset_server.load("eth_icon.png"),
            transform: Transform {
                scale: Vec3::splat(0.5),
                translation: Vec3::new(-18., -7., 0.),
                ..default()
            },
            ..default()
        })
        .id();
    commands.entity(outer).push_children(&[icon]);

    let inner_rect = shapes::Rectangle {
        extents: Vec2::new(250., 16. - 3.),
        origin: shapes::RectangleOrigin::TopLeft,
    };
    let builder = GeometryBuilder::new().add(&inner_rect);
    let inner = commands
        .spawn((
            ShapeBundle {
                path: builder.build(),
                transform: Transform::from_xyz(3. / 2., -3. / 2., 0.),
                ..default()
            },
            Fill::color(Color::rgb_u8(132, 132, 132)),
        ))
        .insert(EthGauge)
        .id();
    commands.entity(outer).push_children(&[inner]);
}

////////////////////////////////////// Components //////////////////////////////////////

#[derive(Resource)]
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

#[derive(Resource, Deref)]
pub struct EthPicked(pub Instant);

/////////////////////////////////////// Systems ////////////////////////////////////////

pub fn make_eth(
    eth_handle: Res<EthHandle>,
    picked_eth_at: Res<EthPicked>,
    mut commands: Commands,
    player_q: Query<&Transform, With<Player>>,
    eth_q: Query<Entity, With<Eth>>,
) {
    let eth_count = eth_q.iter().count();

    if eth_count == 0 && picked_eth_at.elapsed() > Duration::from_secs(3) {
        let player_x = player_q.single().translation.x;

        let (a, b) = if player_x < -0.3 * (GLOBAL_WIDTH / 2.) {
            (5.0, 1.0)
        } else if player_x > 0.4 * (GLOBAL_WIDTH / 2.) {
            (1.0, 3.0)
        } else {
            (0.5, 0.5)
        };

        let beta = Beta::new(a, b).unwrap();
        let v = beta.sample(&mut rand::thread_rng());
        let x = v * (GLOBAL_WIDTH - 100.) - (GLOBAL_WIDTH - 100.) / 2.;

        spawn_eth(&mut commands, Vec3::new(x, -222., 20.), &eth_handle);
    }
}

pub fn player_collects_eth(
    mut picked_eth_at: ResMut<EthPicked>,
    mut commands: Commands,
    mut ev_unit_changed: EventWriter<UnitChanged>,
    mut player_q: Query<(Entity, &Transform, &mut EthOwned), With<Player>>,
    eth_q: Query<(Entity, &Eth, &Transform)>,
) {
    let (player, player_transform, mut player_eth) = player_q.single_mut();
    let player_x = player_transform.translation.x;
    let player_y = player_transform.translation.y;

    for (eth_id, eth, eth_transform) in eth_q.iter() {
        let eth_x = eth_transform.translation.x;
        if (player_x - eth_x).abs() < 30. && player_y < -100. {
            player_eth.add(eth);
            commands.entity(eth_id).despawn();
            picked_eth_at.0 = Instant::now();

            if player_eth.is_full() {
                ev_unit_changed
                    .send(UnitChanged::entity(player).new_condition(UnitCondition::Upgraded));
            }
        }
    }
}

pub fn decay_player_eth(
    time: Res<Time>,
    mut ev_unit_changed: EventWriter<UnitChanged>,
    mut player_q: Query<(Entity, &mut EthOwned, &UnitCondition), With<Player>>,
) {
    let (player, mut player_eth, player_condition) = player_q.single_mut();

    if let UnitCondition::Upgraded = player_condition {
        player_eth.remove(2. * time.delta_seconds());
        if player_eth.is_empty() {
            ev_unit_changed.send(UnitChanged::entity(player).new_condition(UnitCondition::Normal));
        }
    }
}

pub fn player_eth_gauge(
    player_q: Query<(&EthOwned, &UnitCondition), With<Player>>,
    mut gauge_q: Query<(&mut TessPath, &mut Fill), With<EthGauge>>,
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
        UnitCondition::Normal => Fill::color(Color::rgb_u8(132, 132, 132)),
        UnitCondition::Upgraded => Fill::color(Color::rgb_u8(200, 160, 24)),
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
