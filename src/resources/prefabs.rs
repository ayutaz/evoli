use bevy::prelude::*;
use std::collections::HashMap;

use crate::components::creatures::CreatureDefinition;

#[derive(Resource, Default)]
pub struct CreaturePrefabs {
    pub prefabs: HashMap<String, CreatureDefinition>,
}

#[derive(Resource, Default)]
pub struct Factions(pub HashMap<String, Entity>);
