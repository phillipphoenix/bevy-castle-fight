use crate::common_components::*;
use crate::waypoints::{WaypointFollower, WaypointMap};
use bevy::prelude::*;
use bevy_rapier2d::prelude::*;

#[derive(Component)]
struct Unit;

#[derive(Component)]
pub struct MovementSpeed(pub f32);

#[derive(Component, Debug)]
pub struct AttackStats {
    pub damage: f32,
    pub attack_speed: f32,
    /// Used to check if the attack target is within striking range.
    pub attack_range: f32,
    pub time_till_next_attack: f32,
}

#[derive(Component)]
pub struct OpponentFollower;

#[derive(Component)]
pub struct MoveTarget(pub Entity);

/// One-time use points that are used to move to.
#[derive(Component)]
pub struct MoveToPoint(pub Vec2);

pub fn spawn_unit(
    commands: &mut Commands,
    team: Team,
    sprite: Handle<Image>,
    x: f32,
    y: f32,
    waypoint_map: &Res<WaypointMap>,
    waypoint_id: Option<String>,
) {
    let mut unit_entity = commands.spawn((
        TeamEntity { team },
        Unit,
        OpponentFollower,
        Health {
            health: 5.,
            max_health: 5.,
        },
        AttackStats {
            attack_speed: 2.,
            attack_range: 16. + 32. * 0., // Melee.
            damage: 1.,
            time_till_next_attack: 0.,
        },
        MovementSpeed(32. * 3.),
        SpriteBundle {
            transform: Transform::from_xyz(x, y, 0.1),
            texture: sprite,
            ..Default::default()
        },
        RigidBody::KinematicPositionBased,
        Collider::ball(32. * 3.), // Vision sensor.
        Sensor,
        CollisionGroups::new(Group::GROUP_2, Group::GROUP_1),
        ActiveCollisionTypes::all(), // TODO: Optimize later.
        ActiveEvents::COLLISION_EVENTS,
    ));

    let text_color = match team {
        Team::TeamRed => Color::RED,
        Team::TeamBlue => Color::BLUE,
    };

    unit_entity.with_children(|builder| {
        builder.spawn((
            Collider::ball(20.), // Actual collider matching sprite size.
            CollisionGroups::new(Group::GROUP_1, Group::GROUP_2),
        ));
    });

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

    if let Some(ref waypoint_id_str) = waypoint_id {
        let waypoint = waypoint_map.all_waypoints.get(waypoint_id_str).unwrap();

        unit_entity.insert(WaypointFollower {
            waypoint: *waypoint,
        });
    }
}
