use bevy::prelude::*;
use bevy_ecs_ldtk::app::LdtkEntityAppExt;

pub struct CameraPlugin;

impl Plugin for CameraPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(Startup, init_camera)
            .add_systems(Update, move_camera);
    }
}

#[derive(Component)]
struct MainCamera;

fn init_camera(mut commands: Commands) {
    commands.spawn((Camera2dBundle::default(), MainCamera));
}

fn move_camera(
    mouse_input: Res<ButtonInput<MouseButton>>,
    key_input: Res<ButtonInput<KeyCode>>,
    mut query: Query<&mut Transform, With<MainCamera>>,
    time: Res<Time<Real>>,
) {
    // Keyboard movement.
    let mut move_direction = Vec2::new(0., 0.);
    if key_input.pressed(KeyCode::ArrowLeft) {
        move_direction += Vec2::new(-1., 0.);
    }
    if key_input.pressed(KeyCode::ArrowRight) {
        move_direction += Vec2::new(1., 0.);
    }
    if key_input.pressed(KeyCode::ArrowUp) {
        move_direction += Vec2::new(0., 1.);
    }
    if key_input.pressed(KeyCode::ArrowDown) {
        move_direction += Vec2::new(0., -1.);
    }

    let mut cam_transform = query.single_mut();
    let cam_movement_speed: f32 = 32. * 10.;
    let movement = move_direction.extend(cam_transform.translation.z)
        * cam_movement_speed
        * time.delta_seconds();
    cam_transform.translation += movement;
}
