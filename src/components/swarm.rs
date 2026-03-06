use bevy::prelude::*;

#[derive(Clone, Debug, Default, Component)]
pub struct SwarmCenter {
    pub entities: Vec<Entity>,
}

#[derive(Clone, Debug, Default, Component)]
pub struct SwarmBehavior {
    #[allow(dead_code)]
    pub swarm_center: Option<Entity>,

    pub attraction: f32,
    pub deviation: f32,
}
