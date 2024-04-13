use crate::game::attack::AttackStats;
use crate::game::health::Health;
use crate::game::movement::{MovementSpeed, OpponentFollower, WaypointFollower};
use crate::game::teams::Team;
use crate::game::vision::{InVision, Visible, VisionRange};
use crate::game::waypoints::WaypointMap;
use crate::game::InGameTag;
use bevy::prelude::*;
use bevy_rapier2d::prelude::*;

// --- Components ---
#[derive(Component)]
struct Unit;

// --- Helper functions ---

/// Function to help spawn a unit.
pub fn spawn_unit(
    commands: &mut Commands,
    team: Team,
    sprite: Handle<Image>,
    x: f32,
    y: f32,
    waypoint_map: &Res<WaypointMap>,
) {
    let mut unit_entity = commands.spawn((
        InGameTag,
        team,
        Unit,
        Visible,
        VisionRange(32. * 4.),
        InVision {
            friendlies: vec![],
            enemies: vec![],
        },
        OpponentFollower,
        Health {
            health: 5,
            max_health: 5,
        },
        AttackStats {
            attack_speed: 2.,
            attack_range: 16. + 32. * 0., // Melee.
            damage: 1,
            time_till_next_attack: 0.,
        },
        MovementSpeed(32. * 3.),
        SpriteBundle {
            transform: Transform::from_xyz(x, y, 10.),
            texture: sprite,
            ..Default::default()
        },
        RigidBody::KinematicPositionBased,
        Collider::ball(20.), // Actual collider matching sprite size.
    ));
    unit_entity.insert(Name::new("Unit"));

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
