use bevy::prelude::*;

use crate::main_menu::init_screen::InitScreenPlugin;
use crate::AppState;

mod init_screen;

pub struct MainMenuPlugin;

impl Plugin for MainMenuPlugin {
    fn build(&self, app: &mut App) {
        app.add_plugins(InitScreenPlugin {
            state: AppState::MainMenu,
        })
        .add_systems(OnEnter(AppState::MainMenu), main_menu)
        .add_systems(OnExit(AppState::MainMenu), main_menu_cleanup);
    }
}

// --- Components ---

#[derive(Component)]
pub struct MainMenuTag;

pub fn main_menu() {
    println!("You are on the main menu");
}

pub fn main_menu_cleanup(mut commands: Commands, query: Query<Entity, With<MainMenuTag>>) {
    for main_menu_entity in query.iter() {
        commands.entity(main_menu_entity).despawn_recursive();
    }
}
