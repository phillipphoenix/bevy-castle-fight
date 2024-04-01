use crate::health::Health;
use crate::movement::{MoveTarget, MoveToPoint, WaypointFollower};
use crate::teams::Team;
use crate::vision::{InVision, VisionRange};
use crate::waypoints::{IsStartPoint, Waypoint, WaypointMap};
use bevy::app::{App, Plugin};
use bevy::input::common_conditions::input_toggle_active;
use bevy::prelude::KeyCode;
use bevy_inspector_egui::quick::{ResourceInspectorPlugin, WorldInspectorPlugin};

pub struct InspectorPlugin;

impl Plugin for InspectorPlugin {
    fn build(&self, app: &mut App) {
        app.add_plugins((
            WorldInspectorPlugin::default().run_if(input_toggle_active(true, KeyCode::Escape)),
            ResourceInspectorPlugin::<WaypointMap>::default()
                .run_if(input_toggle_active(true, KeyCode::Escape)),
        ))
        // Types.
        .register_type::<Team>()
        .register_type::<IsStartPoint>()
        .register_type::<Waypoint>()
        .register_type::<WaypointFollower>()
        .register_type::<Health>()
        .register_type::<MoveTarget>()
        .register_type::<MoveToPoint>()
        .register_type::<VisionRange>()
        .register_type::<InVision>();
    }
}
