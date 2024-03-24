use crate::common_components::{Team, TeamEntity};
use bevy::prelude::*;
use bevy::utils::HashMap;
use bevy_ecs_ldtk::prelude::*;

pub struct WaypointPlugin;

impl Plugin for WaypointPlugin {
    fn build(&self, app: &mut App) {
        app.insert_resource(WaypointMap {
            start_point_waypoints: HashMap::default(),
        })
        .register_ldtk_entity::<WaypointBundle>("Waypoint")
        .add_systems(
            Update,
            (
                resolve_next_waypoint_references,
                add_start_waypoints_to_resources,
            ),
        );
    }
}

#[derive(Component, Reflect)]
pub struct Waypoint {
    pub id: Option<String>,
    pub next_waypoint: Option<Entity>,
}

#[derive(Component)]
pub struct WaypointFollower {
    pub waypoint: Entity,
}

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

/// Used to load waypoints from LDTK map.
#[derive(Bundle, LdtkEntity)]
struct WaypointBundle {
    #[sprite_bundle]
    sprite_bundle: SpriteBundle,
    #[with(TeamEntity::from_field)]
    team_entity: TeamEntity,
    #[with(UnresolvedNextWaypointRef::from_field)]
    unresolved_next_waypoint: UnresolvedNextWaypointRef,
    #[with(IsStartPoint::from_field)]
    is_start_point: IsStartPoint,
}

/// Will be resolved into a waypoint upon being added to an entity.
#[derive(Debug, Default, Component)]
struct UnresolvedNextWaypointRef(Option<EntityIid>);

impl UnresolvedNextWaypointRef {
    pub fn from_field(entity_instance: &EntityInstance) -> UnresolvedNextWaypointRef {
        UnresolvedNextWaypointRef(
            entity_instance
                .get_maybe_entity_ref_field("nextWaypoint")
                .expect("Expected waypoint to have a nextWaypoint field.")
                .as_ref()
                .map(|entity_ref| EntityIid::new(entity_ref.entity_iid.clone())),
        )
    }
}

fn resolve_next_waypoint_references(
    mut commands: Commands,
    unresolved_next_waypoint_refs: Query<
        (Entity, &UnresolvedNextWaypointRef),
        Added<UnresolvedNextWaypointRef>,
    >,
    ldtk_entities: Query<(Entity, &EntityIid)>,
) {
    for (entity, unresolved_next_wp_ref) in unresolved_next_waypoint_refs.iter() {
        if let Some(next_wp_iid) = unresolved_next_wp_ref.0.as_ref() {
            let (next_wp_entity, _) = ldtk_entities
                .iter()
                .find(|(_, iid)| *iid == next_wp_iid)
                .expect("The referenced next waypoint should exist.");
            commands
                .entity(entity)
                .remove::<UnresolvedNextWaypointRef>()
                .insert(Waypoint {
                    id: None,
                    next_waypoint: Some(next_wp_entity),
                });
        } else {
            // If we can't resolve the reference to an IID,
            // the entity probably doesn't exist, and we will add a default waypoint.
            commands
                .entity(entity)
                .remove::<UnresolvedNextWaypointRef>()
                .insert(Waypoint {
                    id: None,
                    next_waypoint: None,
                });
        }
    }
}

fn add_start_waypoints_to_resources(
    query: Query<(Entity, &TeamEntity, &Transform), Added<Waypoint>>,
    mut waypoint_map: ResMut<WaypointMap>,
) {
    for (new_entity, team_entity, transform) in query.iter() {
        let team_waypoint_list = waypoint_map
            .start_point_waypoints
            .entry(team_entity.team)
            .or_insert(Vec::new());
        team_waypoint_list.push((new_entity, transform.translation.xy()));
    }
}

/// This method was just for testing purposes (spawning a couple of waypoints).
pub fn add_waypoints(
    mut commands: Commands,
    asset_server: Res<AssetServer>,
    mut waypoint_map: ResMut<WaypointMap>,
) {
    let spawn_waypoint = |commands: &mut Commands,
                          asset_server: &Res<AssetServer>,
                          id: Option<String>,
                          x: f32,
                          y: f32,
                          next_waypoint: Option<Entity>|
     -> Entity {
        let entity = commands
            .spawn((
                Waypoint {
                    id: id.clone(),
                    next_waypoint,
                },
                SpriteBundle {
                    transform: Transform::from_xyz(x, y, 0.),
                    texture: asset_server.load("prototype-flag.png"),
                    ..Default::default()
                },
            ))
            .id();

        // if let Some(ref id_str) = id {
        //     waypoint_map.all_waypoints.insert(id_str.clone(), entity);
        // }

        entity
    };

    // --- RED team waypoints
    let waypoint3_red = spawn_waypoint(
        &mut commands,
        &asset_server,
        None,
        32. * -5.,
        32. * 5.,
        None,
    );
    let waypoint2_red = spawn_waypoint(
        &mut commands,
        &asset_server,
        None,
        0.,
        32. * 5.,
        Some(waypoint3_red),
    );
    let waypoint1_red = spawn_waypoint(
        &mut commands,
        &asset_server,
        Some("FirstRed".to_string()),
        0.,
        0.,
        Some(waypoint2_red),
    );

    // Make the waypoints loop.
    commands.entity(waypoint3_red).insert(Waypoint {
        id: None,
        next_waypoint: Some(waypoint1_red),
    });

    // --- BLUE team waypoints
    let waypoint3_blue =
        spawn_waypoint(&mut commands, &asset_server, None, 32. * 5., 32. * 5., None);
    let waypoint2_blue = spawn_waypoint(
        &mut commands,
        &asset_server,
        None,
        0.,
        32. * 5.,
        Some(waypoint3_blue),
    );
    let waypoint1_blue = spawn_waypoint(
        &mut commands,
        &asset_server,
        Some("FirstBlue".to_string()),
        0.,
        0.,
        Some(waypoint2_blue),
    );

    // Make the waypoints loop.
    commands.entity(waypoint3_blue).insert(Waypoint {
        id: None,
        next_waypoint: Some(waypoint1_blue),
    });
}
