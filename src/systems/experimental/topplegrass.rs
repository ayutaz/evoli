use bevy::prelude::*;

use rand::{thread_rng, Rng};
use std::f32;

use crate::{
    components::creatures::{FallingTag, Movement, TopplegrassTag},
    resources::experimental::wind::Wind,
    resources::world_bounds::WorldBounds,
    systems::spawner::CreatureSpawnEvent,
};

/// A new topplegrass entity is spawned periodically, SPAWN_INTERVAL is the period in seconds.
const SPAWN_INTERVAL: f32 = 10.0;
/// The standard scaling to apply to the entity.
const TOPPLEGRASS_BASE_SCALE: f32 = 0.002;
/// At which height the topplegrass entity should spawn.
const HEIGHT: f32 = 0.5;
/// Magic value for angular velocity to prevent visible slipping.
const ANGULAR_V_MAGIC: f32 = 2.0;
/// The minimum velocity that a topplegrass entity must have in order to start jumping.
const JUMP_THRESHOLD: f32 = 1.0;
/// The chance per elapsed second that any given non-falling topplegrass will jump.
const JUMP_PROBABILITY: f32 = 4.0;

/// Local resource for tracking spawn timing.
#[derive(Resource)]
pub struct TopplegrassSpawnTimer {
    pub secs_to_next_spawn: f32,
}

impl Default for TopplegrassSpawnTimer {
    fn default() -> Self {
        Self {
            secs_to_next_spawn: 0.0,
        }
    }
}

/// Periodically schedules a Topplegrass entity to be spawned through a CreatureSpawnEvent.
pub fn topplegrass_spawn_system(
    mut spawn_events: EventWriter<CreatureSpawnEvent>,
    mut timer: ResMut<TopplegrassSpawnTimer>,
    time: Res<Time>,
    world_bounds: Res<WorldBounds>,
    wind: Res<Wind>,
) {
    timer.secs_to_next_spawn -= time.delta_secs();
    if timer.secs_to_next_spawn.is_sign_negative() {
        timer.secs_to_next_spawn = SPAWN_INTERVAL;

        let spawn_pos = gen_spawn_location(&wind, &world_bounds);
        let transform = Transform {
            translation: spawn_pos,
            scale: Vec3::splat(TOPPLEGRASS_BASE_SCALE),
            ..default()
        };

        spawn_events.send(CreatureSpawnEvent {
            creature_type: "Topplegrass".to_string(),
            transform,
        });
    }
}

/// Returns a Vec3 representing the position in which to spawn the next entity.
fn gen_spawn_location(wind: &Wind, bounds: &WorldBounds) -> Vec3 {
    let mut rng = thread_rng();
    let wind_vec = wind.wind;

    if wind_towards_direction(wind_vec, Vec2::new(1.0, 0.0)) {
        Vec3::new(
            bounds.left,
            rng.gen_range(bounds.bottom..bounds.top),
            HEIGHT,
        )
    } else if wind_towards_direction(wind_vec, Vec2::new(0.0, 1.0)) {
        Vec3::new(
            rng.gen_range(bounds.left..bounds.right),
            bounds.bottom,
            HEIGHT,
        )
    } else if wind_towards_direction(wind_vec, Vec2::new(-1.0, 0.0)) {
        Vec3::new(
            bounds.right,
            rng.gen_range(bounds.bottom..bounds.top),
            HEIGHT,
        )
    } else {
        Vec3::new(
            rng.gen_range(bounds.left..bounds.right),
            bounds.top,
            HEIGHT,
        )
    }
}

/// Returns true if and only if the given wind vector is roughly in line with the given
/// cardinal_direction vector, within a margin of PI/4 radians.
fn wind_towards_direction(wind: Vec2, cardinal_direction: Vec2) -> bool {
    let angle = wind.angle_to(cardinal_direction).abs();
    angle < f32::consts::FRAC_PI_4
}

/// Controls the rolling animation of the Topplegrass.
pub fn toppling_system(
    mut commands: Commands,
    time: Res<Time>,
    wind: Res<Wind>,
    mut grounded_query: Query<
        (Entity, &mut Movement, &mut Transform),
        (With<TopplegrassTag>, Without<FallingTag>),
    >,
    mut falling_query: Query<
        (Entity, &mut Movement, &mut Transform),
        (With<TopplegrassTag>, With<FallingTag>),
    >,
) {
    let dt = time.delta_secs();
    let mut rng = thread_rng();

    // Phase 1 & 2: Process grounded topplegrass
    let mut to_add_falling = Vec::new();
    for (entity, mut movement, mut transform) in &mut grounded_query {
        let rot_x = Quat::from_rotation_x(-ANGULAR_V_MAGIC * movement.velocity.y * dt);
        let rot_y = Quat::from_rotation_y(ANGULAR_V_MAGIC * movement.velocity.x * dt);
        transform.rotation = rot_x * rot_y * transform.rotation;

        movement.velocity.x = wind.wind.x;
        movement.velocity.y = wind.wind.y;

        let speed = Vec2::new(movement.velocity.x, movement.velocity.y).length();
        if speed > JUMP_THRESHOLD && rng.gen::<f32>() < JUMP_PROBABILITY * dt {
            movement.velocity.z = rng.gen_range(0.4..0.7);
            to_add_falling.push(entity);
        }
    }

    for entity in to_add_falling {
        commands.entity(entity).insert(FallingTag);
    }

    // Phase 3: Process falling topplegrass
    let mut to_remove_falling = Vec::new();
    for (entity, mut movement, mut transform) in &mut falling_query {
        let rot_x = Quat::from_rotation_x(-ANGULAR_V_MAGIC * movement.velocity.y * dt);
        let rot_y = Quat::from_rotation_y(ANGULAR_V_MAGIC * movement.velocity.x * dt);
        transform.rotation = rot_x * rot_y * transform.rotation;

        movement.velocity.x = wind.wind.x;
        movement.velocity.y = wind.wind.y;

        if transform.translation.z <= HEIGHT && movement.velocity.z.is_sign_negative() {
            transform.translation.z = HEIGHT;
            movement.velocity.z = 0.0;
            to_remove_falling.push(entity);
        }
    }

    for entity in to_remove_falling {
        commands.entity(entity).remove::<FallingTag>();
    }
}
