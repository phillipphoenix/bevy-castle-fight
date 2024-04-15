use bevy::prelude::*;
use bevy_ecs_ldtk::prelude::*;
use bevy_rapier2d::prelude::*;

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

use crate::game::ui::UiPlugin;
use crate::AppState;

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
mod spawning;
mod systems;
pub mod teams;
mod ui;
mod unit_spawning;
mod units;
pub mod vision;
pub mod waypoints;

pub struct GamePlugin;

impl Plugin for GamePlugin {
    fn build(&self, app: &mut App) {
        app.add_plugins((
            CastleFightLdtkPlugin {
                state: AppState::Game,
            },
            ResourcesPlugin,
            CameraPlugin,
            UiPlugin {
                state: AppState::Game,
            },
            WaypointPlugin {
                state: AppState::Game,
            },
            VisionPlugin {
                state: AppState::Game,
            },
            AttackPlugin {
                state: AppState::Game,
            },
            HealthPlugin {
                state: AppState::Game,
            },
            MovementPlugin {
                state: AppState::Game,
            },
            BuildingSpawningPlugin {
                state: AppState::Game,
            },
            UnitSpawningPlugin {
                state: AppState::Game,
            },
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
        .add_systems(OnEnter(AppState::Game), setup)
        .add_systems(OnExit(AppState::Game), cleanup_after_game);
    }
}

// States - Related to the gameplay

#[derive(States, Debug, Clone, PartialEq, Eq, Hash, Default)]
pub enum SimulationState {
    #[default]
    Running,
    Paused,
}

// --- Components ---

#[derive(Component, Default)]
pub struct InGameTag;

// --- Systems ---

fn setup(mut commands: Commands, asset_server: Res<AssetServer>) {
    commands.spawn((
        InGameTag,
        LdtkWorldBundle {
            ldtk_handle: asset_server.load("maps/map-0.ldtk"),
            ..Default::default()
        },
    ));
}

fn cleanup_after_game(mut commands: Commands, query: Query<Entity, With<InGameTag>>) {
    for game_entity in query.iter() {
        commands.entity(game_entity).despawn_recursive();
    }
}
