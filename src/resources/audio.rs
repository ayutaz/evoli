use bevy::prelude::*;

#[derive(Resource)]
pub struct BgmHandle(pub Option<Handle<AudioSource>>);
