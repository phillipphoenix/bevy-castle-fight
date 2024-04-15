use bevy::prelude::*;
use bevy::utils::HashMap;
use bevy_rapier2d::pipeline::CollisionEvent;

use crate::game::buildings::{spawn_building, spawn_ghost_building, Building, BuildingGhost};
use crate::game::grid_traits::SnapToGrid;
use crate::game::resources::MousePosition;
use crate::game::teams::Team;
use crate::load_game::load_factions::BuildingBlueprint;
use crate::resources::PlayerSettings;

// --- Plugin ---

pub struct BuildingSpawningPlugin<S: States> {
    pub state: S,
}

impl<S: States> Plugin for BuildingSpawningPlugin<S> {
    fn build(&self, app: &mut App) {
        app.add_event::<InitPlaceBuildingEvent>().add_systems(
            Update,
            (
                on_init_place_building,
                update_ghost_building_position.after(on_init_place_building),
                cancel_building,
                ghost_building_collision_system,
                building_placement,
            )
                .run_if(in_state(self.state.clone())),
        );
    }
}

// --- Events ---

#[derive(Event)]
pub struct InitPlaceBuildingEvent(pub BuildingBlueprint, pub Team);

// --- Systems ---

fn on_init_place_building(
    mut commands: Commands,
    mut ev_place_building: EventReader<InitPlaceBuildingEvent>,
    query: Query<Entity, With<BuildingGhost>>,
    mouse_position: Res<MousePosition>,
) {
    // If there is already a ghost building, clear the queue,
    // as we can only build one building at a time.
    if !query.is_empty() {
        ev_place_building.clear();
        return;
    }

    for ev in ev_place_building.read() {
        let building_blueprint = &ev.0;
        let team = &ev.1;
        spawn_ghost_building(
            &mut commands,
            *team,
            mouse_position.x,
            mouse_position.y,
            building_blueprint.clone(),
        );
    }
}

fn update_ghost_building_position(
    mut query: Query<&mut Transform, With<BuildingGhost>>,
    mouse_position: Res<MousePosition>,
) {
    if let Ok(mut ghost_transform) = query.get_single_mut() {
        let z = ghost_transform.translation.z;
        ghost_transform.translation =
            Vec3::new(mouse_position.x, mouse_position.y, z).snap_to_grid(32.);
    }
}

fn cancel_building(
    mut commands: Commands,
    mouse_button_input: Res<ButtonInput<MouseButton>>,
    query: Query<Entity, With<BuildingGhost>>,
) {
    if let Ok(ghost_entity) = query.get_single() {
        if mouse_button_input.just_pressed(MouseButton::Right) {
            commands.entity(ghost_entity).despawn_recursive();
        }
    }
}

fn ghost_building_collision_system(
    mut collisions: EventReader<CollisionEvent>,
    mut ghost_query: Query<(Entity, &mut BuildingGhost, &mut Sprite)>,
    building_query: Query<Entity, With<Building>>,
    parent_query: Query<&Parent>,
) {
    let sort_entities = |entity1: &Entity, entity2: &Entity| -> Option<(Entity, Entity)> {
        if ghost_query.get(*entity1).is_ok() {
            let opt_building_entity = parent_query.get(*entity2);
            if let Ok(building_entity) = opt_building_entity {
                return Some((*entity1, building_entity.get()));
            }
        } else if ghost_query.get(*entity2).is_ok() {
            let opt_building_entity = parent_query.get(*entity1);
            if let Ok(building_entity) = opt_building_entity {
                return Some((*entity2, building_entity.get()));
            }
        }
        None
    };

    // Contains ghosts and if their placement is valid.
    let mut ghost_collisions: HashMap<Entity, bool> = HashMap::new();

    // TODO: Change this to add collided entities to a list and remove them. Then check if the list is empty or not for if placement is valid.
    for event in collisions.read() {
        match event {
            CollisionEvent::Started(entity1, entity2, _) => {
                if let Some((ghost_entity, other_entity)) = sort_entities(entity1, entity2) {
                    info!("GHOST {:?} and OTHER {:?}", ghost_entity, other_entity);
                    if building_query.get(other_entity).is_ok() {
                        ghost_collisions.entry(ghost_entity).or_insert(false);
                    }
                }
            }
            CollisionEvent::Stopped(entity1, entity2, _) => {
                if let Some((ghost_entity, other_entity)) = sort_entities(entity1, entity2) {
                    if building_query.get(other_entity).is_ok() {
                        ghost_collisions.entry(ghost_entity).or_insert(true);
                    }
                }
            }
        }
    }

    // Update ghosts based on collected collision states.
    for (entity, placement_valid) in ghost_collisions.iter() {
        if let Ok((_, mut ghost_building, mut ghost_sprite)) = ghost_query.get_mut(*entity) {
            ghost_building.placement_valid = *placement_valid;
            ghost_sprite.color = if *placement_valid {
                Color::rgba(0.5, 1.0, 0.5, 0.7)
            } else {
                Color::rgba(1.0, 0.5, 0.5, 0.7)
            };
        }
    }
}

fn building_placement(
    mut commands: Commands,
    mouse_button_input: Res<ButtonInput<MouseButton>>,
    mut ghost_query: Query<(Entity, &BuildingGhost, &Transform), With<BuildingGhost>>,
    player_settings: Res<PlayerSettings>,
) {
    if let Ok((ghost_entity, ghost_building, ghost_transform)) = ghost_query.get_single_mut() {
        if ghost_building.placement_valid && mouse_button_input.just_pressed(MouseButton::Left) {
            commands.entity(ghost_entity).despawn_recursive();
            spawn_building(
                &mut commands,
                ghost_building.team,
                ghost_transform.translation.x,
                ghost_transform.translation.y,
                ghost_building.building_blueprint.clone(),
                &player_settings,
            )
        }
    }
}
