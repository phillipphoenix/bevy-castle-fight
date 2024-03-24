use crate::common_components::*;
use crate::units::*;
use crate::waypoint_plugin::{Waypoint, WaypointFollower};
use bevy::prelude::*;
use bevy_rapier2d::prelude::*;

pub struct UnitPlugin;

impl Plugin for UnitPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(
            Update,
            (
                vision_detect_target.before(sync_attack_move_target),
                sync_attack_move_target,
                sync_waypoint_move_target.after(sync_attack_move_target),
            )
                .before(move_towards_target),
        )
        .add_systems(Update, move_towards_target.before(move_towards_point))
        .add_systems(Update, move_towards_point)
        .add_systems(Update, attack_target);
    }
}

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

/// If entity is an opponent follower and has an attack target, it should become the move target.
fn sync_attack_move_target(
    mut commands: Commands,
    mut attack_move_target_query: Query<
        (Entity, Option<&mut MoveTarget>, &AttackTarget),
        With<OpponentFollower>,
    >,
) {
    for (entity, opt_move_target, attack_target) in attack_move_target_query.iter_mut() {
        match opt_move_target {
            Some(mut move_target) => {
                if move_target.0 != attack_target.0 {
                    move_target.0 = attack_target.0;
                }
            }
            None => {
                commands.entity(entity).insert(MoveTarget(attack_target.0));
            }
        }
    }
}

fn sync_waypoint_move_target(
    mut commands: Commands,
    mut query: Query<
        (
            Entity,
            &Transform,
            Option<&mut MoveTarget>,
            &mut WaypointFollower,
        ),
        Without<AttackTarget>,
    >,
    waypoint_query: Query<(&Transform, &Waypoint)>,
) {
    for (entity, transform, opt_move_target, mut waypoint_follower) in query.iter_mut() {
        match opt_move_target {
            Some(mut move_target) => {
                // If current move target is the current waypoint to follow.
                // we check if we are close enough to the waypoint to switch to the next one.
                if move_target.0 == waypoint_follower.waypoint {
                    if let Ok((waypoint_transform, waypoint)) = waypoint_query.get(move_target.0) {
                        let direction = waypoint_transform.translation - transform.translation;
                        let distance = direction.length();

                        if distance < 32.0 {
                            // Check if there is a next waypoint.
                            if let Some(next_waypoint) = waypoint.next_waypoint {
                                waypoint_follower.waypoint = next_waypoint;
                                move_target.0 = next_waypoint;
                            } else {
                                // If there is no next waypoint, remove the waypoint follower.
                                // Also remove the move target, as we have nothing to follow.
                                commands.entity(entity).remove::<WaypointFollower>();
                                commands.entity(entity).remove::<MoveTarget>();
                            }
                        }
                    }
                }
            }
            None => {
                commands
                    .entity(entity)
                    .insert(MoveTarget(waypoint_follower.waypoint));
            }
        }
    }
}

fn move_towards_target(
    mut commands: Commands,
    mut query: Query<(Entity, &MoveTarget)>,
    target_query: Query<&Transform>,
) {
    for (entity, move_target) in query.iter_mut() {
        if let Ok(target_transform) = target_query.get(move_target.0) {
            commands
                .entity(entity)
                .insert(MoveToPoint(target_transform.translation.truncate()));
        } else {
            // If target doesn't have a transform, it probably doesn't exist.
            // Therefore, we remove the target.
            commands.entity(entity).remove::<MoveTarget>();
        }
    }
}

/// Moves towards one-time-use points.
/// Should always run after systems inserting MoveToPoint components on entities.
fn move_towards_point(
    mut commands: Commands,
    mut query: Query<(Entity, &mut Transform, &MoveToPoint, &MovementSpeed)>,
    time: Res<Time>,
) {
    for (entity, mut transform, move_to_point, movement_speed) in query.iter_mut() {
        let direction = move_to_point.0 - transform.translation.xy();
        // Move towards the target.
        let move_direction = direction.normalize();
        let move_amount = (move_direction.xy() * movement_speed.0 * time.delta_seconds())
            .clamp_length_max(direction.length())
            .extend(0.);
        transform.translation += move_amount;
        // Remove the move to point.
        commands.entity(entity).remove::<MoveToPoint>();
    }
}

fn attack_target(
    mut commands: Commands,
    mut attacker_query: Query<(Entity, &mut AttackStats, &AttackTarget)>,
    mut defender_query: Query<&mut Health>,
    time: Res<Time>,
) {
    for (entity, mut attack_stats, target) in attacker_query.iter_mut() {
        // Don't attack, if attack cooldown hasn't finished.
        if attack_stats.time_till_next_attack > 0. {
            attack_stats.time_till_next_attack -= time.delta_seconds();
            return;
        }

        if let Ok(mut health) = defender_query.get_mut(target.0) {
            // TODO: Make a more intricate damage calculation.
            health.health -= attack_stats.damage;
            info!("{:?} damage taken!", attack_stats.damage);
            // Reset attack cooldown.
            attack_stats.time_till_next_attack = 1. / attack_stats.attack_speed;
        } else {
            // If the target has no health component,
            // it probably died, so lets remove the attack target.
            commands.entity(entity).remove::<AttackTarget>();

            // TODO: When fixing targeting, remove the target from the list, if it exist. Remove the component if there are no targets left.
        }
    }
}
