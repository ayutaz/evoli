use bevy::prelude::*;
use serde::{Deserialize, Serialize};

#[derive(Default, Debug, Clone, Deserialize, Serialize, Component)]
pub struct Digestion {
    /// Points of fullness lost every second
    pub nutrition_burn_rate: f32,
}

#[derive(Default, Debug, Clone, Deserialize, Serialize, Component)]
pub struct Fullness {
    pub max: f32,
    pub value: f32,
}

#[derive(Default, Debug, Clone, Deserialize, Serialize, Component)]
pub struct Nutrition {
    /// Nutritional value of the entity
    pub value: f32,
}

/// Digestion-related data for creature definitions (used in RON deserialization).
#[derive(Default, Debug, Clone, Deserialize, Serialize)]
#[serde(default)]
#[serde(deny_unknown_fields)]
pub struct DigestionData {
    pub fullness: Option<Fullness>,
    pub digestion: Option<Digestion>,
    pub nutrition: Option<Nutrition>,
}
