use bevy::prelude::*;

use std::cmp::Ordering;

use crate::components::creatures::{AvoidObstaclesTag, Movement};
use crate::resources::world_bounds::WorldBounds;
use crate::systems::behaviors::decision::Closest;

#[derive(Default)]
pub struct Obstacle;

/// Determine the closest bounding wall based on a location
fn closest_wall(location: &Vec3, bounds: &WorldBounds) -> Vec3 {
    let bounds_left = Vec3::new(bounds.left, location.y, location.z);
    let bounds_right = Vec3::new(bounds.right, location.y, location.z);
    let bounds_top = Vec3::new(location.x, bounds.top, location.z);
    let bounds_bottom = Vec3::new(location.x, bounds.bottom, location.z);

    // Iterates through each bound, calculates the distance vector, and returns the minimum
    [bounds_left, bounds_right, bounds_top, bounds_bottom]
        .iter()
        .map(|v| *v - *location)
        .min_by(|a, b| {
            if a.length_squared() < b.length_squared() {
                Ordering::Less
            } else {
                Ordering::Greater
            }
        })
        .unwrap()
}

pub fn closest_obstacle_system(
    mut commands: Commands,
    world_bounds: Res<WorldBounds>,
    existing_obstacles: Query<Entity, With<Closest<Obstacle>>>,
    query: Query<(Entity, &Transform, &Movement, &AvoidObstaclesTag)>,
) {
    // Right now the only obstacles are the world bound walls, so it's
    // safe to clear this out.
    for entity in existing_obstacles.iter() {
        commands.entity(entity).remove::<Closest<Obstacle>>();
    }

    let threshold = 3.0f32.powi(2);
    for (entity, transform, _movement, _avoid) in query.iter() {
        // Find the closest wall to this entity
        let wall_dir = closest_wall(&transform.translation, &world_bounds);
        if wall_dir.length_squared() < threshold {
            commands
                .entity(entity)
                .insert(Closest::<Obstacle>::new(wall_dir));
        }
    }
}
