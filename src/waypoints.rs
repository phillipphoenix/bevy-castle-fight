use bevy::{
    asset::AssetServer,
    ecs::{
        component::Component,
        entity::Entity,
        system::{Commands, Res, ResMut, Resource},
    },
    sprite::SpriteBundle,
    transform::components::Transform,
    utils::HashMap,
};

#[derive(Component)]
pub struct Waypoint {
    pub id: Option<String>,
    pub next_waypoint: Option<Entity>,
}

#[derive(Component)]
pub struct WaypointFollower {
    pub waypoint: Entity,
}

#[derive(Resource)]
pub struct WaypointMap {
    pub all_waypoints: HashMap<String, Entity>,
}

pub fn add_waypoints(
    mut commands: Commands,
    asset_server: Res<AssetServer>,
    mut waypoint_map: ResMut<WaypointMap>,
) {
    let mut spawn_waypoint = |commands: &mut Commands,
                              asset_server: &Res<AssetServer>,
                              id: Option<String>,
                              x: f32,
                              y: f32,
                              next_waypoint: Option<Entity>|
     -> Entity {
        let entity = commands
            .spawn((
                Waypoint {
                    id: id.clone(),
                    next_waypoint,
                },
                SpriteBundle {
                    transform: Transform::from_xyz(x, y, 0.),
                    texture: asset_server.load("prototype-flag.png"),
                    ..Default::default()
                },
            ))
            .id();

        if let Some(ref id_str) = id {
            waypoint_map.all_waypoints.insert(id_str.clone(), entity);
        }

        entity
    };

    // --- RED team waypoints
    let waypoint3_red = spawn_waypoint(
        &mut commands,
        &asset_server,
        None,
        32. * -5.,
        32. * 5.,
        None,
    );
    let waypoint2_red = spawn_waypoint(
        &mut commands,
        &asset_server,
        None,
        0.,
        32. * 5.,
        Some(waypoint3_red),
    );
    let waypoint1_red = spawn_waypoint(
        &mut commands,
        &asset_server,
        Some("FirstRed".to_string()),
        0.,
        0.,
        Some(waypoint2_red),
    );

    // Make the waypoints loop.
    commands.entity(waypoint3_red).insert(Waypoint {
        id: None,
        next_waypoint: Some(waypoint1_red),
    });

    // --- BLUE team waypoints
    let waypoint3_blue =
        spawn_waypoint(&mut commands, &asset_server, None, 32. * 5., 32. * 5., None);
    let waypoint2_blue = spawn_waypoint(
        &mut commands,
        &asset_server,
        None,
        0.,
        32. * 5.,
        Some(waypoint3_blue),
    );
    let waypoint1_blue = spawn_waypoint(
        &mut commands,
        &asset_server,
        Some("FirstBlue".to_string()),
        0.,
        0.,
        Some(waypoint2_blue),
    );

    // Make the waypoints loop.
    commands.entity(waypoint3_blue).insert(Waypoint {
        id: None,
        next_waypoint: Some(waypoint1_blue),
    });
}
