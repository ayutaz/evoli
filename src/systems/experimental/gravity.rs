use bevy::prelude::*;

use crate::components::creatures::{FallingTag, Movement};

/// Acceleration due to gravity.
const GRAVITY: f32 = 4.0;

/// Applies the force of gravity on all entities with the FallingTag.
/// Will reduce the vertical velocity each frame according to GRAVITY.
pub fn gravity_system(
    time: Res<Time>,
    mut query: Query<&mut Movement, With<FallingTag>>,
) {
    let dt = time.delta_secs();
    for mut movement in &mut query {
        // TODO: Add terminal velocity cap on falling speed.
        movement.velocity.z -= GRAVITY * dt;
    }
}
