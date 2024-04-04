use std::time::Duration;

use bevy::prelude::*;
use bevy::time::common_conditions::on_timer;
use bevy_spatial::kdtree::KDTree2;
use bevy_spatial::{AutomaticUpdate, SpatialAccess, SpatialStructure, TransformMode};

use crate::game::teams::Team;

// --- Plugin ---

pub struct VisionPlugin;

impl Plugin for VisionPlugin {
    fn build(&self, app: &mut App) {
        app.add_plugins(
            AutomaticUpdate::<Team>::new()
                .with_spatial_ds(SpatialStructure::KDTree2)
                .with_frequency(Duration::from_millis(200))
                .with_transform(TransformMode::GlobalTransform),
        )
        .add_systems(
            Update,
            check_vision.run_if(on_timer(Duration::from_millis(200))),
        );
    }
}

// --- Types ---

// type alias for easier usage later
type TeamEntityTree = KDTree2<Team>;

// --- Components ---

/// From how far away can an entity spot for instance opponents.
#[derive(Component, Reflect)]
pub struct VisionRange(pub f32);

#[derive(Component)]
pub struct Visible;

#[derive(Component, Reflect)]
pub struct InVision {
    pub friendlies: Vec<Entity>,
    pub enemies: Vec<Entity>,
}

// --- Systems ---

fn check_vision(
    team_entity_tree: Res<TeamEntityTree>,
    mut query: Query<(&Transform, &Team, &VisionRange, &mut InVision)>,
    other_query: Query<(&Team, Option<&Visible>)>,
) {
    for (transform, team, vision_range, mut in_vision) in query.iter_mut() {
        in_vision.friendlies.clear();
        in_vision.enemies.clear();

        for (_, opt_entity) in
            team_entity_tree.within_distance(transform.translation.xy(), vision_range.0)
        {
            if let Some(entity) = opt_entity {
                if let Ok((other_team, opt_other_visible)) = other_query.get(entity) {
                    if team == other_team {
                        in_vision.friendlies.push(entity);
                    } else if opt_other_visible.is_some() {
                        // Only add enemy, if it is visible.
                        in_vision.enemies.push(entity);
                    }
                }
            }
        }
    }
}
