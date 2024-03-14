use bevy::{
    prelude::*,
    render::{settings::WgpuSettings, RenderPlugin},
};

#[derive(Component)]
struct Unit;

#[derive(Component)]
struct Waypoint;

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
        .add_systems(Startup, add_camera)
        .add_systems(Startup, add_waypoints)
        .add_systems(Startup, add_units)
        .run();
}

fn add_camera(mut commands: Commands) {
    commands.spawn(Camera2dBundle::default());
}

fn add_units(mut commands: Commands, asset_server: Res<AssetServer>) {
    commands.spawn((
        Unit,
        SpriteBundle {
            texture: asset_server.load("prototype-unit.png"),
            ..Default::default()
        },
    ));
}

fn add_waypoints(mut commands: Commands, asset_server: Res<AssetServer>) {
    commands.spawn((
        Waypoint,
        SpriteBundle {
            transform: Transform::from_xyz(32., 0., 0.),
            texture: asset_server.load("prototype-flag.png"),
            ..Default::default()
        },
    ));
}
