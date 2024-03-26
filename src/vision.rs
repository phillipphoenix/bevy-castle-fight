use crate::attack::{AttackStats, AttackTarget};
use crate::health::Health;
use crate::movement::MoveTarget;
use crate::teams::TeamEntity;
use bevy::prelude::*;
use bevy_rapier2d::geometry::Sensor;
use bevy_rapier2d::pipeline::CollisionEvent;

// --- Plugin ---

pub struct VisionPlugin;

impl Plugin for VisionPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(Update, vision_detect_target);
    }
}

// --- Components ---

/// From how far away can an entity spot for instance opponents.
#[derive(Component)]
pub struct VisionRange(f32);

// --- Systems ---

/// Check if a sensor collider detects another unit.
/// If it does detect a unit, and it is from another team, set that as the attack target.
fn vision_detect_target(
    mut commands: Commands,
    mut collisions: EventReader<CollisionEvent>,
    attack_query: Query<(Entity, &TeamEntity), (With<AttackStats>, Without<AttackTarget>)>,
    defend_query: Query<(Entity, &TeamEntity), With<Health>>,
    attack_target_query: Query<(Entity, &AttackTarget, Option<&MoveTarget>)>,
    sensor_query: Query<&Sensor>,
    parent_query: Query<&Parent>,
) {
    let sort_entities = |entity1: &Entity, entity2: &Entity| -> Option<(Entity, Entity)> {
        if sensor_query.get(*entity1).is_ok() {
            let opt_rigid_body_entity = parent_query.get(*entity2);
            if let Ok(seen_entity) = opt_rigid_body_entity {
                return Some((*entity1, seen_entity.get()));
            }
        } else if sensor_query.get(*entity2).is_ok() {
            let opt_rigid_body_entity = parent_query.get(*entity1);
            if let Ok(seen_entity) = opt_rigid_body_entity {
                return Some((*entity2, seen_entity.get()));
            }
        }
        None
    };

    let check_set_target = |entity1: &Entity, entity2: &Entity, commands: &mut Commands| {
        if let Ok((attacker_entity, attacker_team)) = attack_query.get(*entity1) {
            if let Ok((defender_entity, defender_team)) = defend_query.get(*entity2) {
                if attacker_team.team != defender_team.team {
                    commands
                        .entity(attacker_entity)
                        .insert(AttackTarget(defender_entity));
                }
            } else {
                info!("Defender not found...");
            }
        } else {
            info!("Attacker not found...")
        }
    };

    let check_remove_target = |entity1: &Entity, entity2: &Entity, commands: &mut Commands| {
        if let Ok((attacker_entity, target, opt_move_target)) = attack_target_query.get(*entity1) {
            if target.0 == *entity2 {
                commands.entity(attacker_entity).remove::<AttackTarget>();
                // If the unit is also moving towards the attack target, remove the move target too.
                if let Some(move_target) = opt_move_target {
                    if move_target.0 == *entity2 {
                        commands.entity(attacker_entity).remove::<MoveTarget>();
                    }
                }
            }
        }
    };

    for collision in collisions.read() {
        match collision {
            CollisionEvent::Started(entity1, entity2, _) => {
                if let Some((viewer, seen_entity)) = sort_entities(entity1, entity2) {
                    info!("Viewer: {:?} has seen: {:?}", viewer, seen_entity);
                    check_set_target(&viewer, &seen_entity, &mut commands);
                }
            }
            CollisionEvent::Stopped(entity1, entity2, _) => {
                if let Some((viewer, seen_entity)) = sort_entities(entity1, entity2) {
                    check_remove_target(&viewer, &seen_entity, &mut commands);
                }
            }
        }
    }
}
