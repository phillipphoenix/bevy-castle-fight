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

    let waypoint3 = spawn_waypoint(&mut commands, &asset_server, None, 32. * 5., 32. * 5., None);
    let waypoint2 = spawn_waypoint(
        &mut commands,
        &asset_server,
        None,
        32.,
        32. * 5.,
        Some(waypoint3),
    );
    let _ = spawn_waypoint(
        &mut commands,
        &asset_server,
        Some("First".to_string()),
        32.,
        0.,
        Some(waypoint2),
    );
}
