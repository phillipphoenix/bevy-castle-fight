use crate::game::teams::Team;
use bevy::prelude::PostUpdate;
use bevy::prelude::*;
use bevy_ecs_ldtk::prelude::LdtkFields;
use bevy_ecs_ldtk::EntityInstance;

// --- Plugin ---

pub struct HealthPlugin;

impl Plugin for HealthPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(PostUpdate, check_death);
    }
}

// --- Components ---

#[derive(Default, Component, Debug, Reflect)]
pub struct Health {
    pub max_health: i32,
    pub health: i32,
}

impl Health {
    pub fn from_field(entity_instance: &EntityInstance) -> Health {
        let health = entity_instance
            .get_int_field("health")
            .expect("This entity should have a health field.");
        Health {
            health: *health,
            max_health: *health,
        }
    }
}

// --- Systems ---

/// Should run after attack_system.
fn check_death(mut commands: Commands, query: Query<(Entity, &Health), With<Team>>) {
    for (entity, health) in query.iter() {
        if health.health <= 0 {
            commands.entity(entity).despawn_recursive();
            info!("{:?} died as health was depleted!", entity);
        }
    }
}
