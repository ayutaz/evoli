use bevy::prelude::*;
use serde::{Deserialize, Serialize};
use std::time::Duration;

#[derive(Default, Debug, Clone, Deserialize, Serialize, Component)]
pub struct Health {
    pub max_health: f32,
    pub value: f32,
}

#[derive(Default, Debug, Clone, Deserialize, Serialize, Component)]
pub struct Damage {
    /// Points subtracted from target's health per hit
    pub damage: f32,
}

#[derive(Default, Debug, Clone, Deserialize, Serialize, Component)]
pub struct Speed {
    pub attacks_per_second: f32,
}

/// As long as the cooldown component is attached to an entity, that entity won't be able to attack.
#[derive(Default, Debug, PartialEq, Eq, Clone, Deserialize, Serialize, Component)]
pub struct Cooldown {
    pub time_left: Duration,
}

impl Cooldown {
    pub fn new(time_left: Duration) -> Cooldown {
        Cooldown { time_left }
    }
}

/// Indicate whether the entity is part of a faction.
/// At runtime, the faction is resolved to an Entity.
#[derive(Debug, PartialEq, Eq, Clone, Component)]
pub struct HasFaction(pub Entity);

/// Store the faction entities this component's owner considers to be prey.
/// At runtime, prey factions are resolved to Entities.
#[derive(Default, Debug, PartialEq, Eq, Clone, Component)]
pub struct FactionPrey(pub Vec<Entity>);

impl FactionPrey {
    pub fn is_prey(&self, other: &Entity) -> bool {
        self.0.contains(other)
    }
}

/// Intermediate type for RON deserialization of HasFaction (faction name as string).
#[derive(Deserialize, Serialize, Default, Clone, Debug)]
pub struct HasFactionData(pub String);

/// Intermediate type for RON deserialization of FactionPrey (faction names as strings).
#[derive(Deserialize, Serialize, Default, Clone, Debug)]
pub struct FactionPreyData(pub Vec<String>);

/// Combat-related data for creature definitions (used in RON deserialization).
#[derive(Default, Debug, Clone, Deserialize, Serialize)]
#[serde(default)]
#[serde(deny_unknown_fields)]
pub struct CombatData {
    pub health: Option<Health>,
    pub speed: Option<Speed>,
    pub damage: Option<Damage>,
    pub faction: Option<String>,
}
