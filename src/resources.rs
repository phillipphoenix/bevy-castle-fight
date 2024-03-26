use bevy::prelude::*;

/*
Used for loading resources and for systems updating those resources, for instance mouse position.
This is NOT the place to add resources for assets. These should have a separate plugin.
*/

// --- Plugin ---

pub struct ResourcesPlugin;

impl Plugin for ResourcesPlugin {
    fn build(&self, app: &mut App) {
        app.insert_resource(MousePosition { x: 0., y: 0. })
            .add_systems(Update, get_mouse_position);
    }
}

// --- Resources ---

#[derive(Resource)]
pub struct MousePosition {
    pub x: f32,
    pub y: f32,
}

// --- Systems ---

fn get_mouse_position(
    window_query: Query<&Window>,
    camera_query: Query<(&Camera, &GlobalTransform)>,
    mut mouse_position: ResMut<MousePosition>,
) {
    if let Ok(window) = window_query.get_single() {
        if let Ok((camera, camera_transform)) = camera_query.get_single() {
            if let Some(mouse_world_pos) = window
                .cursor_position()
                .and_then(|cursor| camera.viewport_to_world_2d(camera_transform, cursor))
                .map(|ray| (ray.x, ray.y))
            {
                mouse_position.x = mouse_world_pos.0;
                mouse_position.y = mouse_world_pos.1;
            }
        }
    }
}
