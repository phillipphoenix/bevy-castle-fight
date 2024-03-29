use crate::health::Health;
use crate::vision::InVision;
use bevy::prelude::*;

// --- Plugin ---

pub struct AttackPlugin;

impl Plugin for AttackPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(PreUpdate, find_attack_target)
            .add_systems(Update, attack_target);
    }
}

// --- Components ---

#[derive(Component, Debug)]
pub struct AttackStats {
    pub damage: i32,
    pub attack_speed: f32,
    /// Used to check if the attack target is within striking range.
    pub attack_range: f32,
    pub time_till_next_attack: f32,
}

#[derive(Component)]
pub struct AttackTarget(pub Entity); // TODO: Make it contain a list of targets.
                                     // The reason for the above to do is that when a target is destroyed or out of sight
                                     // new potential targets that are already inside the sensor collider range will not be
                                     // be used as new targets. We therefore need to store all targets within the vision range.
                                     // NOTE: Perhaps we could have the list on another component (AttackTargetsInVision)
                                     // This component would always create a new AttackTarget component with the next target, if none exist.

// --- Systems ---

#[allow(clippy::type_complexity)]
fn find_attack_target(
    mut commands: Commands,
    query: Query<(Entity, &InVision), (With<AttackStats>, Without<AttackTarget>)>,
) {
    for (entity, in_vision) in query.iter() {
        if !in_vision.enemies.is_empty() {
            let target = in_vision.enemies[0];
            commands.entity(entity).insert(AttackTarget(target));
        }
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
