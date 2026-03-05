use bevy::prelude::*;
use serde::{Deserialize, Serialize};
use std::collections::HashSet;

#[derive(Default, Clone, Debug, Serialize, Deserialize, Component)]
#[serde(default)]
pub struct Perception {
    pub range: f32,
}

#[derive(Default, Clone, Debug, Component)]
pub struct DetectedEntities {
    pub entities: HashSet<Entity>,
}
