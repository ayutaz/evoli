use bevy::prelude::*;

use rand::{thread_rng, Rng};
use std::f32;

use crate::{
    components::{
        creatures::{AvoidObstaclesTag, Movement, Wander},
        swarm::{SwarmBehavior, SwarmCenter},
    },
    systems::spawner::CreatureSpawnEvent,
};

/// Local resource for tracking swarm spawn timing.
#[derive(Resource)]
pub struct SwarmSpawnTimer {
    pub timer: f32,
}

impl Default for SwarmSpawnTimer {
    fn default() -> Self {
        Self { timer: 0.0 }
    }
}

/// Periodically spawns a new swarm with a center entity and several swarmling children.
pub fn swarm_spawn_system(
    mut commands: Commands,
    mut timer: ResMut<SwarmSpawnTimer>,
    mut spawn_events: EventWriter<CreatureSpawnEvent>,
    time: Res<Time>,
) {
    let delta_seconds = time.delta_secs();
    timer.timer -= delta_seconds;
    if timer.timer <= 0.0 {
        let mut rng = thread_rng();
        timer.timer = 10.0f32;

        let x = rng.gen_range(-10.0..10.0);
        let y = rng.gen_range(-10.0..10.0);

        // Spawn the swarm center entity
        let swarm_entity = commands
            .spawn((
                Transform::from_translation(Vec3::new(x, y, 2.0)),
                Movement {
                    velocity: Vec3::ZERO,
                    max_movement_speed: 0.8,
                },
                Wander {
                    radius: 1.0,
                    angle: 0.0,
                },
                AvoidObstaclesTag,
            ))
            .id();

        let nb_swarm_individuals = rng.gen_range(3..10);
        let mut swarmling_entities = Vec::new();

        for _ in 0..nb_swarm_individuals {
            let sx = rng.gen_range(-1.0..1.0);
            let sy = rng.gen_range(-1.0..1.0);

            let swarmling_entity = commands
                .spawn((
                    SwarmBehavior {
                        swarm_center: Some(swarm_entity),
                        attraction: 0.5f32,
                        deviation: 0.5f32,
                    },
                    Transform {
                        translation: Vec3::new(sx, sy, 0.0),
                        scale: Vec3::splat(0.1),
                        ..default()
                    },
                    Movement {
                        velocity: Vec3::new(
                            rng.gen_range(-1.0..1.0),
                            rng.gen_range(-1.0..1.0),
                            0.0,
                        ),
                        max_movement_speed: 5.0,
                    },
                ))
                .id();

            // Set parent-child relationship (Bevy 0.15)
            commands.entity(swarmling_entity).set_parent(swarm_entity);

            swarmling_entities.push(swarmling_entity);

            // Send spawn event so the spawner system can attach Ixie-specific components
            spawn_events.send(CreatureSpawnEvent {
                creature_type: "Ixie".to_string(),
                transform: Transform::default(), // transform already set on entity
            });
        }

        // Insert SwarmCenter component on the swarm entity
        commands.entity(swarm_entity).insert(SwarmCenter {
            entities: swarmling_entities,
        });
    }
}

/// Cleans up swarm centers: removes dead swarmlings from the list and despawns the center
/// entity when all swarmlings are gone.
pub fn swarm_center_system(
    mut commands: Commands,
    mut query: Query<(Entity, &mut SwarmCenter)>,
    swarm_behaviors: Query<&SwarmBehavior>,
) {
    for (entity, mut swarm_center) in &mut query {
        swarm_center
            .entities
            .retain(|swarmling| swarm_behaviors.get(*swarmling).is_ok());

        if swarm_center.entities.is_empty() {
            commands.entity(entity).despawn_recursive();
        }
    }
}

/// Simulates swarm behavior: each swarmling is attracted toward the center of its
/// local coordinate space and deviates sideways, creating a swirling pattern.
pub fn swarm_behavior_system(
    time: Res<Time>,
    mut query: Query<(&Transform, &SwarmBehavior, &mut Movement)>,
) {
    let delta_seconds = time.delta_secs();

    if delta_seconds <= f32::EPSILON {
        return;
    }

    let time_step = 0.01;
    let iterations = (delta_seconds / time_step) as u32 + 1;

    for (transform, swarm_behavior, mut movement) in &mut query {
        let original_position = transform.translation;
        let mut current_position = original_position;
        let mut current_velocity = movement.velocity;
        let pull_factor = 10.0;
        let side_factor = 5.0;

        for t in 0..iterations {
            let iter_step = time_step.min(delta_seconds - time_step * t as f32);

            let center_pull = if current_position.length_squared() > 0.16 {
                swarm_behavior.attraction * pull_factor * (-current_position)
            } else {
                Vec3::ZERO
            };

            let mut side_direction = Vec3::new(current_velocity.y, -current_velocity.x, 0.0);
            if side_direction.length_squared() >= f32::EPSILON {
                side_direction = side_direction.normalize();
            }

            let side_deviation_force = swarm_behavior.deviation * side_factor * side_direction;
            let delta_velocity = iter_step * (center_pull + side_deviation_force);
            current_velocity += delta_velocity;

            let speed = current_velocity.length();
            if speed > movement.max_movement_speed {
                current_velocity *= movement.max_movement_speed / speed;
            }

            current_position += iter_step * current_velocity;
        }

        movement.velocity = (current_position - original_position) / delta_seconds;
    }
}
