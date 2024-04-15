use bevy::prelude::*;

use crate::game::teams::Team;
use crate::game::units::spawn_unit;
use crate::game::waypoints::WaypointMap;
use crate::load_game::load_factions::UnitBlueprint;
use crate::resources::PlayerSettings;

// --- Plugin ---

pub struct UnitSpawningPlugin<S: States> {
    pub state: S,
}

impl<S: States> Plugin for UnitSpawningPlugin<S> {
    fn build(&self, app: &mut App) {
        app.add_systems(
            Update,
            unit_spawner_spawn_units.run_if(in_state(self.state.clone())),
        );
    }
}

// --- Components ---

#[derive(Component)]
pub struct UnitSpawner {
    pub spawn_time: f32,
    pub time_left: f32,
    pub unit_blueprint: UnitBlueprint,
}

// --- Systems ---

fn unit_spawner_spawn_units(
    mut commands: Commands,
    waypoint_map: Res<WaypointMap>,
    time: Res<Time>,
    mut query: Query<(&mut UnitSpawner, &Transform, &Team)>,
    player_settings: Res<PlayerSettings>,
) {
    for (mut unit_spawner, transform, team) in query.iter_mut() {
        if unit_spawner.time_left > 0. {
            unit_spawner.time_left -= time.delta_seconds()
        } else {
            spawn_unit(
                &mut commands,
                *team,
                unit_spawner.unit_blueprint.clone(),
                transform.translation.x,
                transform.translation.y,
                &waypoint_map,
                &player_settings,
            );
            unit_spawner.time_left = unit_spawner.spawn_time
        }
    }
}
