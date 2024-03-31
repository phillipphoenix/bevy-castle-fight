use bevy::prelude::*;
use bevy_rapier2d::prelude::*;

//mod
pub mod health;
pub mod movement;
pub mod vision;
pub mod waypoints;
pub mod teams;
mod attack;
mod building_spawning;
mod camera;
mod castle_fight_ldtk;
mod resources;
mod unit_spawning;
mod buildings;
mod grid_traits;
mod units;
mod systems;

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
use systems::*;

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
        //State Transitions
        .add_systems(OnEnter(SimulationState::Paused),pause_simulation)
        .add_systems(OnEnter(SimulationState::Running),unpause_simulation)
        // Systems
        .add_systems(Update, toggle_simulation.run_if(in_state(AppState::Game)))
        ;
        
    }
}

// States - Related to the gameplay

#[derive(States, Debug, Clone, PartialEq, Eq, Hash, Default)]
pub enum SimulationState {
    #[default]
    Running,
    Paused,
}