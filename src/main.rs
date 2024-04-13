use bevy::{
    prelude::*,
    render::{settings::WgpuSettings, RenderPlugin},
};

use game::GamePlugin;
use inspector_plugin::InspectorPlugin;
use main_menu::MainMenuPlugin;
use systems::*;

use crate::load_game::LoadGamePlugin;

mod game;
mod inspector_plugin;
mod load_game;
mod main_menu;
mod systems;

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
        .insert_state(AppState::LoadGameAssets)
        //State transitions
        // Debug plugins.
        .add_plugins(InspectorPlugin)
        //Our plugins
        .add_plugins((MainMenuPlugin, LoadGamePlugin, GamePlugin))
        // Systems.
        .add_systems(Update, transition_to_game_state)
        .add_systems(Update, transition_to_main_menu_state)
        .run();
}

// States
#[derive(States, Debug, Clone, PartialEq, Eq, Hash, Default)]
pub enum AppState {
    #[default]
    LoadGameAssets,
    MainMenu,
    Game,
}
