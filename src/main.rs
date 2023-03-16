#![allow(clippy::type_complexity)]
#![allow(clippy::too_many_arguments)]
#![allow(clippy::single_component_path_imports)]

mod ape;
mod common;
mod eth;
mod inputs;
mod player;

mod prelude {
    pub use std::collections::{HashMap, HashSet};

    #[cfg(target_arch = "wasm32")]
    pub use instant::{Duration, Instant};
    #[cfg(not(target_arch = "wasm32"))]
    pub use std::time::{Duration, Instant};

    #[cfg(not(target_arch = "wasm32"))]
    #[cfg(debug_assertions)]
    #[allow(unused_imports)]
    use bevy_dylib;

    pub use bevy::asset::LoadState;
    pub use bevy::ecs::event::Events;
    pub use bevy::input::gamepad::{
        Gamepad, GamepadAxisType, GamepadButton, GamepadConnection, GamepadConnectionEvent,
        GamepadEvent,
    };
    pub use bevy::input::keyboard::KeyboardInput;
    pub use bevy::prelude::*;
    pub use bevy::render::camera::OrthographicProjection;
    pub use bevy::render::camera::ScalingMode;
    pub use bevy::text::TextLayoutInfo;
    pub use bevy_embedded_assets::EmbeddedAssetPlugin;
    pub use bevy_mod_aseprite::{
        Aseprite, AsepriteAnimation, AsepriteBundle, AsepritePlugin, AsepriteSystems, AsepriteTag,
    };
    pub use bevy_prototype_lyon::prelude::{
        Fill, Geometry, GeometryBuilder, Path as TessPath, ShapeBundle, ShapePlugin, Stroke,
    };
    pub use bevy_prototype_lyon::shapes;
    pub use lyon_tessellation as tess;
    pub use rand::seq::SliceRandom;
    pub use rand_distr::{Beta, Distribution};

    pub mod sprites {
        use bevy_mod_aseprite::aseprite;
        aseprite!(pub Paladin, "player_paladin.ase");
        aseprite!(pub Crusader, "player_crusader.ase");
    }

    pub use crate::ape::*;
    pub use crate::common::*;
    pub use crate::eth::*;
    pub use crate::inputs::*;
    pub use crate::player::*;

    pub const GLOBAL_WIDTH: f32 = 1200.; // matches background.png width
    pub const GLOBAL_HEIGHT: f32 = 600.; // matches background.png height
    pub const PROJECTION_SCALE: f32 = 300.;
}

use bevy::window::WindowResolution;

use crate::prelude::*;

fn main() {
    App::new()
        // Setup plugins
        .add_plugins(
            DefaultPlugins
                .set(WindowPlugin {
                    primary_window: Some(Window {
                        title: "Angry Apes".to_string(),
                        resolution: WindowResolution::new(GLOBAL_WIDTH, GLOBAL_HEIGHT),
                        ..default()
                    }),
                    ..default()
                })
                .set(ImagePlugin::default_nearest())
                .build()
                .add_before::<AssetPlugin, _>(EmbeddedAssetPlugin),
        )
        .add_plugin(ShapePlugin)
        .add_plugin(AsepritePlugin)
        // Initialize game
        .init_resource::<AsepriteHandles>()
        .init_resource::<Events<UnitChanged>>()
        .init_resource::<Events<UnitAttack>>()
        .init_resource::<InputKind>()
        .init_resource::<Score>()
        .add_state::<AppState>()
        // Game related systems
        .add_system(load_assets.in_schedule(OnEnter(AppState::Loading)))
        .add_system(check_assets.in_set(OnUpdate(AppState::Loading)))
        .add_system(setup.in_schedule(OnExit(AppState::Loading)))
        .add_systems(
            (
                // Input related systems
                gamepad_connection_events.before(handle_input),
                handle_input,
            )
                .in_set(OnUpdate(AppState::InGame)),
        )
        .add_systems(
            (
                // Player related systems
                move_units,
                fall_units,
                tick_dashes,
                cooldown_dashes,
                transition_units.before(AsepriteSystems::Animate),
                unit_attacks_ape.after(transition_units),
                reorient_units_on_sprite_change,
                update_units,
            )
                .in_set(OnUpdate(AppState::InGame)),
        )
        .add_systems(
            (
                // Eth related systems
                make_eth,
                animate_eth,
                player_collects_eth,
                player_eth_gauge,
                decay_player_eth,
            )
                .in_set(OnUpdate(AppState::InGame)),
        )
        .add_systems(
            (
                // Ape related systems
                make_ape,
                move_apes,
                trigger_ape_attack,
                ape_attacks_player_collision,
                animate_apes_wounds,
                animate_apes_attacks,
                display_dead_apes_hud,
            )
                .in_set(OnUpdate(AppState::InGame)),
        )
        // Gameover related systems
        .add_system(despawn_game_state.in_schedule(OnEnter(AppState::GameOver)))
        .add_system(gameover_screen.in_set(OnUpdate(AppState::GameOver)))
        .add_system(respawn_game_state.in_schedule(OnExit(AppState::GameOver)))
        .run();
}

fn load_assets(mut aseprite_handles: ResMut<AsepriteHandles>, asset_server: Res<AssetServer>) {
    for asprite_path in [sprites::Paladin::PATH, sprites::Crusader::PATH] {
        let aseprite = asset_server.load(asprite_path);
        aseprite_handles.insert(asprite_path, aseprite);
    }
}

fn check_assets(
    aseprite_handles: ResMut<AsepriteHandles>,
    asset_server: Res<AssetServer>,
    mut state: ResMut<NextState<AppState>>,
) {
    if let LoadState::Loaded =
        asset_server.get_group_load_state(aseprite_handles.iter().map(|(_, handle)| handle.id()))
    {
        state.set(AppState::InGame);
    }
}

fn setup(
    mut commands: Commands,
    asset_server: Res<AssetServer>,
    mut texture_atlases: ResMut<Assets<TextureAtlas>>,
    aseprite_handles: Res<AsepriteHandles>,
    aseprites: Res<Assets<Aseprite>>,
) {
    init_eth(&mut commands, &asset_server, &mut texture_atlases);

    let font_handle = spawn_font(&mut commands, &asset_server);
    let ape_icon_h = init_ape_icon(&mut commands, &asset_server);
    spawn_gameover_screen(&mut commands, &asset_server, &font_handle, &ape_icon_h);

    spawn_camera(&mut commands);

    spawn_game_state(
        &mut commands,
        &asset_server,
        &mut texture_atlases,
        &font_handle,
        &aseprite_handles,
        &aseprites,
    );
}
