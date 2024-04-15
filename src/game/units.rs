use bevy::prelude::*;
use bevy_rapier2d::prelude::*;

use crate::game::movement::WaypointFollower;
use crate::game::spawning::add_blueprint_components;
use crate::game::teams::Team;
use crate::game::waypoints::WaypointMap;
use crate::game::InGameTag;
use crate::load_game::load_factions::UnitBlueprint;
use crate::resources::PlayerSettings;

// --- Components ---
#[derive(Component)]
struct Unit;

// --- Helper functions ---

/// Function to help spawn a unit.
pub fn spawn_unit(
    commands: &mut Commands,
    team: Team,
    unit_blueprint: UnitBlueprint,
    x: f32,
    y: f32,
    waypoint_map: &Res<WaypointMap>,
    player_settings: &Res<PlayerSettings>,
) {
    let mut unit_entity = commands.spawn((
        InGameTag,
        team,
        Unit,
        SpriteBundle {
            transform: Transform::from_xyz(x, y, 10.),
            texture: unit_blueprint.sprite.clone(),
            ..Default::default()
        },
        RigidBody::KinematicPositionBased,
        Collider::ball(20.), // Actual collider matching sprite size.
    ));
    unit_entity.insert(Name::new(format!(
        "Unit: {} - Team: {}",
        unit_blueprint.name, team
    )));

    // Insert components from the blueprint.
    add_blueprint_components(
        &mut unit_entity,
        &unit_blueprint.components,
        player_settings,
    );

    let text_color = team.get_color();

    unit_entity.with_children(|builder| {
        builder.spawn(Text2dBundle {
            text: Text {
                sections: vec![TextSection::new(
                    team.to_string(),
                    TextStyle {
                        color: text_color,
                        ..Default::default()
                    },
                )],
                ..Default::default()
            },
            transform: Transform::from_translation(Vec3::new(0.0, -22.0, 1.0)),
            ..Default::default()
        });
    });

    if let Some(start_waypoint) = waypoint_map.get_closest_start_waypoint(Vec2::new(x, y), team) {
        unit_entity.insert(WaypointFollower {
            waypoint: start_waypoint,
        });
    }
}

// --- Systems ---
