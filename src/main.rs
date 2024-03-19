use bevy::{
    prelude::*,
    render::{settings::WgpuSettings, RenderPlugin},
    utils::HashMap,
};
use bevy_rapier2d::prelude::*;
use std::fmt;
use std::fmt::Formatter;

use waypoints::*;

mod waypoints;

#[derive(Clone, Copy, PartialEq, Debug)]
enum Team {
    TeamRed,
    TeamBlue,
}

impl fmt::Display for Team {
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
        match self {
            Team::TeamRed => write!(f, "RED"),
            Team::TeamBlue => write!(f, "BLUE"),
        }
    }
}

#[derive(Component, Debug)]
struct Unit {
    team: Team,
}

#[derive(Component)]
struct MovementSpeed(f32);

#[derive(Component, Debug)]
struct Health {
    max_health: f32,
    health: f32,
}

#[derive(Component, Debug)]
struct Attack {
    target: Option<Entity>,
    damage: f32,
    attack_speed: f32,
    time_till_next_attack: f32,
}

#[derive(Component)]
struct Building;

#[derive(Component)]
struct BuildingGhost {
    placement_valid: bool,
    team: Team,
}

#[derive(Component)]
struct UnitSpawner {
    spawn_time: f32,
    time_left: f32,
}

#[derive(Resource)]
struct MousePosition {
    x: f32,
    y: f32,
}

fn main() {
    App::new()
        .add_plugins((
            DefaultPlugins.set(RenderPlugin {
                render_creation: bevy::render::settings::RenderCreation::Automatic(WgpuSettings {
                    // This is necessary to remove infinite errors on Windows machines with AMD GPUs.
                    // Might not be required in the future - (2024-03-14).
                    backends: Some(bevy::render::settings::Backends::VULKAN),
                    ..Default::default()
                }),
                synchronous_pipeline_compilation: true,
            }),
            RapierPhysicsPlugin::<NoUserData>::default(),
            RapierDebugRenderPlugin::default(),
        ))
        .insert_resource(WaypointMap {
            all_waypoints: HashMap::default(),
        })
        .insert_resource(MousePosition { x: 0., y: 0. })
        .add_systems(Startup, (add_camera,))
        .add_systems(Startup, add_waypoints.before(add_units))
        // .add_systems(Startup, add_units)
        .add_systems(Startup, add_buildings)
        .add_systems(Update, get_mouse_position)
        .add_systems(
            Update,
            (
                go_to_next_waypoint,
                spawn_units,
                detect_targets_system,
                attack_system,
                check_death.after(attack_system),
            ),
        )
        .add_systems(
            Update,
            (
                building_system,
                update_ghost_building_position,
                cancel_building,
                ghost_building_collision_system,
                building_placement,
            ),
        )
        .run();
}

fn add_camera(mut commands: Commands) {
    commands.spawn(Camera2dBundle::default());
}

fn spawn_unit(
    commands: &mut Commands,
    team: Team,
    sprite: Handle<Image>,
    x: f32,
    y: f32,
    waypoint_map: &Res<WaypointMap>,
    waypoint_id: Option<String>,
) {
    let mut unit_entity = commands.spawn((
        Unit { team },
        Health {
            health: 5.,
            max_health: 5.,
        },
        Attack {
            target: None,
            attack_speed: 2.,
            damage: 1.,
            time_till_next_attack: 0.,
        },
        MovementSpeed(32. * 3.),
        SpriteBundle {
            transform: Transform::from_xyz(x, y, 0.),
            texture: sprite,
            ..Default::default()
        },
        Collider::ball(20.0),
        Sensor,
        ActiveCollisionTypes::all(), // TODO: Optimize later.
        ActiveEvents::COLLISION_EVENTS,
    ));

    let text_color = match team {
        Team::TeamRed => Color::RED,
        Team::TeamBlue => Color::BLUE,
    };

    unit_entity.with_children(|builder| {
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
            transform: Transform::from_translation(Vec3::new(0.0, -22.0, 1.0)),
            ..Default::default()
        });
    });

    if let Some(ref waypoint_id_str) = waypoint_id {
        let waypoint = waypoint_map.all_waypoints.get(waypoint_id_str).unwrap();

        unit_entity.insert(WaypointFollower {
            waypoint: *waypoint,
        });
    }
}

fn spawn_building(commands: &mut Commands, team: Team, x: f32, y: f32, sprite: Handle<Image>) {
    let mut building_entity = commands.spawn((
        Unit { team },
        Building,
        Health {
            health: 10.,
            max_health: 10.,
        },
        SpriteBundle {
            texture: sprite,
            transform: Transform::from_xyz(x, y, 0.),
            ..Default::default()
        },
        UnitSpawner {
            spawn_time: 5.0,
            time_left: 5.0,
        },
        Collider::cuboid(32.0, 32.0),
        Sensor,
        ActiveCollisionTypes::all(), // TODO: Optimize later.
        ActiveEvents::COLLISION_EVENTS,
    ));

    let text_color = match team {
        Team::TeamRed => Color::RED,
        Team::TeamBlue => Color::BLUE,
    };

    building_entity.with_children(|builder| {
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
            transform: Transform::from_translation(Vec3::new(0.0, -40.0, 1.0)),
            ..Default::default()
        });
    });
}

fn spawn_ghost_building(
    commands: &mut Commands,
    team: Team,
    x: f32,
    y: f32,
    sprite: Handle<Image>,
) {
    commands.spawn((
        BuildingGhost {
            placement_valid: true,
            team,
        },
        SpriteBundle {
            texture: sprite,
            transform: Transform::from_xyz(x, y, 0.),
            sprite: Sprite {
                color: Color::rgba(0.5, 1.0, 0.5, 0.5),
                ..Default::default()
            },
            ..Default::default()
        },
        Collider::cuboid(32.0, 32.0),
        Sensor,
        ActiveCollisionTypes::all(), // TODO: Optimize later.
        // TODO: Set collision groups for further optimisation.
        ActiveEvents::COLLISION_EVENTS,
    ));
}

fn add_buildings(mut commands: Commands, asset_server: Res<AssetServer>) {
    let building_sprite = asset_server.load("prototype-building.png");

    spawn_building(
        &mut commands,
        Team::TeamRed,
        32.0 * -4.0,
        0.,
        building_sprite.clone(),
    );
    spawn_building(
        &mut commands,
        Team::TeamBlue,
        32.0 * 4.0,
        0.,
        building_sprite.clone(),
    );
}

fn add_units(
    mut commands: Commands,
    asset_server: Res<AssetServer>,
    waypoint_map: Res<WaypointMap>,
) {
    spawn_unit(
        &mut commands,
        Team::TeamRed,
        asset_server.load("prototype-unit.png"),
        32. * 5.,
        0.,
        &waypoint_map,
        None,
    );
}

fn spawn_units(
    mut commands: Commands,
    asset_server: Res<AssetServer>,
    waypoint_map: Res<WaypointMap>,
    time: Res<Time>,
    mut query: Query<(&mut UnitSpawner, &Transform, &Unit)>,
) {
    for (mut unit_spawner, transform, unit) in query.iter_mut() {
        if unit_spawner.time_left > 0. {
            unit_spawner.time_left -= time.delta_seconds()
        } else {
            let waypoint_id = match unit.team {
                Team::TeamRed => "FirstRed",
                Team::TeamBlue => "FirstBlue",
            };

            spawn_unit(
                &mut commands,
                unit.team,
                asset_server.load("prototype-unit.png"),
                transform.translation.x,
                transform.translation.y,
                &waypoint_map,
                Some(waypoint_id.to_string()),
            );
            unit_spawner.time_left = unit_spawner.spawn_time
        }
    }
}

fn go_to_next_waypoint(
    mut commands: Commands,
    time: Res<Time>,
    mut query: Query<
        (
            Entity,
            &MovementSpeed,
            &mut Transform,
            &mut WaypointFollower,
        ),
        (With<Unit>, Without<Waypoint>),
    >,
    waypoint_transform_query: Query<&Transform, (With<Waypoint>, Without<Unit>)>,
    waypoint_waypoint_query: Query<&Waypoint>,
) {
    for (entity, movement_speed, mut transform, mut follower) in query.iter_mut() {
        if let Ok(waypoint_transform) = waypoint_transform_query.get(follower.waypoint) {
            let direction = waypoint_transform.translation - transform.translation;
            let distance = direction.length();

            if distance < 32.0 {
                // Check if there is a next waypoint.
                if let Ok(waypoint) = waypoint_waypoint_query.get(follower.waypoint) {
                    if let Some(next_waypoint) = waypoint.next_waypoint {
                        follower.waypoint = next_waypoint;
                    }
                } else {
                    // If there is no next waypoint, remove the waypoint follower.
                    commands.entity(entity).remove::<WaypointFollower>();
                }
            } else {
                // Move towards the waypoint.
                let move_direction = direction.normalize();
                transform.translation += move_direction * movement_speed.0 * time.delta_seconds();
            }
        }
    }
}

fn detect_targets_system(
    mut collisions: EventReader<CollisionEvent>,
    mut unit_attack_query: Query<(Entity, &Unit, &mut Attack)>,
    unit_defend_query: Query<(Entity, &Unit, &Health)>,
) {
    let check_set_target =
        |entity1: &Entity,
         entity2: &Entity,
         unit_attack_query: &mut Query<(Entity, &Unit, &mut Attack)>| {
            // Get the units.
            let maybe_unit1 = unit_attack_query.get_mut(*entity1);
            let maybe_unit2 = unit_defend_query.get(*entity2);

            if let (Ok((_, unit1, mut unit1_attack)), Ok((unit2_entity, unit2, _))) =
                (maybe_unit1, maybe_unit2)
            {
                if unit1.team != unit2.team && unit1_attack.target.is_none() {
                    unit1_attack.target = Some(unit2_entity);
                }
            }
        };

    let check_remove_target =
        |entity1: &Entity,
         entity2: &Entity,
         unit_attack_query: &mut Query<(Entity, &Unit, &mut Attack)>| {
            let maybe_unit1 = unit_attack_query.get_mut(*entity1);

            if let Ok((_, _, mut unit1_attack)) = maybe_unit1 {
                if let Some(target) = unit1_attack.target {
                    if target == *entity2 {
                        unit1_attack.target = None;
                    }
                }
            }
        };

    for collision in collisions.read() {
        match collision {
            CollisionEvent::Started(entity1, entity2, _) => {
                // Check in both directions, as both units might have the attack component.
                check_set_target(entity1, entity2, &mut unit_attack_query);
                check_set_target(entity2, entity1, &mut unit_attack_query);
            }
            CollisionEvent::Stopped(entity1, entity2, _) => {
                // Check in both directions, as both units might have the attack component.
                check_remove_target(entity1, entity2, &mut unit_attack_query);
                check_remove_target(entity2, entity1, &mut unit_attack_query);
            }
        }
    }
}

fn attack_system(
    mut attack_query: Query<&mut Attack, With<Unit>>,
    mut defender_query: Query<&mut Health, With<Unit>>,
    time: Res<Time>,
) {
    for mut attack in attack_query.iter_mut() {
        // The attacker must have a target to be considered.
        if let Some(target) = attack.target {
            // Don't attack, if attack cooldown hasn't finished.
            if attack.time_till_next_attack > 0. {
                attack.time_till_next_attack -= time.delta_seconds();
                return;
            }
            // Get the defender and subtract health.
            if let Ok(mut defender) = defender_query.get_mut(target) {
                // TODO: Make a more intricate damage calculation.
                defender.health -= attack.damage;
                info!("{:?} damage taken!", attack.damage);
                // Reset attack cooldown.
                attack.time_till_next_attack = 1. / attack.attack_speed;
            }
        }
    }
}

/// Should run after attack_system.
fn check_death(mut commands: Commands, query: Query<(Entity, &Health), With<Unit>>) {
    for (entity, health) in query.iter() {
        if health.health <= 0. {
            commands.entity(entity).despawn_recursive();
            info!("{:?} died as health was depleted!", entity);
        }
    }
}

fn get_mouse_position(
    window_query: Query<&Window>,
    camera_query: Query<(&Camera, &GlobalTransform)>,
    mut mouse_position: ResMut<MousePosition>,
) {
    if let Ok(window) = window_query.get_single() {
        if let Ok((camera, camera_transform)) = camera_query.get_single() {
            if let Some(mouse_world_pos) = window
                .cursor_position()
                .and_then(|cursor| camera.viewport_to_world_2d(camera_transform, cursor))
                .map(|ray| (ray.x, ray.y))
            {
                mouse_position.x = mouse_world_pos.0;
                mouse_position.y = mouse_world_pos.1;
            }
        }
    }
}

fn building_system(
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
        ghost_transform.translation = Vec3::new(mouse_position.x, mouse_position.y, 1.);
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
) {
    // Contains ghosts and if their placement is valid.
    let mut ghost_collisions: HashMap<Entity, bool> = HashMap::new();

    for event in collisions.read() {
        match event {
            CollisionEvent::Started(entity1, entity2, _) => {
                let (ghost, building) = if ghost_query.get(*entity1).is_ok() {
                    (entity1, entity2)
                } else if ghost_query.get(*entity2).is_ok() {
                    (entity2, entity1)
                } else {
                    continue;
                };

                if building_query.get(*building).is_ok() {
                    ghost_collisions.insert(*ghost, false);
                }
            }
            CollisionEvent::Stopped(entity1, entity2, _) => {
                let (ghost, building) = if ghost_query.get(*entity1).is_ok() {
                    (entity1, entity2)
                } else if ghost_query.get(*entity2).is_ok() {
                    (entity2, entity1)
                } else {
                    continue;
                };

                if building_query.get(*building).is_ok() {
                    // When collision stops, consider re-validating placement.
                    // This might be too optimistic if there are multiple buildings overlapping.
                    ghost_collisions.entry(*ghost).or_insert(true);
                }
            }
        }
    }

    // Update ghosts based on collected collision states.
    for (entity, placement_valid) in ghost_collisions.iter() {
        if let Ok((_, mut ghost_building, mut ghost_sprite)) = ghost_query.get_mut(*entity) {
            ghost_building.placement_valid = *placement_valid;
            ghost_sprite.color = if *placement_valid {
                Color::rgba(0.5, 1.0, 0.5, 0.5)
            } else {
                Color::rgba(1.0, 0.5, 0.5, 0.5)
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
