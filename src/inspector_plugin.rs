use crate::common_components::{Health, TeamEntity};
use crate::units::{MoveTarget, MoveToPoint};
use crate::waypoint_plugin::{IsStartPoint, Waypoint};
use bevy::app::{App, Plugin};
use bevy::input::common_conditions::input_toggle_active;
use bevy::prelude::KeyCode;
use bevy_inspector_egui::quick::WorldInspectorPlugin;

pub struct InspectorPlugin;

impl Plugin for InspectorPlugin {
    fn build(&self, app: &mut App) {
        app.add_plugins(
            WorldInspectorPlugin::default().run_if(input_toggle_active(true, KeyCode::Escape)),
        )
        .register_type::<TeamEntity>()
        .register_type::<IsStartPoint>()
        .register_type::<Waypoint>()
        .register_type::<Health>()
        .register_type::<MoveTarget>()
        .register_type::<MoveToPoint>();
    }
}
