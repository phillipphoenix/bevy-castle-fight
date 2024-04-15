use bevy::prelude::*;
use bevy_rapier2d::prelude::*;

use crate::game::spawning::add_blueprint_components;
use crate::game::teams::Team;
use crate::game::InGameTag;
use crate::load_game::load_factions::BuildingBlueprint;
use crate::resources::PlayerSettings;

// --- Components ---

#[derive(Component, Default)]
pub struct Building;

#[derive(Component, Default)]
pub struct Castle;

#[derive(Component)]
pub struct BuildingGhost {
    pub placement_valid: bool,
    pub team: Team,
    pub building_blueprint: BuildingBlueprint,
}

// --- Helper functions ---

/// Helper function to spawn a building. This is not a system.
pub fn spawn_building(
    commands: &mut Commands,
    team: Team,
    x: f32,
    y: f32,
    building_blueprint: BuildingBlueprint,
    player_settings: &Res<PlayerSettings>,
) {
    let mut building_entity = commands.spawn((
        InGameTag,
        team,
        Building,
        SpriteBundle {
            texture: building_blueprint.sprite.clone(),
            transform: Transform::from_xyz(x, y, 10.),
            ..Default::default()
        },
        RigidBody::KinematicPositionBased,
        Collider::cuboid(32.0, 32.0), // Actual collider matching sprite size.
    ));
    building_entity.insert(Name::new(format!(
        "Building: {} - Team: {}",
        building_blueprint.name, team
    )));

    // Insert components from the blueprint.
    add_blueprint_components(
        &mut building_entity,
        &building_blueprint.components,
        player_settings,
    );

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
    building_blueprint: BuildingBlueprint,
) {
    commands.spawn((
        InGameTag,
        BuildingGhost {
            placement_valid: true,
            team,
            building_blueprint: building_blueprint.clone(),
        },
        SpriteBundle {
            texture: building_blueprint.sprite.clone(),
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
