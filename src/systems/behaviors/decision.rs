use bevy::prelude::*;
use bevy::utils::HashSet;

use std::collections::HashMap;
use std::marker::PhantomData;

use crate::components::combat::{FactionPrey, HasFaction};
use crate::components::creatures::*;

// --- Marker types ---

#[derive(Default)]
pub struct Prey;

#[derive(Default)]
pub struct Predator;

// --- FactionQueries resource ---
// Replaces the old Amethyst `Query<T>` component (BitSet-based) with a Resource
// that maps faction Entity -> set of matching Entities.

#[derive(Resource)]
pub struct FactionQueries<T: Send + Sync + 'static> {
    pub queries: HashMap<Entity, HashSet<Entity>>,
    _marker: PhantomData<T>,
}

impl<T: Send + Sync + 'static> Default for FactionQueries<T> {
    fn default() -> Self {
        FactionQueries {
            queries: HashMap::new(),
            _marker: PhantomData,
        }
    }
}

// --- Closest component ---
// Stores the direction vector (not just distance) to the closest entity of type T.

#[derive(Component)]
pub struct Closest<T: Send + Sync + 'static> {
    pub distance: Vec3,
    _marker: PhantomData<T>,
}

impl<T: Send + Sync + 'static> Closest<T> {
    pub fn new(distance: Vec3) -> Closest<T> {
        Closest {
            distance,
            _marker: PhantomData,
        }
    }
}

// --- SeekConfig resource ---
// Replaces the per-system fields that were on SeekSystem<T> in Amethyst.
// In Bevy, system functions cannot have custom fields, so we store configuration in a Resource.

#[derive(Resource)]
pub struct SeekConfig<T: Send + Sync + 'static> {
    /// A rotation quaternion applied to the steering force direction.
    pub attraction_modifier: Quat,
    /// The magnitude of the desired velocity towards/away from the target.
    pub attraction_magnitude: f32,
    _marker: PhantomData<T>,
}

impl<T: Send + Sync + 'static> SeekConfig<T> {
    pub fn new(attraction_modifier: Quat, attraction_magnitude: f32) -> SeekConfig<T> {
        SeekConfig {
            attraction_modifier,
            attraction_magnitude,
            _marker: PhantomData,
        }
    }
}

// --- QueryPredatorsAndPreySystem ---
// For each faction entity (one that has FactionPrey), calculate the set of entities
// that they consider prey and the set of entities that consider them predators.

pub fn query_predators_and_prey_system(
    faction_query: Query<(Entity, &FactionPrey)>,
    creature_query: Query<(Entity, &HasFaction)>,
    mut prey_queries: ResMut<FactionQueries<Prey>>,
    mut predator_queries: ResMut<FactionQueries<Predator>>,
) {
    // Ensure each faction has entries, and clear old data
    for (faction_entity, _) in faction_query.iter() {
        prey_queries
            .queries
            .entry(faction_entity)
            .or_default()
            .clear();
        predator_queries
            .queries
            .entry(faction_entity)
            .or_default()
            .clear();
    }

    // Build prey queries: for each faction, find all creatures whose faction is listed as prey
    for (faction_entity, faction_preys) in faction_query.iter() {
        let preys = prey_queries.queries.get_mut(&faction_entity).unwrap();
        for (creature_entity, creature_faction) in creature_query.iter() {
            if faction_preys.is_prey(&creature_faction.0) {
                preys.insert(creature_entity);
            }
        }
    }

    // Build predator queries: for each creature that has a faction, find what factions
    // consider that creature's faction as prey, and mark the creature as a predator of those factions.
    for (predator_entity, predator_has_faction) in creature_query.iter() {
        // Get the FactionPrey for this creature's faction
        if let Ok((_, predator_faction_preys)) = faction_query.get(predator_has_faction.0) {
            for (prey_faction_entity, _) in faction_query.iter() {
                if predator_faction_preys.is_prey(&prey_faction_entity) {
                    let predators = predator_queries
                        .queries
                        .get_mut(&prey_faction_entity)
                        .unwrap();
                    predators.insert(predator_entity);
                }
            }
        }
    }
}

// --- ClosestSystem ---
// Generic system that finds the closest entity from FactionQueries<T> for each creature.
// Attaches/updates Closest<T> component on entities that have a nearby match (within 5.0 units).

pub fn closest_system<T: Send + Sync + 'static>(
    mut commands: Commands,
    transforms: Query<&Transform>,
    creature_query: Query<(Entity, &Transform, &HasFaction)>,
    existing_closest: Query<Entity, With<Closest<T>>>,
    faction_queries: Res<FactionQueries<T>>,
) {
    // Remove old Closest<T> components (the referenced entities might have moved or been deleted)
    for entity in existing_closest.iter() {
        commands.entity(entity).remove::<Closest<T>>();
    }

    for (entity, transform, faction) in creature_query.iter() {
        // If the query is not attached to the faction, skip
        let query_entities = match faction_queries.queries.get(&faction.0) {
            Some(set) => set,
            None => continue,
        };

        let mut closest_opt: Option<Vec3> = None;
        let mut min_sq_distance = 5.0f32.powi(2);

        let position = transform.translation;

        for &query_entity in query_entities.iter() {
            if let Ok(query_transform) = transforms.get(query_entity) {
                let query_position = query_transform.translation;
                let difference = query_position - position;
                let sq_distance = difference.length_squared();
                if sq_distance < min_sq_distance {
                    min_sq_distance = sq_distance;
                    closest_opt = Some(difference);
                }
            }
        }

        if let Some(c) = closest_opt {
            commands.entity(entity).insert(Closest::<T>::new(c));
        }
    }
}

// --- SeekSystem ---
// Applies a steering force towards (or away from, depending on config) the Closest<T> entity.
// With attraction_modifier as identity quaternion, this seeks.
// With attraction_modifier rotating 180 degrees, this evades.

pub fn seek_system<T: Send + Sync + 'static>(
    time: Res<Time>,
    config: Res<SeekConfig<T>>,
    mut query: Query<(&Closest<T>, &mut Movement)>,
) {
    let delta_time = time.delta_secs();
    for (closest, mut movement) in query.iter_mut() {
        if closest.distance.length() < f32::EPSILON {
            continue;
        }
        let target_velocity = closest.distance.normalize() * config.attraction_magnitude;
        let steering_force = target_velocity - movement.velocity;
        // Apply rotation modifier to steering force
        let rotated_force = config.attraction_modifier * steering_force;
        movement.velocity += rotated_force * delta_time;
    }
}
