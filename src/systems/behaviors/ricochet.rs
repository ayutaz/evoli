use bevy::prelude::*;

use crate::components::creatures;
use crate::resources::world_bounds::WorldBounds;

pub fn ricochet_system(
    bounds: Res<WorldBounds>,
    mut query: Query<(
        &Transform,
        &creatures::RicochetTag,
        &mut creatures::Movement,
    )>,
) {
    for (transform, _ricochet, mut movement) in query.iter_mut() {
        if transform.translation.x >= bounds.right || transform.translation.x <= bounds.left {
            movement.velocity.x = -movement.velocity.x;
        }

        if transform.translation.y >= bounds.top || transform.translation.y <= bounds.bottom {
            movement.velocity.y = -movement.velocity.y;
        }
    }
}
