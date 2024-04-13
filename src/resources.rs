use bevy::prelude::*;

use crate::load_game::load_factions::FactionBlueprint;

#[derive(Resource)]
pub struct SelectedFaction(pub FactionBlueprint);
