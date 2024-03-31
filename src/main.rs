//use bevy::input::common_conditions::input_just_pressed;
use bevy::{
    prelude::*,
    render::{settings::WgpuSettings, RenderPlugin},
};
use bevy_ecs_ldtk::prelude::*;


mod game;
mod main_menu;
mod inspector_plugin;
mod systems;


use inspector_plugin::InspectorPlugin;
use game::GamePlugin;
use main_menu::MainMenuPlugin;
use systems::*;

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
        //States
        .insert_state(AppState::Game)
        //State transitions
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
        .add_systems(Update, transition_to_gamestate)
        .add_systems(Update,transition_to_main_menu_state)
        .run();
}

fn setup(mut commands: Commands, asset_server: Res<AssetServer>) {
    commands.spawn(LdtkWorldBundle {
        ldtk_handle: asset_server.load("maps/map-0.ldtk"),
        ..Default::default()
    });
}

// States
#[derive(States, Debug, Clone, PartialEq, Eq, Hash, Default)]
pub enum AppState {
    #[default]
    Game,
    MainMenu,
}