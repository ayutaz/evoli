use bevy::prelude::*;

use crate::components::creatures::CreatureTag;
use crate::components::experimental::perception::{DetectedEntities, Perception};
use crate::resources::experimental::spatial_grid::SpatialGrid;

/// Rebuilds the spatial grid each frame by inserting all entities with a CreatureTag.
pub fn spatial_grid_system(
    mut grid: ResMut<SpatialGrid>,
    query: Query<(Entity, &Transform), With<CreatureTag>>,
) {
    grid.reset();
    for (entity, transform) in &query {
        grid.insert(entity, transform);
    }
}

/// Detects nearby entities for each entity with a Perception component, using the SpatialGrid
/// for broad-phase acceleration and a squared-distance check for fine-grained filtering.
pub fn entity_detection_system(
    mut commands: Commands,
    grid: Res<SpatialGrid>,
    all_transforms: Query<(Entity, &Transform)>,
    mut perception_query: Query<(
        Entity,
        &Perception,
        &Transform,
        Option<&mut DetectedEntities>,
    )>,
) {
    for (entity, perception, transform, detected_opt) in &mut perception_query {
        let pos = transform.translation;
        let sq_range = perception.range * perception.range;
        let nearby = grid.query(transform, perception.range);

        let mut detected_set = std::collections::HashSet::new();
        for (other_entity, other_transform) in all_transforms.iter() {
            if other_entity == entity {
                continue;
            }
            if !nearby.contains(&other_entity.index()) {
                continue;
            }
            let other_pos = other_transform.translation;
            if (pos - other_pos).length_squared() < sq_range {
                detected_set.insert(other_entity);
            }
        }

        match detected_opt {
            Some(mut detected) => {
                detected.entities = detected_set;
            }
            None => {
                commands.entity(entity).insert(DetectedEntities {
                    entities: detected_set,
                });
            }
        }
    }
}
