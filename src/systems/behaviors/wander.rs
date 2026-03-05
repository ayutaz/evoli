use bevy::prelude::*;
use rand::{thread_rng, Rng};

use crate::components::creatures;

pub fn wander_system(
    time: Res<Time>,
    mut query: Query<(&mut creatures::Wander, &mut creatures::Movement, &Transform)>,
) {
    let delta_time = time.delta_secs();
    let mut rng = thread_rng();

    for (mut wander, mut movement, transform) in query.iter_mut() {
        let position = transform.translation;
        let future_position = position + movement.velocity * 0.5;

        let direction = wander.get_direction();
        let target = future_position + direction;

        let desired_velocity = target - position;

        movement.velocity += desired_velocity * delta_time;
        // Quick and dirty fix to keep entities from wandering into the ground if they target
        // an entity not on the same z-level as themselves.
        movement.velocity.z = 0.0;

        let change = 10.0;
        if rng.gen::<bool>() {
            wander.angle += change * delta_time; // Radians per second
        } else {
            wander.angle -= change * delta_time;
        }
    }
}
