use bevy::prelude::*;
use std::time::Duration;

use crate::components::combat::{Cooldown, Damage, FactionPrey, HasFaction, Health, Speed};
use crate::components::digestion::{Fullness, Nutrition};
use crate::systems::collision::CollisionEvent;

/// Event emitted when one entity attacks another.
#[derive(Event)]
pub struct AttackEvent {
    pub attacker: Entity,
    pub defender: Entity,
}

/// Decrements cooldown timers each frame. Removes the `Cooldown` component when time expires.
pub fn cooldown_system(
    mut commands: Commands,
    time: Res<Time>,
    mut query: Query<(Entity, &mut Cooldown)>,
) {
    for (entity, mut cooldown) in query.iter_mut() {
        match cooldown.time_left.checked_sub(time.delta()) {
            Some(time_left) => {
                cooldown.time_left = time_left;
            }
            None => {
                commands.entity(entity).remove::<Cooldown>();
            }
        }
    }
}

/// Reads `CollisionEvent`s, checks faction relationships (prey/predator), and emits `AttackEvent`s.
pub fn find_attack_system(
    mut collision_events: EventReader<CollisionEvent>,
    mut attack_events: EventWriter<AttackEvent>,
    has_faction_query: Query<&HasFaction>,
    faction_prey_query: Query<&FactionPrey>,
) {
    for event in collision_events.read() {
        let opt_factions = has_faction_query
            .get(event.entity_a)
            .ok()
            .and_then(|a| has_faction_query.get(event.entity_b).ok().map(|b| (a, b)));

        if let Some((faction_a, faction_b)) = opt_factions {
            // Check if A considers B's faction as prey
            if let Ok(preys_a) = faction_prey_query.get(faction_a.0) {
                if preys_a.is_prey(&faction_b.0) {
                    attack_events.send(AttackEvent {
                        attacker: event.entity_a,
                        defender: event.entity_b,
                    });
                }
            }

            // Check if B considers A's faction as prey
            if let Ok(preys_b) = faction_prey_query.get(faction_b.0) {
                if preys_b.is_prey(&faction_a.0) {
                    attack_events.send(AttackEvent {
                        attacker: event.entity_b,
                        defender: event.entity_a,
                    });
                }
            }
        }
    }
}

/// Reads `AttackEvent`s, applies damage to defenders, transfers nutrition to attackers,
/// and adds a `Cooldown` component to the attacker.
pub fn perform_default_attack_system(
    mut commands: Commands,
    mut attack_events: EventReader<AttackEvent>,
    damage_query: Query<(&Damage, &Speed), Without<Cooldown>>,
    mut health_query: Query<&mut Health>,
    mut fullness_query: Query<&mut Fullness>,
    mut nutrition_query: Query<&mut Nutrition>,
) {
    for event in attack_events.read() {
        // Check if attacker can attack (has Damage + Speed, no Cooldown)
        if let Ok((damage, speed)) = damage_query.get(event.attacker) {
            let damage_value = damage.damage;
            let cooldown_duration =
                Duration::from_millis((1000.0 / speed.attacks_per_second) as u64);

            // Apply damage to defender's health
            if let Ok(mut health) = health_query.get_mut(event.defender) {
                health.value -= damage_value;
            }

            // Transfer nutrition from defender to attacker's fullness
            if let Ok(mut attacker_fullness) = fullness_query.get_mut(event.attacker) {
                if let Ok(mut defender_nutrition) = nutrition_query.get_mut(event.defender) {
                    let delta = defender_nutrition.value.min(damage_value);
                    defender_nutrition.value -= delta;
                    attacker_fullness.value += delta;
                }
            }

            // Add cooldown to attacker
            commands
                .entity(event.attacker)
                .insert(Cooldown::new(cooldown_duration));
        }
    }
}
