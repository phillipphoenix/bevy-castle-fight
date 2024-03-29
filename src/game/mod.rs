use bevy::prelude::*;
use bevy_rapier2d::prelude::*;

//mod
mod attack;
mod building_spawning;
mod camera;
mod castle_fight_ldtk;
pub mod health;
pub mod movement;
mod resources;
mod unit_spawning;
pub mod vision;
pub mod waypoints;
mod buildings;
mod grid_traits;
pub mod teams;
mod units;

// use
use attack::AttackPlugin;
use building_spawning::BuildingSpawningPlugin;
use camera::CameraPlugin;
use castle_fight_ldtk::CastleFightLdtkPlugin;
use health::HealthPlugin;
use movement::MovementPlugin;
use resources::ResourcesPlugin;
use unit_spawning::UnitSpawningPlugin;
use vision::VisionPlugin;
use waypoints::WaypointPlugin;

pub struct GamePlugin;

impl Plugin for GamePlugin {
    fn build(&self, app: &mut App) {
        app.add_plugins((
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
        // Physics plugins.
        .add_plugins((
            RapierPhysicsPlugin::<NoUserData>::default(),
            RapierDebugRenderPlugin::default(),
        ));
        
    }
}
