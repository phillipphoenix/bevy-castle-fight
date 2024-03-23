use crate::common_components::*;
use bevy::prelude::*;
use bevy_rapier2d::prelude::*;

#[derive(Component)]
pub struct Building;

#[derive(Component)]
pub struct BuildingGhost {
    pub placement_valid: bool,
    pub team: Team,
}

#[derive(Component)]
pub struct UnitSpawner {
    pub spawn_time: f32,
    pub time_left: f32,
}

pub fn spawn_building(commands: &mut Commands, team: Team, x: f32, y: f32, sprite: Handle<Image>) {
    let mut building_entity = commands.spawn((
        TeamEntity { team },
        Building,
        Health {
            health: 10.,
            max_health: 10.,
        },
        SpriteBundle {
            texture: sprite,
            transform: Transform::from_xyz(x, y, 0.),
            ..Default::default()
        },
        UnitSpawner {
            spawn_time: 5.0,
            time_left: 5.0,
        },
        Collider::cuboid(32.0, 32.0),
        Sensor,
        ActiveCollisionTypes::all(), // TODO: Optimize later.
        ActiveEvents::COLLISION_EVENTS,
    ));

    let text_color = match team {
        Team::TeamRed => Color::RED,
        Team::TeamBlue => Color::BLUE,
    };

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
            transform: Transform::from_xyz(x, y, 0.),
            sprite: Sprite {
                color: Color::rgba(0.5, 1.0, 0.5, 0.5),
                ..Default::default()
            },
            ..Default::default()
        },
        Collider::cuboid(32.0, 32.0),
        Sensor,
        ActiveCollisionTypes::all(), // TODO: Optimize later.
        // TODO: Set collision groups for further optimisation.
        ActiveEvents::COLLISION_EVENTS,
    ));
}
