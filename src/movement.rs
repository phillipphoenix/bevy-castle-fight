use crate::attack::AttackTarget;
use crate::waypoints::Waypoint;
use bevy::prelude::*;

// --- Plugin ---

pub struct MovementPlugin;

impl Plugin for MovementPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(
            Update,
            (
                sync_attack_move_target,
                sync_waypoint_move_target.after(sync_attack_move_target),
                move_towards_target,
                move_towards_point.after(move_towards_target),
            ),
        );
    }
}

// --- Components ---

#[derive(Component)]
pub struct MovementSpeed(pub f32);

#[derive(Component)]
pub struct OpponentFollower;

#[derive(Component, Reflect)]
pub struct WaypointFollower {
    pub waypoint: Entity,
}

#[derive(Component, Reflect)]
pub struct MoveTarget(pub Entity);

/// One-time use points that are used to move to.
#[derive(Component, Reflect)]
pub struct MoveToPoint(pub Vec2);

// --- Systems ---

/// Moves towards MoveToPoint (which are on-time-use and will be removed after use).
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
            // .clamp_length_max(direction.length())
            .extend(0.);
        transform.translation += move_amount;
        // Remove the move to point.
        commands.entity(entity).remove::<MoveToPoint>();
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

                        if distance < 64.0 {
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
