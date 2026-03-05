use bevy::prelude::*;

#[derive(Resource, Clone, Debug)]
pub struct WorldBounds {
    pub left: f32,
    pub right: f32,
    pub bottom: f32,
    pub top: f32,
}

impl WorldBounds {
    pub fn new(left: f32, right: f32, bottom: f32, top: f32) -> Self {
        WorldBounds {
            left,
            right,
            bottom,
            top,
        }
    }
}
