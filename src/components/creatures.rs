use bevy::prelude::*;
use serde::{Deserialize, Serialize};

use crate::components::{
    collider::Circle, combat::CombatData, digestion::DigestionData,
    experimental::perception::Perception,
};

pub type CreatureType = String;

/// Tag all creatures for when we need to run operations against everything.
#[derive(Clone, Copy, Debug, Default, Deserialize, Serialize, Component)]
pub struct CreatureTag;

#[derive(Clone, Copy, Debug, Default, Deserialize, Serialize, Component)]
pub struct RicochetTag;

/// Entities tagged with this Component (and of course a Transform and Movement) will actively
/// avoid obstacles by steering away from them.
/// The world bounds currently (v0.2.0) are the only obstacles.
#[derive(Clone, Copy, Debug, Default, Deserialize, Serialize, Component)]
pub struct AvoidObstaclesTag;

/// Required on Topplegrass, this is what gives it its toppling animation.
#[derive(Clone, Copy, Debug, Default, Deserialize, Serialize, Component)]
pub struct TopplegrassTag;

/// Give this tag to any entity that is falling and should be affected by gravity.
#[derive(Clone, Copy, Debug, Default, Deserialize, Serialize, Component)]
pub struct FallingTag;

/// Entities tagged with this Component will despawn as soon as their position is outside the world bounds.
#[derive(Clone, Copy, Debug, Default, Deserialize, Serialize, Component)]
pub struct DespawnWhenOutOfBoundsTag;

#[derive(Clone, Copy, Debug, Default, Deserialize, Serialize, Component)]
pub struct IntelligenceTag;

#[derive(Clone, Debug, Default, Deserialize, Serialize, Component)]
pub struct Movement {
    #[serde(default)]
    pub velocity: Vec3,
    pub max_movement_speed: f32,
}

#[derive(Clone, Copy, Debug, Default, Deserialize, Serialize, Component)]
pub struct Wander {
    pub angle: f32,
    pub radius: f32,
}

impl Wander {
    pub fn get_direction(&self) -> Vec3 {
        Vec3::new(
            self.radius * self.angle.cos(),
            self.radius * self.angle.sin(),
            0.0,
        )
    }
}

#[derive(Clone, Debug, Default, Deserialize, Serialize, Component)]
pub struct Carcass {
    pub creature_type: CreatureType,
}

/// Data-driven creature definition for RON deserialization.
/// Replaces the old `CreaturePrefabData`.
#[derive(Deserialize, Serialize, Default, Clone, Debug)]
#[serde(default)]
#[serde(deny_unknown_fields)]
pub struct CreatureDefinition {
    pub name: Option<String>,
    pub gltf: Option<String>,
    pub movement: Option<Movement>,
    pub wander: Option<Wander>,
    pub collider: Option<Circle>,
    pub digestion: Option<DigestionData>,
    pub combat: Option<CombatData>,
    pub intelligence_tag: bool,
    pub perception: Option<Perception>,
    pub ricochet_tag: bool,
    pub carcass: Option<Carcass>,
    pub avoid_obstacles_tag: bool,
    pub despawn_when_out_of_bounds_tag: bool,
    pub topplegrass_tag: bool,
    pub falling_tag: bool,
    pub creature_tag: bool,
}
