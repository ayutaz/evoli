use bevy::prelude::*;
use serde::{Deserialize, Serialize};

#[derive(Resource, Default, Clone, Debug, Deserialize, Serialize)]
pub struct Wind {
    pub wind: Vec2,
}
