use bevy::prelude::*;
use bevy_ecs_ldtk::prelude::*;
use bevy_rapier2d::prelude::*;

use crate::buildings::{Building, Castle};
use crate::health::Health;
use crate::teams::Team;
use crate::waypoints::{IsStartPoint, Waypoint};

/*
Handles all LDtk bundles and processing and resolving of these components.
*/

// --- Plugin ---

pub struct CastleFightLdtkPlugin;

impl Plugin for CastleFightLdtkPlugin {
    fn build(&self, app: &mut App) {
        app.register_ldtk_entity::<CastleBundle>("Castle")
            .register_ldtk_entity::<WaypointBundle>("Waypoint")
            .add_systems(Update, (process_castle, resolve_next_waypoint_references));
    }
}

// --- Components ---

#[derive(Default, Bundle, LdtkEntity)]
pub struct CastleBundle {
    castle: Castle,
    building: Building,
    #[sprite_bundle]
    sprite_bundle: SpriteBundle,
    #[with(Team::from_field)]
    team: Team,
    #[with(Health::from_field)]
    health: Health,
}

/// Used to load waypoints from LDTK map.
#[derive(Bundle, LdtkEntity)]
struct WaypointBundle {
    #[sprite_bundle]
    sprite_bundle: SpriteBundle,
    #[with(Team::from_field)]
    team: Team,
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

// --- Systems ---

/// Will take in unresolved waypoint references and turn them into actual waypoint references.
/// It runs whenever a new UnresolvedNextWaypointRef component is added to an entity.
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

fn process_castle(mut commands: Commands, new_castles: Query<(Entity, &Team), Added<Castle>>) {
    for (entity, team) in new_castles.iter() {
        let mut castle = commands.entity(entity);
        castle.insert((
            RigidBody::KinematicPositionBased,
            // Add below back, if building has attack and a vision range.
            // Collider::cuboid(32.0 * 2, 32.0 * 2),
            // Sensor,
            CollisionGroups::new(Group::GROUP_2, Group::GROUP_1),
            ActiveCollisionTypes::all(), // TODO: Optimize later.
            ActiveEvents::COLLISION_EVENTS,
        ));

        let text_color = team.get_color();

        castle.with_children(|builder| {
            builder.spawn((
                Collider::cuboid(96. / 2., 96. / 2.), // Actual collider matching sprite size.
                CollisionGroups::new(Group::GROUP_1, Group::GROUP_2 | Group::GROUP_3),
            ));
        });

        castle.with_children(|builder| {
            builder.spawn(Text2dBundle {
                text: Text {
                    sections: vec![TextSection::new(
                        team.to_string(),
                        TextStyle {
                            color: text_color,
                            ..Default::default()
                        },
                    )],
                    ..Default::default()
                },
                transform: Transform::from_translation(Vec3::new(0.0, -60.0, 1.0)),
                ..Default::default()
            });
        });
    }
}
