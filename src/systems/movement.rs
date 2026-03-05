use bevy::prelude::*;

use crate::components::creatures::{CreatureTag, Movement};

/// Applies velocity to translation for all entities with Movement and Transform.
pub fn movement_system(
    time: Res<Time>,
    mut query: Query<(&Movement, &mut Transform)>,
) {
    let dt = time.delta_secs();
    for (movement, mut transform) in &mut query {
        transform.translation += movement.velocity * dt;
    }
}

/// Rotates creatures to face the direction they are moving.
/// Only applies to entities with the CreatureTag component.
pub fn creature_rotation_system(
    mut query: Query<(&Movement, &mut Transform), With<CreatureTag>>,
) {
    for (movement, mut transform) in &mut query {
        if movement.velocity.length_squared() > 0.001 {
            let angle = movement.velocity.y.atan2(movement.velocity.x);
            transform.rotation = Quat::from_rotation_z(angle);
        }
    }
}

/// Caps velocity at max_movement_speed for all entities with Movement.
pub fn velocity_cap_system(
    mut query: Query<&mut Movement>,
) {
    for mut movement in &mut query {
        let mag = movement.velocity.length();
        if mag > movement.max_movement_speed && mag > 0.0 {
            movement.velocity = movement.velocity * (movement.max_movement_speed / mag);
        }
    }
}
