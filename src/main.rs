mod waypoints;

use bevy::render::render_resource::{AsBindGroup, ShaderRef};
use bevy::sprite::{Material2d, Material2dPlugin, MaterialMesh2dBundle};
use bevy::{
    prelude::*,
    render::{settings::WgpuSettings, RenderPlugin},
    utils::HashMap,
};
use waypoints::*;

#[derive(Clone, Copy)]
enum Team {
    TeamRed,
    TeamBlue,
}

#[derive(Component)]
struct Unit {
    team: Team,
}

#[derive(Component)]
struct MovementSpeed(f32);

#[derive(Component)]
struct Building;

#[derive(Component)]
struct UnitSpawner {
    spawn_time: f32,
    time_left: f32,
}

#[derive(Asset, TypePath, AsBindGroup, Debug, Clone)]
struct TeamColourMaterial {
    #[uniform(0)]
    colour_base: Vec4,
    #[uniform(1)]
    colour_team: Vec4,
}

impl Material2d for TeamColourMaterial {
    fn fragment_shader() -> ShaderRef {
        "shaders/team_colour.wgsl".into()
    }
}

#[derive(Resource)]
struct TeamMaterials {
    team_red: Handle<TeamColourMaterial>,
    team_blue: Handle<TeamColourMaterial>,
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
            Material2dPlugin::<TeamColourMaterial>::default(),
        ))
        .insert_resource(WaypointMap {
            all_waypoints: HashMap::default(),
        })
        .add_systems(
            Startup,
            (
                setup_materials.before(add_units).before(add_buildings),
                add_camera,
            ),
        )
        .add_systems(Startup, add_waypoints.before(add_units))
        .add_systems(Startup, add_units)
        .add_systems(Startup, add_buildings)
        .add_systems(Update, (go_to_next_waypoint, spawn_units))
        .run();
}

fn add_camera(mut commands: Commands) {
    commands.spawn(Camera2dBundle::default());
}

fn setup_materials(mut commands: Commands, mut materials: ResMut<Assets<TeamColourMaterial>>) {
    let material_red = materials.add(TeamColourMaterial {
        colour_base: Color::WHITE.rgba_to_vec4(),
        colour_team: Color::RED.rgba_to_vec4(),
    });
    let material_blue = materials.add(TeamColourMaterial {
        colour_base: Color::WHITE.rgba_to_vec4(),
        colour_team: Color::BLUE.rgba_to_vec4(),
    });

    commands.insert_resource(TeamMaterials {
        team_red: material_red,
        team_blue: material_blue,
    });
}

fn spawn_unit(
    commands: &mut Commands,
    team: Team,
    team_materials: &Res<TeamMaterials>,
    sprite: Handle<Image>,
    x: f32,
    y: f32,
    waypoint_map: &Res<WaypointMap>,
    waypoint_id: Option<String>,
) {
    let material_handle = match team {
        Team::TeamRed => team_materials.team_red.clone(),
        Team::TeamBlue => team_materials.team_blue.clone(),
    };

    let mut unit_entity = commands.spawn((
        Unit { team },
        MovementSpeed(64.),
        SpriteBundle {
            transform: Transform::from_xyz(x, y, 0.),
            texture: sprite,
            ..Default::default()
        },
    ));

    unit_entity.insert(MaterialMesh2dBundle {
        material: material_handle,
        ..Default::default()
    });

    if let Some(ref waypoint_id_str) = waypoint_id {
        let waypoint = waypoint_map.all_waypoints.get(waypoint_id_str).unwrap();

        unit_entity.insert(WaypointFollower {
            waypoint: *waypoint,
        });
    }
}

fn add_buildings(mut commands: Commands, asset_server: Res<AssetServer>) {
    commands.spawn((
        Unit {
            team: Team::TeamRed,
        },
        SpriteBundle {
            texture: asset_server.load("prototype-building.png"),
            transform: Transform::from_xyz(32.0 * -4.0, 0., 0.),
            ..Default::default()
        },
        UnitSpawner {
            spawn_time: 5.0,
            time_left: 5.0,
        },
    ));
}

fn add_units(
    mut commands: Commands,
    asset_server: Res<AssetServer>,
    waypoint_map: Res<WaypointMap>,
) {
    let first_waypoint = waypoint_map.all_waypoints.get("First").unwrap();

    commands.spawn((
        Unit {
            team: Team::TeamRed,
        },
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

fn spawn_units(
    mut commands: Commands,
    asset_server: Res<AssetServer>,
    team_materials: Res<TeamMaterials>,
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
                &team_materials,
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
