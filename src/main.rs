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
    pub use bevy::input::gamepad::{Gamepad, GamepadAxisType, GamepadButton};
    pub use bevy::input::keyboard::KeyboardInput;
    pub use bevy::prelude::*;
    pub use bevy::render::camera::OrthographicProjection;
    pub use bevy::render::camera::ScalingMode;
    pub use bevy::text::Text2dSize;
    pub use bevy_embedded_assets::EmbeddedAssetPlugin;
    pub use bevy_mod_aseprite::{
        Aseprite, AsepriteAnimation, AsepriteBundle, AsepritePlugin, AsepriteSystems, AsepriteTag,
    };
    pub use bevy_prototype_lyon::prelude::{
        DrawMode, FillMode, Geometry, GeometryBuilder, Path as TessPath, ShapePlugin, StrokeMode,
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

use crate::prelude::*;

fn main() {
    App::new()
        // Setup plugins
        .add_plugins(
            DefaultPlugins
                .set(WindowPlugin {
                    window: WindowDescriptor {
                        title: "Angry Apes".to_string(),
                        width: GLOBAL_WIDTH,
                        height: GLOBAL_HEIGHT,
                        ..default()
                    },
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
        .add_state(AppState::Loading)
        // Game related systems
        .add_system_set(SystemSet::on_enter(AppState::Loading).with_system(load_assets))
        .add_system_set(SystemSet::on_update(AppState::Loading).with_system(check_assets))
        .add_system_set(SystemSet::on_exit(AppState::Loading).with_system(setup))
        .add_system_set(
            SystemSet::on_update(AppState::InGame)
                // Input related systems
                .with_system(gamepad_connection_events.before(handle_input))
                .with_system(handle_input)
                // Player related systems
                .with_system(move_units)
                .with_system(fall_units)
                .with_system(tick_dashes)
                .with_system(cooldown_dashes)
                .with_system(transition_units.before(AsepriteSystems::Animate))
                .with_system(unit_attacks_ape.after(transition_units))
                .with_system(reorient_units_on_sprite_change)
                .with_system(update_units.at_end())
                // Eth related systems
                .with_system(make_eth)
                .with_system(animate_eth)
                .with_system(player_collects_eth)
                .with_system(player_eth_gauge)
                .with_system(decay_player_eth)
                // Ape related systems
                .with_system(make_ape)
                .with_system(move_apes)
                .with_system(trigger_ape_attack)
                .with_system(ape_attacks_player_collision)
                .with_system(animate_apes_wounds)
                .with_system(animate_apes_attacks)
                .with_system(display_dead_apes_hud),
        )
        // Gameover related systems
        .add_system_set(SystemSet::on_enter(AppState::GameOver).with_system(despawn_game_state))
        .add_system_set(SystemSet::on_update(AppState::GameOver).with_system(gameover_screen))
        .add_system_set(SystemSet::on_exit(AppState::GameOver).with_system(respawn_game_state))
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
    mut state: ResMut<State<AppState>>,
) {
    if let LoadState::Loaded =
        asset_server.get_group_load_state(aseprite_handles.iter().map(|(_, handle)| handle.id()))
    {
        state.set(AppState::InGame).unwrap();
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
