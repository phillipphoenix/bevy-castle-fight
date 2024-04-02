use bevy::prelude::*;
use bevy_ecs_ldtk::prelude::*;
use bevy_rapier2d::prelude::*;

//mod
mod attack;
mod building_spawning;
mod buildings;
mod camera;
mod castle_fight_ldtk;
mod grid_traits;
pub mod health;
pub mod movement;
mod resources;
mod systems;
pub mod teams;
mod unit_spawning;
mod units;
pub mod vision;
pub mod waypoints;

// use
use attack::AttackPlugin;
use building_spawning::BuildingSpawningPlugin;
use camera::CameraPlugin;
use castle_fight_ldtk::CastleFightLdtkPlugin;
use health::HealthPlugin;
use movement::MovementPlugin;
use resources::ResourcesPlugin;
use systems::*;
use unit_spawning::UnitSpawningPlugin;
use vision::VisionPlugin;
use waypoints::WaypointPlugin;

use crate::AppState;

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
        ))
        // States
        .init_state::<SimulationState>()
        // Third party plugins.
        .add_plugins(LdtkPlugin)
        // Third party resources
        .insert_resource(LevelSelection::index(0))
        //State Transitions
        .add_systems(OnEnter(SimulationState::Paused), pause_simulation)
        .add_systems(OnEnter(SimulationState::Running), unpause_simulation)
        // Systems
        .add_systems(Update, toggle_simulation.run_if(in_state(AppState::Game)))
        .add_systems(Startup, setup);
    }
}

// States - Related to the gameplay

#[derive(States, Debug, Clone, PartialEq, Eq, Hash, Default)]
pub enum SimulationState {
    #[default]
    Running,
    Paused,
}

fn setup(mut commands: Commands, asset_server: Res<AssetServer>) {
    commands.spawn(LdtkWorldBundle {
        ldtk_handle: asset_server.load("maps/map-0.ldtk"),
        ..Default::default()
    });
}
