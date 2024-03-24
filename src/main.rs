mod building_plugin;
mod buildings;
mod common_components;
mod unit_plugin;
mod units;
mod waypoints;

use bevy::input::common_conditions::{input_just_pressed, input_toggle_active};
use bevy::{
    prelude::*,
    render::{settings::WgpuSettings, RenderPlugin},
    utils::HashMap,
};
use bevy_inspector_egui::quick::WorldInspectorPlugin;
use bevy_rapier2d::prelude::*;

use waypoints::*;

use crate::common_components::{Health, TeamEntity};
use building_plugin::BuildingPlugin;
use unit_plugin::UnitPlugin;

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
            WorldInspectorPlugin::default().run_if(input_toggle_active(true, KeyCode::Escape)),
        ))
        .insert_resource(WaypointMap {
            all_waypoints: HashMap::default(),
        })
        .insert_resource(MousePosition { x: 0., y: 0. })
        .add_plugins((UnitPlugin, BuildingPlugin))
        .add_systems(Startup, add_camera)
        .add_systems(Startup, add_waypoints)
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

fn add_camera(mut commands: Commands) {
    commands.spawn(Camera2dBundle::default());
}

/// Should run after attack_system.
fn check_death(mut commands: Commands, query: Query<(Entity, &Health), With<TeamEntity>>) {
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
