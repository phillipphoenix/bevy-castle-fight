use bevy::ecs::system::EntityCommands;
use bevy::prelude::Res;

use crate::game::attack::AttackStats;
use crate::game::health::Health;
use crate::game::movement::{MovementSpeed, OpponentFollower};
use crate::game::unit_spawning::UnitSpawner;
use crate::game::vision::{InVision, Visible, VisionRange};
use crate::load_game::load_factions::ComponentBlueprint;
use crate::resources::PlayerSettings;

pub fn add_blueprint_components(
    entity_commands: &mut EntityCommands,
    component_blueprints: &Vec<ComponentBlueprint>,
    player_settings: &Res<PlayerSettings>,
) {
    for component_blueprint in component_blueprints.iter() {
        match component_blueprint {
            ComponentBlueprint::Health { max_health, health } => {
                entity_commands.insert(Health {
                    health: *health,
                    max_health: *max_health,
                });
            }
            ComponentBlueprint::Visible => {
                entity_commands.insert(Visible);
            }
            ComponentBlueprint::OpponentFollower => {
                entity_commands.insert(OpponentFollower);
            }
            ComponentBlueprint::MovementSpeed(speed) => {
                entity_commands.insert(MovementSpeed((*speed) as f32));
            }
            ComponentBlueprint::AttackStats {
                damage,
                attack_speed,
                attack_range,
            } => {
                entity_commands.insert(AttackStats {
                    damage: *damage,
                    attack_speed: *attack_speed,
                    attack_range: *attack_range as f32,
                    time_till_next_attack: 0.,
                });
            }
            ComponentBlueprint::UnitSpawner {
                unit_id,
                spawn_time,
            } => {
                entity_commands.insert(UnitSpawner {
                    spawn_time: *spawn_time,
                    time_left: *spawn_time,
                    unit_blueprint: player_settings.faction.units[unit_id].clone(),
                });
            }
            ComponentBlueprint::VisionRange(range) => {
                entity_commands.insert(VisionRange(*range));
                // For vision to work, the InVision component must be present too.
                entity_commands.insert(InVision {
                    friendlies: vec![],
                    enemies: vec![],
                });
            }
        }
    }
}
