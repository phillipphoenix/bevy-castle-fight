use bevy::prelude::*;

use crate::AppState;

pub fn transition_to_gamestate(
    mut commands: Commands,
    keyboard_input: Res<ButtonInput<KeyCode>>,
    simulation_state: Res<State<AppState>>,
) {
    if keyboard_input.just_pressed(KeyCode::KeyG) {
        if simulation_state.get() != &AppState::Game {
            commands.insert_resource(NextState(Some(AppState::Game)));
        println!("Entered Game State")
        } 
    }
        
}

pub fn transition_to_main_menu_state(
    mut commands: Commands,
    keyboard_input: Res<ButtonInput<KeyCode>>,
    simulation_state: Res<State<AppState>>,
) {
    if keyboard_input.just_pressed(KeyCode::KeyM) {
        if simulation_state.get() != &AppState::MainMenu {
            commands.insert_resource(NextState(Some(AppState::MainMenu)));
        println!("Entered Main Menu State")
        }
    } 
}