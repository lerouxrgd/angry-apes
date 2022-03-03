mod ape;
mod common;
mod eth;
mod inputs;
mod player;

mod prelude {
    pub use std::collections::HashSet;
    pub use std::time::{Duration, Instant};

    pub use bevy::app::Events;
    pub use bevy::input::gamepad::{Gamepad, GamepadAxisType, GamepadButton};
    pub use bevy::input::keyboard::KeyboardInput;
    pub use bevy::input::ElementState;
    pub use bevy::prelude::*;
    pub use bevy::render::camera::OrthographicProjection;
    pub use bevy::render::camera::ScalingMode;
    pub use bevy_embedded_assets::EmbeddedAssetPlugin;
    pub use bevy_prototype_lyon::prelude::{
        DrawMode, FillMode, Geometry, GeometryBuilder, Path as TessPath, ShapePlugin, StrokeMode,
    };
    pub use bevy_prototype_lyon::shapes;
    pub use lyon_tessellation as tess;

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
        .add_startup_system(setup)
        // Inputs related
        .add_stage_after(CoreStage::PreUpdate, "inputs", SystemStage::parallel())
        .add_system_set_to_stage(
            "inputs",
            SystemSet::new()
                .with_system(gamepad_connection_events.before("input"))
                .with_system(gamepad_input.label("input"))
                .with_system(keyboard_input.label("input"))
                .with_system(update_units.after("input")),
        )
        // Player related
        .add_system(move_units)
        .add_system(fall_units)
        .add_system(animate_unit_sprites)
        .add_system(unit_attacks_ape)
        // Eth related
        .add_system(make_eth)
        .add_system(animate_eth)
        .add_system(player_collects_eth)
        .add_system(player_eth_gauge)
        .add_system(decay_player_eth)
        // Ape related
        .add_system(make_ape)
        .add_system(move_apes)
        .add_system(trigger_ape_attack)
        .add_system(ape_attacks_player_collision)
        .add_system(animate_apes_attacks)
        .add_system(animate_apes_wounds)
        .add_system(display_dead_apes_counter)
        // Process unit changes
        .add_stage_before(
            CoreStage::PostUpdate,
            "update_units",
            SystemStage::parallel(),
        )
        .add_system_to_stage("update_units", update_units)
        // Game logic resources
        .init_resource::<Events<UnitChanged>>()
        .init_resource::<Events<UnitAttack>>()
        .init_resource::<InputKind>()
        .run();
}

fn setup(
    mut commands: Commands,
    asset_server: Res<AssetServer>,
    mut texture_atlases: ResMut<Assets<TextureAtlas>>,
) {
    spawn_camera(&mut commands);
    spawn_background(&mut commands, &asset_server);
    spawn_platform(&mut commands, &asset_server);

    spawn_player(&mut commands, &asset_server, &mut texture_atlases);
    spawn_life_hud(&mut commands, &asset_server);

    let font_handle = spawn_font(&asset_server);
    init_ape_icon(&mut commands, &asset_server);
    spawn_dead_apes_counter(&mut commands, &asset_server, font_handle);
    spawn_ape(&mut commands, &asset_server, &mut texture_atlases);

    init_eth(&mut commands, &asset_server, &mut texture_atlases);
    spawn_ethbar(&mut commands, &asset_server);
}
