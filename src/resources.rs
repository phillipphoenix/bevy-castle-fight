use crate::game::teams::Team;
use bevy::prelude::*;

use crate::load_game::load_factions::FactionBlueprint;

#[derive(Resource)]
pub struct PlayerSettings {
    pub team: Team,
    pub faction: FactionBlueprint,
}
