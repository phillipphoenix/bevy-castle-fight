mod building_plugin;
mod buildings;
mod camera_plugin;
mod common_components;
mod grid_traits;
mod inspector_plugin;
mod unit_plugin;
mod units;
mod waypoint_plugin;

use crate::camera_plugin::CameraPlugin;
use crate::common_components::{Health, TeamEntity};
use crate::inspector_plugin::InspectorPlugin;
use bevy::input::common_conditions::input_just_pressed;
use bevy::{
    prelude::*,
    render::{settings::WgpuSettings, RenderPlugin},
};
use bevy_ecs_ldtk::prelude::*;
use bevy_rapier2d::prelude::*;
use building_plugin::BuildingPlugin;
use unit_plugin::UnitPlugin;
use waypoint_plugin::*;

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
                synchronous_pipeline_compilation: false,
            }),
            RapierPhysicsPlugin::<NoUserData>::default(),
            RapierDebugRenderPlugin::default(),
            InspectorPlugin,
        ))
        .insert_resource(MousePosition { x: 0., y: 0. })
        .insert_resource(LevelSelection::index(0))
        .add_plugins(CameraPlugin)
        .add_plugins((WaypointPlugin, UnitPlugin, BuildingPlugin))
        .add_plugins(LdtkPlugin)
        .add_systems(Startup, setup)
        .add_systems(
            Update,
            toggle_pause.run_if(input_just_pressed(KeyCode::Space)),
        )
        .add_systems(Update, check_death)
        .add_systems(Update, get_mouse_position)
        .run();
}

fn toggle_pause(mut time: ResMut<Time<Virtual>>) {
    if time.is_paused() {
        time.unpause();
    } else {
        time.pause();
    }
}

fn setup(mut commands: Commands, asset_server: Res<AssetServer>) {
    commands.spawn(LdtkWorldBundle {
        ldtk_handle: asset_server.load("maps/map-0.ldtk"),
        ..Default::default()
    });
}

/// Should run after attack_system.
fn check_death(mut commands: Commands, query: Query<(Entity, &Health), With<TeamEntity>>) {
    for (entity, health) in query.iter() {
        if health.health <= 0 {
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
