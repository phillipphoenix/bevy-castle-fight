use bevy::prelude::{Vec2, Vec3};

/*
Contains helper functions related to grids, for instance snapping to the grid.
*/

// Define a trait for snapping to grid functionality
pub trait SnapToGrid {
    fn snap_to_grid(self, grid_size: f32) -> Self;
}

// Implement the trait for Vec2
impl SnapToGrid for Vec2 {
    fn snap_to_grid(self, grid_size: f32) -> Self {
        Vec2::new(
            (self.x / grid_size).round() * grid_size,
            (self.y / grid_size).round() * grid_size,
        )
    }
}

// Implement the trait for Vec3
impl SnapToGrid for Vec3 {
    fn snap_to_grid(self, grid_size: f32) -> Self {
        Vec3::new(
            (self.x / grid_size).round() * grid_size,
            (self.y / grid_size).round() * grid_size,
            self.z, // Assuming z should remain unchanged
        )
    }
}
