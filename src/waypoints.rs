use crate::teams::Team;
use bevy::prelude::*;
use bevy::utils::HashMap;
use bevy_ecs_ldtk::prelude::*;

// --- Plugin ---

pub struct WaypointPlugin;

impl Plugin for WaypointPlugin {
    fn build(&self, app: &mut App) {
        app.insert_resource(WaypointMap {
            start_point_waypoints: HashMap::default(),
        })
        .add_systems(Update, (add_start_waypoints_to_resources,));
    }
}

// --- Components ---

#[derive(Component, Reflect)]
pub struct Waypoint {
    pub id: Option<String>,
    pub next_waypoint: Option<Entity>,
}

/// The waypoint map is used to find the closest starting waypoint for each team.
#[derive(Resource)]
pub struct WaypointMap {
    pub start_point_waypoints: HashMap<Team, Vec<(Entity, Vec2)>>,
}

impl WaypointMap {
    pub fn get_closest_start_waypoint(&self, current_position: Vec2, team: Team) -> Option<Entity> {
        if let Some(team_waypoint_list) = self.start_point_waypoints.get(&team) {
            team_waypoint_list
                .iter()
                .min_by(|a, b| {
                    a.1.distance_squared(current_position)
                        .partial_cmp(&b.1.distance_squared(current_position))
                        .unwrap()
                })
                .map(|(entity, _)| *entity)
        } else {
            None // Return None, if the team has no waypoints.
        }
    }
}

#[derive(Component, Reflect)]
pub struct IsStartPoint(bool);

impl IsStartPoint {
    pub fn from_field(entity_instance: &EntityInstance) -> IsStartPoint {
        IsStartPoint(
            *entity_instance
                .get_bool_field("isStartPoint")
                .expect("Expect waypoints to have isStartPoint field."),
        )
    }
}

fn add_start_waypoints_to_resources(
    query: Query<(Entity, &Team, &Transform), Added<Waypoint>>,
    mut waypoint_map: ResMut<WaypointMap>,
) {
    for (new_entity, team, transform) in query.iter() {
        let team_waypoint_list = waypoint_map
            .start_point_waypoints
            .entry(*team)
            .or_insert(Vec::new());
        team_waypoint_list.push((new_entity, transform.translation.xy()));
    }
}
