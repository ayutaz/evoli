use bevy::prelude::*;

use crate::components::creatures::DespawnWhenOutOfBoundsTag;
use crate::resources::world_bounds::WorldBounds;

/// Deletes any entity tagged with DespawnWhenOutOfBoundsTag if they are detected to be outside
/// the world bounds (with a small margin) or below the ground.
pub fn out_of_bounds_despawn_system(
    mut commands: Commands,
    bounds: Res<WorldBounds>,
    query: Query<(Entity, &Transform), With<DespawnWhenOutOfBoundsTag>>,
) {
    for (entity, transform) in &query {
        let pos = transform.translation;
        if pos.x < bounds.left - 5.0
            || pos.x > bounds.right + 5.0
            || pos.y < bounds.bottom - 5.0
            || pos.y > bounds.top + 5.0
            || pos.z < -10.0
        {
            commands.entity(entity).despawn_recursive();
        }
    }
}
