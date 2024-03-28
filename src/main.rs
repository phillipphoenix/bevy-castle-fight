use bevy::input::common_conditions::input_just_pressed;
use bevy::{
    prelude::*,
    render::{settings::WgpuSettings, RenderPlugin},
};
use bevy_ecs_ldtk::prelude::*;
use bevy_rapier2d::prelude::*;

use bevy_castle_fight::attack::AttackPlugin;
use bevy_castle_fight::building_spawning::BuildingSpawningPlugin;
use bevy_castle_fight::camera::CameraPlugin;
use bevy_castle_fight::castle_fight_ldtk::CastleFightLdtkPlugin;
use bevy_castle_fight::health::HealthPlugin;
use bevy_castle_fight::inspector_plugin::InspectorPlugin;
use bevy_castle_fight::movement::MovementPlugin;
use bevy_castle_fight::resources::ResourcesPlugin;
use bevy_castle_fight::unit_spawning::UnitSpawningPlugin;
use bevy_castle_fight::vision::VisionPlugin;
use bevy_castle_fight::waypoints::WaypointPlugin;

fn main() {
    App::new()
        .add_plugins((DefaultPlugins.set(RenderPlugin {
            render_creation: bevy::render::settings::RenderCreation::Automatic(WgpuSettings {
                // This is necessary to remove infinite errors on Windows machines with AMD GPUs.
                // Might not be required in the future - (2024-03-14).
                backends: Some(bevy::render::settings::Backends::VULKAN),
                ..Default::default()
            }),
            synchronous_pipeline_compilation: false,
        }),))
        // Debug plugins.
        .add_plugins(InspectorPlugin)
        // Physics plugins.
        .add_plugins((
            RapierPhysicsPlugin::<NoUserData>::default(),
            RapierDebugRenderPlugin::default(),
        ))
        // Third party plugins.
        .add_plugins(LdtkPlugin)
        // Castle Fight plugins.
        .add_plugins((
            CastleFightLdtkPlugin,
            ResourcesPlugin,
            CameraPlugin,
            WaypointPlugin,
            VisionPlugin,
            AttackPlugin,
            HealthPlugin,
            MovementPlugin,
            BuildingSpawningPlugin,
            UnitSpawningPlugin,
        ))
        // Third party resources
        .insert_resource(LevelSelection::index(0))
        // Systems.
        .add_systems(Startup, setup)
        .add_systems(
            Update,
            toggle_pause.run_if(input_just_pressed(KeyCode::Space)),
        )
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
