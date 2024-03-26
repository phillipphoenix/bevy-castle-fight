use crate::teams::TeamEntity;
use crate::units::spawn_unit;
use crate::waypoints::WaypointMap;
use bevy::prelude::*;

// --- Plugin ---

pub struct UnitSpawningPlugin;

impl Plugin for UnitSpawningPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(Update, unit_spawner_spawn_units);
    }
}

// --- Components ---

#[derive(Component)]
pub struct UnitSpawner {
    pub spawn_time: f32,
    pub time_left: f32,
}

// --- Systems ---

fn unit_spawner_spawn_units(
    mut commands: Commands,
    asset_server: Res<AssetServer>,
    waypoint_map: Res<WaypointMap>,
    time: Res<Time>,
    mut query: Query<(&mut UnitSpawner, &Transform, &TeamEntity)>,
) {
    for (mut unit_spawner, transform, unit) in query.iter_mut() {
        if unit_spawner.time_left > 0. {
            unit_spawner.time_left -= time.delta_seconds()
        } else {
            spawn_unit(
                &mut commands,
                unit.team,
                asset_server.load("prototype-unit.png"),
                transform.translation.x,
                transform.translation.y,
                &waypoint_map,
            );
            unit_spawner.time_left = unit_spawner.spawn_time
        }
    }
}
