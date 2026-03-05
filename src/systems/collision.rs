use bevy::prelude::*;

use crate::components::collider;
use crate::components::creatures::{CreatureTag, Movement};
use crate::resources::world_bounds::WorldBounds;

/// Event emitted when two circle colliders overlap.
#[derive(Event, Debug, Clone)]
pub struct CollisionEvent {
    pub entity_a: Entity,
    pub entity_b: Entity,
}

impl CollisionEvent {
    pub fn new(entity_a: Entity, entity_b: Entity) -> CollisionEvent {
        CollisionEvent { entity_a, entity_b }
    }
}

/// O(n^2) collision detection between all circle colliders.
/// When two circles overlap, the velocity of the first entity is redirected
/// away from the second entity, and a CollisionEvent is emitted.
pub fn collision_system(
    mut query: Query<(Entity, &collider::Circle, &mut Movement, &Transform)>,
    others: Query<(Entity, &collider::Circle, &Transform)>,
    mut collision_events: EventWriter<CollisionEvent>,
) {
    // We need to collect positions first to avoid borrow conflicts.
    // The original Amethyst code iterated (circles, mut movements, locals, entities)
    // against (circles, locals, entities), which Bevy's borrow checker won't allow
    // with a single query. We use two separate queries instead.
    let other_data: Vec<(Entity, f32, Vec3)> = others
        .iter()
        .map(|(entity, circle, transform)| (entity, circle.radius, transform.translation))
        .collect();

    for (entity_a, circle_a, mut movement, transform_a) in &mut query {
        let pos_a = transform_a.translation;

        for &(entity_b, radius_b, pos_b) in &other_data {
            if entity_a == entity_b {
                continue;
            }

            let allowed_distance = circle_a.radius + radius_b;
            let direction = pos_a - pos_b;
            if direction.length_squared() < allowed_distance * allowed_distance {
                collision_events.send(CollisionEvent::new(entity_a, entity_b));

                if direction.length() < f32::EPSILON {
                    movement.velocity = -movement.velocity;
                } else {
                    let norm_direction = direction.normalize();
                    movement.velocity = norm_direction * movement.velocity.length();
                }
            }
        }
    }
}

/// Clamps creature positions within the WorldBounds rectangle.
pub fn enforce_bounds_system(
    mut query: Query<&mut Transform, With<CreatureTag>>,
    bounds: Res<WorldBounds>,
) {
    for mut transform in &mut query {
        let pos = transform.translation;

        if pos.x > bounds.right {
            transform.translation.x = bounds.right;
        } else if pos.x < bounds.left {
            transform.translation.x = bounds.left;
        }

        if pos.y > bounds.top {
            transform.translation.y = bounds.top;
        } else if pos.y < bounds.bottom {
            transform.translation.y = bounds.bottom;
        }
    }
}

/// Debug system that logs collision events to the console.
pub fn debug_collision_event_system(
    mut collision_events: EventReader<CollisionEvent>,
) {
    for event in collision_events.read() {
        info!("Received collision event {:?}", event);
    }
}

/// Debug system that draws collider circles using Gizmos.
pub fn debug_collider_system(
    query: Query<(&collider::Circle, &GlobalTransform)>,
    mut gizmos: Gizmos,
) {
    for (circle, global_transform) in &query {
        let mut position = global_transform.translation();
        position.z += 1.0;
        gizmos.circle_2d(
            Isometry2d::from_translation(position.truncate()),
            circle.radius,
            Color::srgba(1.0, 0.5, 0.5, 1.0),
        );
    }
}
