use bevy::prelude::*;

use crate::components::combat::Health;
use crate::components::creatures::Carcass;
use crate::components::digestion::Fullness;
use crate::systems::spawner::CreatureSpawnEvent;

/// Event emitted when a creature dies.
#[derive(Event, Debug, Clone)]
pub struct CreatureDeathEvent {
    pub deceased: Entity,
}

/// Entities die if their fullness reaches zero (or less).
pub fn starvation_system(
    mut commands: Commands,
    query: Query<(Entity, &Fullness)>,
    mut death_events: EventWriter<CreatureDeathEvent>,
) {
    for (entity, fullness) in query.iter() {
        if fullness.value < f32::EPSILON {
            death_events.send(CreatureDeathEvent { deceased: entity });
            commands.entity(entity).despawn_recursive();
        }
    }
}

/// Entities die if their health reaches zero (or less).
pub fn death_by_health_system(
    mut commands: Commands,
    query: Query<(Entity, &Health)>,
    mut death_events: EventWriter<CreatureDeathEvent>,
) {
    for (entity, health) in query.iter() {
        if health.value < f32::EPSILON {
            death_events.send(CreatureDeathEvent { deceased: entity });
            commands.entity(entity).despawn_recursive();
        }
    }
}

/// Reads `CreatureDeathEvent`s and spawns carcass entities at the deceased's position.
pub fn carcass_system(
    mut death_events: EventReader<CreatureDeathEvent>,
    mut spawn_events: EventWriter<CreatureSpawnEvent>,
    query: Query<(&Transform, &Carcass)>,
) {
    for event in death_events.read() {
        // The entity may already be despawned by the time we process the event,
        // so we use `get` which returns Ok only if it still exists with the required components.
        if let Ok((transform, carcass)) = query.get(event.deceased) {
            spawn_events.send(CreatureSpawnEvent {
                creature_type: carcass.creature_type.clone(),
                transform: *transform,
            });
        }
    }
}
