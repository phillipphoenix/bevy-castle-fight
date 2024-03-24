use crate::buildings::*;
use crate::common_components::*;
use crate::grid_traits::SnapToGrid;
use crate::units::spawn_unit;
use crate::waypoint_plugin::WaypointMap;
use crate::MousePosition;
use bevy::asset::AssetServer;
use bevy::prelude::*;
use bevy::prelude::{Commands, Query, Res, Time, Transform};
use bevy::utils::HashMap;
use bevy_rapier2d::pipeline::CollisionEvent;

pub struct BuildingPlugin;

impl Plugin for BuildingPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(
            Update,
            (
                unit_spawner_spawn_units,
                init_building,
                update_ghost_building_position,
                cancel_building,
                ghost_building_collision_system,
                building_placement,
            ),
        );
    }
}

fn unit_spawner_spawn_units(
    mut commands: Commands,
    asset_server: Res<AssetServer>,
    waypoint_map: Res<WaypointMap>,
    time: Res<Time>,
    mut query: Query<(&mut UnitSpawner, &Transform, &TeamEntity)>,
) {
    for (mut unit_spawner, transform, unit) in query.iter_mut() {
        if unit_spawner.time_left > 0. {
            unit_spawner.time_left -= time.delta_seconds()
        } else {
            spawn_unit(
                &mut commands,
                unit.team,
                asset_server.load("prototype-unit.png"),
                transform.translation.x,
                transform.translation.y,
                &waypoint_map,
            );
            unit_spawner.time_left = unit_spawner.spawn_time
        }
    }
}

fn init_building(
    mut commands: Commands,
    keyboard_input: Res<ButtonInput<KeyCode>>,
    asset_server: Res<AssetServer>,
    query: Query<Entity, With<BuildingGhost>>,
    mouse_position: Res<MousePosition>,
) {
    if keyboard_input.just_pressed(KeyCode::KeyQ) && query.is_empty() {
        let building_sprite = asset_server.load("prototype-building.png");
        spawn_ghost_building(
            &mut commands,
            Team::TeamRed,
            mouse_position.x,
            mouse_position.y,
            building_sprite,
        )
    }
    if keyboard_input.just_pressed(KeyCode::KeyW) && query.is_empty() {
        let building_sprite = asset_server.load("prototype-building.png");
        spawn_ghost_building(
            &mut commands,
            Team::TeamBlue,
            mouse_position.x,
            mouse_position.y,
            building_sprite,
        )
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
    asset_server: Res<AssetServer>,
) {
    if let Ok((ghost_entity, ghost_building, ghost_transform)) = ghost_query.get_single_mut() {
        if ghost_building.placement_valid && mouse_button_input.just_pressed(MouseButton::Left) {
            commands.entity(ghost_entity).despawn_recursive();
            let building_sprite = asset_server.load("prototype-building.png");
            spawn_building(
                &mut commands,
                ghost_building.team,
                ghost_transform.translation.x,
                ghost_transform.translation.y,
                building_sprite,
            )
        }
    }
}
