use bevy::prelude::*;

use crate::load_game::load_factions::FactionLoaderPlugin;
use crate::load_game::LoadingSet::{LoadStartup, LoadUpdate};
use crate::AppState;

pub mod load_factions;

pub struct LoadGamePlugin;

impl Plugin for LoadGamePlugin {
    fn build(&self, app: &mut App) {
        app
            // System sets.
            .configure_sets(OnEnter(AppState::LoadGameAssets), LoadStartup)
            .configure_sets(
                Update,
                LoadUpdate
                    .run_if(in_state(AppState::LoadGameAssets))
                    .after(LoadStartup),
            )
            // Plugins.
            .add_plugins(FactionLoaderPlugin)
            // Third party plugins.
            // Third party resources
            // State Transitions
            // Systems
            .add_systems(Update, apply_deferred.after(LoadStartup).before(LoadUpdate));
    }
}

// Sets - Groups of systems

#[derive(SystemSet, Debug, Clone, PartialEq, Eq, Hash)]
pub enum LoadingSet {
    LoadStartup,
    LoadUpdate,
}
