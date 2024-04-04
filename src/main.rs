use bevy::{
    prelude::*,
    render::{settings::WgpuSettings, RenderPlugin},
};

mod game;
mod inspector_plugin;
mod main_menu;
mod systems;

use game::GamePlugin;
use inspector_plugin::InspectorPlugin;
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
        .add_plugins((GamePlugin, MainMenuPlugin))
        // Systems.
        .add_systems(Update, transition_to_gamestate)
        .add_systems(Update, transition_to_main_menu_state)
        .run();
}

// States
#[derive(States, Debug, Clone, PartialEq, Eq, Hash, Default)]
pub enum AppState {
    #[default]
    Game,
    MainMenu,
}
