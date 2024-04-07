use bevy::prelude::*;

use crate::AppState;

pub fn transition_to_game_state(
    keyboard_input: Res<ButtonInput<KeyCode>>,
    simulation_state: Res<State<AppState>>,
    mut next_state: ResMut<NextState<AppState>>,
) {
    let current_state = simulation_state.get();
    if keyboard_input.just_pressed(KeyCode::KeyG)
        && *current_state != AppState::Game
        && *current_state != AppState::LoadGameAssets
    {
        next_state.set(AppState::LoadGameAssets);
        println!("Entered Load Game State")
    }
}

pub fn transition_to_main_menu_state(
    keyboard_input: Res<ButtonInput<KeyCode>>,
    simulation_state: Res<State<AppState>>,
    mut next_state: ResMut<NextState<AppState>>,
) {
    if keyboard_input.just_pressed(KeyCode::KeyM) && *simulation_state.get() == AppState::Game {
        next_state.set(AppState::MainMenu);
        println!("Entered Main Menu State")
    }
}
