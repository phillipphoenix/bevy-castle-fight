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

#[derive(Clone, Copy, PartialEq)]
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

#[derive(Component)]
struct Unit {
    team: Team,
}

#[derive(Component)]
struct MovementSpeed(f32);

#[derive(Component)]
struct Health {
    max_health: f32,
    health: f32,
}

#[derive(Component)]
struct Attack {
    target: Option<Entity>,
    damage: f32,
    attack_speed: f32,
    time_till_next_attack: f32,
}

#[derive(Component)]
struct Building;

#[derive(Component)]
struct UnitSpawner {
    spawn_time: f32,
    time_left: f32,
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
        ))
        .insert_resource(WaypointMap {
            all_waypoints: HashMap::default(),
        })
        .add_systems(Startup, (add_camera,))
        .add_systems(Startup, add_waypoints.before(add_units))
        // .add_systems(Startup, add_units)
        .add_systems(Startup, add_buildings)
        .add_systems(
            Update,
            (go_to_next_waypoint, spawn_units, detect_targets_system),
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
        MovementSpeed(64.),
        SpriteBundle {
            transform: Transform::from_xyz(x, y, 0.),
            texture: sprite,
            ..Default::default()
        },
        Collider::ball(16.0),
        Sensor,
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

fn spawn_building(
    commands: &mut Commands,
    team: Team,
    x: f32,
    y: f32,
    asset_server: &Res<AssetServer>,
) {
    let mut building_entity = commands.spawn((
        Unit { team },
        SpriteBundle {
            texture: asset_server.load("prototype-building.png"),
            transform: Transform::from_xyz(x, y, 0.),
            ..Default::default()
        },
        UnitSpawner {
            spawn_time: 5.0,
            time_left: 5.0,
        },
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

fn add_buildings(mut commands: Commands, asset_server: Res<AssetServer>) {
    spawn_building(&mut commands, Team::TeamRed, 32.0 * -4.0, 0., &asset_server);
    spawn_building(&mut commands, Team::TeamBlue, 32.0 * 4.0, 0., &asset_server);
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
        0.,
        0.,
        &waypoint_map,
        Some("First".to_string()),
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
            spawn_unit(
                &mut commands,
                unit.team,
                asset_server.load("prototype-unit.png"),
                transform.translation.x,
                transform.translation.y,
                &waypoint_map,
                Some("First".to_string()),
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
    for collision in collisions.read() {
        match collision {
            CollisionEvent::Started(entity1, entity2, _) => {
                // Get the units.
                let maybe_unit1 = unit_attack_query.get_mut(*entity1);
                let maybe_unit2 = unit_defend_query.get(*entity2);

                if let (Ok((_, unit1, mut unit1_attack)), Ok((unit2_entity, unit2, _))) =
                    (maybe_unit1, maybe_unit2)
                {
                    if unit1.team != unit2.team && unit1_attack.target.is_none() {
                        unit1_attack.target = Some(unit2_entity);
                        println!("Target found!")
                    }
                }
            }
            CollisionEvent::Stopped(entity1, entity2, _) => {
                let maybe_unit1 = unit_attack_query.get_mut(*entity1);

                if let Ok((_, _, mut unit1_attack)) = maybe_unit1 {
                    if let Some(target) = unit1_attack.target {
                        if target == *entity2 {
                            unit1_attack.target = None;
                        }
                    }
                }
            }
        }
    }
}
