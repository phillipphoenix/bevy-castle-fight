use crate::game::health::Health;
use crate::game::teams::Team;
use crate::game::unit_spawning::UnitSpawner;
use crate::game::vision::Visible;
use bevy::prelude::*;
use bevy_rapier2d::prelude::*;

// --- Components ---

#[derive(Component, Default)]
pub struct Building;

#[derive(Component, Default)]
pub struct Castle;

#[derive(Component)]
pub struct BuildingGhost {
    pub placement_valid: bool,
    pub team: Team,
}

// --- Helper functions ---

/// Helper function to spawn a building. This is not a system.
pub fn spawn_building(commands: &mut Commands, team: Team, x: f32, y: f32, sprite: Handle<Image>) {
    let mut building_entity = commands.spawn((
        team,
        Building,
        Visible,
        Health {
            health: 10,
            max_health: 10,
        },
        SpriteBundle {
            texture: sprite,
            transform: Transform::from_xyz(x, y, 10.),
            ..Default::default()
        },
        UnitSpawner {
            spawn_time: 5.0,
            time_left: 5.0,
        },
        RigidBody::KinematicPositionBased,
        Collider::cuboid(32.0, 32.0), // Actual collider matching sprite size.
    ));

    let text_color = team.get_color();

    building_entity.with_children(|builder| {
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
            transform: Transform::from_translation(Vec3::new(0.0, -40.0, 1.0)),
            ..Default::default()
        });
    });
}

/// Helper function to spawn a ghost building (for building new buildings). This is not a system.
pub fn spawn_ghost_building(
    commands: &mut Commands,
    team: Team,
    x: f32,
    y: f32,
    sprite: Handle<Image>,
) {
    commands.spawn((
        BuildingGhost {
            placement_valid: true,
            team,
        },
        SpriteBundle {
            texture: sprite,
            transform: Transform::from_xyz(x, y, 10.1),
            sprite: Sprite {
                color: Color::rgba(0.5, 1.0, 0.5, 0.7),
                ..Default::default()
            },
            ..Default::default()
        },
        Collider::cuboid(31.0, 31.0),
        Sensor,
        CollisionGroups::new(Group::GROUP_3, Group::GROUP_1),
        ActiveCollisionTypes::all(), // TODO: Optimize later.
        ActiveEvents::COLLISION_EVENTS,
    ));
}
