use bevy::prelude::*;

#[derive(Resource, Default, Clone, Debug)]
pub struct DebugConfig {
    pub visible: bool,
}
