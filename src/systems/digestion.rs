use bevy::prelude::*;

use crate::components::digestion::{Digestion, Fullness};

/// Reduces `Fullness` over time based on the entity's `Digestion` burn rate.
pub fn digestion_system(time: Res<Time>, mut query: Query<(&Digestion, &mut Fullness)>) {
    let delta_time = time.delta_secs();
    for (digestion, mut fullness) in query.iter_mut() {
        let burned = digestion.nutrition_burn_rate * delta_time;
        fullness.value -= burned;
    }
}
