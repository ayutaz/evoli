use bevy::prelude::*;

use crate::components::combat::Health;

/// Uses Gizmos to draw a health bar above each entity that has `Health` and `Transform`.
pub fn debug_health_system(
    mut gizmos: Gizmos,
    query: Query<(&Health, &Transform)>,
) {
    for (health, transform) in query.iter() {
        let pos = transform.translation;
        let bar_length = health.value / 100.0;

        // Draw health bar slightly above and in front of the entity
        let start = Vec3::new(pos.x, pos.y + 0.5, pos.z + 0.5);
        let end = Vec3::new(pos.x + bar_length, pos.y + 0.5, pos.z + 0.5);

        gizmos.line(start, end, Color::srgb(0.0, 1.0, 0.0));
    }
}
