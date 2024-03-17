mod waypoints;

use bevy::{
    prelude::*,
    render::{settings::WgpuSettings, RenderPlugin},
    utils::HashMap,
};
use waypoints::*;

#[derive(Component)]
struct Unit;

#[derive(Component)]
struct MovementSpeed(f32);

fn main() {
    App::new()
        .add_plugins(DefaultPlugins.set(RenderPlugin {
            render_creation: bevy::render::settings::RenderCreation::Automatic(WgpuSettings {
                // This is necessary to remove infinite errors on Windows machines with AMD GPUs.
                // Might not be required in the future - (2024-03-14).
                backends: Some(bevy::render::settings::Backends::VULKAN),
                ..Default::default()
            }),
            synchronous_pipeline_compilation: true,
        }))
        .insert_resource(WaypointMap {
            all_waypoints: HashMap::default(),
        })
        .add_systems(Startup, add_camera)
        .add_systems(Startup, add_waypoints.before(add_units))
        .add_systems(Startup, add_units)
        .add_systems(Update, go_to_next_waypoint)
        .run();
}

fn add_camera(mut commands: Commands) {
    commands.spawn(Camera2dBundle::default());
}

fn add_units(
    mut commands: Commands,
    asset_server: Res<AssetServer>,
    waypoint_map: Res<WaypointMap>,
) {
    let first_waypoint = waypoint_map.all_waypoints.get("First").unwrap();

    commands.spawn((
        Unit,
        MovementSpeed(64.),
        SpriteBundle {
            texture: asset_server.load("prototype-unit.png"),
            ..Default::default()
        },
        WaypointFollower {
            waypoint: *first_waypoint,
        },
    ));
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
        if let Some(waypoint_transform) = waypoint_transform_query.get(follower.waypoint).ok() {
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
