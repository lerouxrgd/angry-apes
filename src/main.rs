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

    pub use bevy::app::Events;
    pub use bevy::input::gamepad::{Gamepad, GamepadAxisType, GamepadButton};
    pub use bevy::input::keyboard::KeyboardInput;
    pub use bevy::input::ElementState;
    pub use bevy::prelude::*;
    pub use bevy::render::camera::OrthographicProjection;
    pub use bevy::render::camera::ScalingMode;
    pub use bevy::text::Text2dSize;
    pub use bevy_embedded_assets::EmbeddedAssetPlugin;
    pub use bevy_prototype_lyon::prelude::{
        DrawMode, FillMode, Geometry, GeometryBuilder, Path as TessPath, ShapePlugin, StrokeMode,
    };
    pub use bevy_prototype_lyon::shapes;
    pub use lyon_tessellation as tess;
    pub use rand::seq::SliceRandom;
    pub use rand_distr::{Beta, Distribution};

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
        // Core resources
        .insert_resource(WindowDescriptor {
            title: "Angry Apes".to_string(),
            width: GLOBAL_WIDTH,
            height: GLOBAL_HEIGHT,
            ..Default::default()
        })
        .insert_resource(Msaa { samples: 1 })
        // Setup plugins
        .add_plugins_with(DefaultPlugins, |group| {
            group.add_before::<bevy::asset::AssetPlugin, _>(EmbeddedAssetPlugin)
        })
        .add_plugin(ShapePlugin)
        // Initialize game
        .add_startup_system(setup)
        .add_state(AppState::InGame)
        // Process inputs at the very beginning
        .add_stage_after(
            CoreStage::PreUpdate,
            "inputs",
            SystemStage::single_threaded(),
        )
        .add_system_set_to_stage("inputs", State::<AppState>::get_driver())
        .add_system_set_to_stage(
            "inputs",
            SystemSet::on_update(AppState::InGame)
                .with_system(gamepad_connection_events.before("input"))
                .with_system(gamepad_input.label("input"))
                .with_system(keyboard_input.label("input"))
                .with_system(update_units.after("input")),
        )
        .add_system_set(
            SystemSet::on_update(AppState::InGame)
                // Player related systems
                .with_system(move_units)
                .with_system(fall_units)
                .with_system(cooldown_units)
                .with_system(animate_unit_sprites)
                .with_system(unit_attacks_ape)
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
                .with_system(animate_apes_attacks)
                .with_system(animate_apes_wounds)
                .with_system(display_dead_apes_hud),
        )
        // Gameover related systems
        .add_system_set(SystemSet::on_enter(AppState::GameOver).with_system(despawn_game_state))
        .add_system_set(SystemSet::on_update(AppState::GameOver).with_system(gameover_screen))
        .add_system_set(SystemSet::on_exit(AppState::GameOver).with_system(respawn_game_state))
        // Process unit changes at the end
        .add_stage_before(
            CoreStage::PostUpdate,
            "update_units",
            SystemStage::single_threaded(),
        )
        .add_system_set_to_stage("update_units", State::<AppState>::get_driver())
        .add_system_set_to_stage(
            "update_units",
            SystemSet::on_update(AppState::InGame).with_system(update_units),
        )
        // Game logic resources
        .init_resource::<Events<UnitChanged>>()
        .init_resource::<Events<UnitAttack>>()
        .init_resource::<InputKind>()
        .init_resource::<Score>()
        .run();
}

fn setup(
    mut commands: Commands,
    asset_server: Res<AssetServer>,
    mut texture_atlases: ResMut<Assets<TextureAtlas>>,
) {
    let font_handle = spawn_font(&mut commands, &asset_server);
    let ape_icon_h = init_ape_icon(&mut commands, &asset_server);
    init_eth(&mut commands, &asset_server, &mut texture_atlases);
    spawn_camera(&mut commands);
    spawn_gameover_screen(&mut commands, &asset_server, &font_handle, &ape_icon_h);

    spawn_game_state(
        &mut commands,
        &asset_server,
        &mut texture_atlases,
        &font_handle,
    );
}
