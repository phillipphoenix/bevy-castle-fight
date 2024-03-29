use bevy::input::common_conditions::input_just_pressed;
use bevy::{
    prelude::*,
    render::{settings::WgpuSettings, RenderPlugin},
};
use bevy_ecs_ldtk::prelude::*;


mod game;
mod main_menu;
mod inspector_plugin;

use inspector_plugin::InspectorPlugin;
use game::GamePlugin;
use main_menu::MainMenuPlugin;

fn main() {
    App::new()
        .add_plugins((DefaultPlugins.set(RenderPlugin {
            render_creation: bevy::render::settings::RenderCreation::Automatic(WgpuSettings {
                // This is necessary to remove infinite errors on Windows machines with AMD GPUs.
                // Might not be required in the future - (2024-03-14).
                backends: Some(bevy::render::settings::Backends::VULKAN),
                ..Default::default()
            }),
            synchronous_pipeline_compilation: false,
        }),))
        // Debug plugins.
        .add_plugins(InspectorPlugin)
        //Our plugins
        .add_plugins((
            GamePlugin,
            MainMenuPlugin
        ))
        // Third party plugins.
        .add_plugins(LdtkPlugin)
        // Third party resources
        .insert_resource(LevelSelection::index(0))
        // Systems.
        .add_systems(Startup, setup)
        .add_systems(
            Update,
            toggle_pause.run_if(input_just_pressed(KeyCode::Space)),
        )
        .run();
}

fn toggle_pause(mut time: ResMut<Time<Virtual>>) {
    if time.is_paused() {
        time.unpause();
    } else {
        time.pause();
    }
}

fn setup(mut commands: Commands, asset_server: Res<AssetServer>) {
    commands.spawn(LdtkWorldBundle {
        ldtk_handle: asset_server.load("maps/map-0.ldtk"),
        ..Default::default()
    });
}
