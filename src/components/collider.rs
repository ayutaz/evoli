use bevy::prelude::*;
use serde::{Deserialize, Serialize};

#[derive(Clone, Debug, Deserialize, Serialize, Component)]
pub struct Circle {
    pub radius: f32,
}
