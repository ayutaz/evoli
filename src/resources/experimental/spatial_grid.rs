use bevy::prelude::*;

use std::collections::{HashMap, HashSet};

use crate::utils::spatial_hash::SpatialBuildHasher;

// The SpatialGrid is a spatial hashing structure used to accelerate neighbor searches for entities.
#[derive(Resource)]
pub struct SpatialGrid {
    cell_size: f32,
    cells: HashMap<(i32, i32), HashSet<u32>, SpatialBuildHasher>,
}

impl SpatialGrid {
    pub fn new(cell_size: f32) -> Self {
        SpatialGrid {
            cell_size,
            cells: HashMap::with_hasher(SpatialBuildHasher),
        }
    }

    pub fn reset(&mut self) {
        self.cells = HashMap::with_hasher(SpatialBuildHasher);
    }

    // Insert an entity in the grid based on its Transform component.
    pub fn insert(&mut self, entity: Entity, transform: &Transform) {
        let translation = transform.translation;
        let x_cell = (translation.x / self.cell_size).floor() as i32;
        let y_cell = (translation.y / self.cell_size).floor() as i32;

        let cell_entry = self.cells.entry((x_cell, y_cell)).or_default();
        cell_entry.insert(entity.index());
    }

    // Query the entities close to a certain position.
    // The range of the query is defined by the range input.
    pub fn query(&self, transform: &Transform, range: f32) -> HashSet<u32> {
        let translation = transform.translation;
        let x_cell = (translation.x / self.cell_size).floor() as i32;
        let y_cell = (translation.y / self.cell_size).floor() as i32;
        let integer_range = (range / self.cell_size).ceil() as i32;
        let mut entities = HashSet::new();
        for x in -integer_range..(integer_range + 1) {
            for y in -integer_range..(integer_range + 1) {
                if let Some(cell) = self.cells.get(&(x_cell + x, y_cell + y)) {
                    entities.extend(cell);
                }
            }
        }
        entities
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn grid_creation_insertion_and_query() {
        let mut world = World::new();
        let mut spatial_grid = SpatialGrid::new(1.0f32);

        let transform = Transform::default();
        let e1 = world.spawn_empty().id();
        let e2 = world.spawn_empty().id();
        let e3 = world.spawn_empty().id();
        spatial_grid.insert(e1, &transform);
        spatial_grid.insert(e2, &transform);
        spatial_grid.insert(e3, &transform);

        let mut transform2 = Transform::from_translation(Vec3::new(10.0, 10.0, 10.0));
        let e4 = world.spawn_empty().id();
        spatial_grid.insert(e4, &transform2);

        transform2.translation = Vec3::new(10.5, 12.5, 10.0);
        let e5 = world.spawn_empty().id();
        spatial_grid.insert(e5, &transform2);

        assert_eq!(spatial_grid.query(&transform2, 1.0f32).len(), 1);
        assert_eq!(spatial_grid.query(&transform, 1.0f32).len(), 3);
    }
}
