# Bevy 移行: 共有型定義（全エージェント共通コンテキスト）

> **注意**: このドキュメントは移行完了後の実装に合わせて更新済み（2026-03-06）。

## Cargo.toml

```toml
[package]
name = "evolution-island"
version = "0.3.0"
edition = "2021"

[dependencies]
bevy = "0.15"
rand = "0.8"
serde = { version = "1.0", features = ["derive"] }
ron = "0.8"

[features]
profiler = []
perception_debug = []
```

## AppState / GamePlayState

```rust
use bevy::prelude::*;

#[derive(States, Default, Debug, Clone, PartialEq, Eq, Hash)]
pub enum AppState {
    #[default]
    Loading,
    Menu,
    InGame,
}

#[derive(SubStates, Default, Debug, Clone, PartialEq, Eq, Hash)]
#[source(AppState = AppState::InGame)]
pub enum GamePlayState {
    #[default]
    Running,
    Paused,
}
```

## コンポーネント型（全エージェント共通）

```rust
// --- Tags (NullStorage → Component) ---
// すべてのタグは Clone, Copy, Deserialize, Serialize も derive している
#[derive(Clone, Copy, Debug, Default, Deserialize, Serialize, Component)]
pub struct CreatureTag;
#[derive(Clone, Copy, Debug, Default, Deserialize, Serialize, Component)]
pub struct RicochetTag;
#[derive(Clone, Copy, Debug, Default, Deserialize, Serialize, Component)]
pub struct AvoidObstaclesTag;
#[derive(Clone, Copy, Debug, Default, Deserialize, Serialize, Component)]
pub struct TopplegrassTag;
#[derive(Clone, Copy, Debug, Default, Deserialize, Serialize, Component)]
pub struct FallingTag;
#[derive(Clone, Copy, Debug, Default, Deserialize, Serialize, Component)]
pub struct DespawnWhenOutOfBoundsTag;
#[derive(Clone, Copy, Debug, Default, Deserialize, Serialize, Component)]
pub struct IntelligenceTag;

// --- Data Components ---
#[derive(Clone, Debug, Default, Deserialize, Serialize, Component)]
pub struct Movement {
    #[serde(default)]
    pub velocity: Vec3,
    pub max_movement_speed: f32,
}

#[derive(Clone, Copy, Debug, Default, Deserialize, Serialize, Component)]
pub struct Wander { pub angle: f32, pub radius: f32 }

pub type CreatureType = String;

#[derive(Clone, Debug, Default, Deserialize, Serialize, Component)]
pub struct Carcass { pub creature_type: CreatureType }

#[derive(Clone, Debug, Deserialize, Serialize, Component)]
pub struct Circle { pub radius: f32 }

// --- Combat ---
#[derive(Default, Debug, Clone, Deserialize, Serialize, Component)]
pub struct Health { pub max_health: f32, pub value: f32 }

#[derive(Default, Debug, Clone, Deserialize, Serialize, Component)]
pub struct Damage { pub damage: f32 }

#[derive(Default, Debug, Clone, Deserialize, Serialize, Component)]
pub struct Speed { pub attacks_per_second: f32 }

#[derive(Default, Debug, PartialEq, Eq, Clone, Deserialize, Serialize, Component)]
pub struct Cooldown { pub time_left: Duration }  // std::time::Duration

#[derive(Debug, PartialEq, Eq, Clone, Component)]
pub struct HasFaction(pub Entity);

#[derive(Default, Debug, PartialEq, Eq, Clone, Component)]
pub struct FactionPrey(pub Vec<Entity>);

/// RON デシリアライズ用の戦闘データまとめ型
#[derive(Default, Debug, Clone, Deserialize, Serialize)]
#[serde(default)]
#[serde(deny_unknown_fields)]
pub struct CombatData {
    pub health: Option<Health>,
    pub speed: Option<Speed>,
    pub damage: Option<Damage>,
    pub faction: Option<String>,
}

// --- Digestion ---
#[derive(Default, Debug, Clone, Deserialize, Serialize, Component)]
pub struct Digestion { pub nutrition_burn_rate: f32 }

#[derive(Default, Debug, Clone, Deserialize, Serialize, Component)]
pub struct Fullness { pub max: f32, pub value: f32 }

#[derive(Default, Debug, Clone, Deserialize, Serialize, Component)]
pub struct Nutrition { pub value: f32 }

/// RON デシリアライズ用の消化データまとめ型
#[derive(Default, Debug, Clone, Deserialize, Serialize)]
#[serde(default)]
#[serde(deny_unknown_fields)]
pub struct DigestionData {
    pub fullness: Option<Fullness>,
    pub digestion: Option<Digestion>,
    pub nutrition: Option<Nutrition>,
}

// --- Perception ---
#[derive(Default, Clone, Debug, Serialize, Deserialize, Component)]
#[serde(default)]
pub struct Perception { pub range: f32 }

#[derive(Default, Clone, Debug, Component)]
pub struct DetectedEntities { pub entities: HashSet<Entity> }  // std::collections::HashSet

// --- Swarm ---
#[derive(Clone, Debug, Default, Component)]
pub struct SwarmCenter { pub entities: Vec<Entity> }

#[derive(Clone, Debug, Default, Component)]
pub struct SwarmBehavior {
    pub swarm_center: Option<Entity>,
    pub attraction: f32,
    pub deviation: f32,
}
```

## データ駆動: CreatureDefinition

```rust
/// RON ファイルから生物を定義するデータ型
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
```

## リソース型

```rust
#[derive(Resource, Clone, Debug)]
pub struct WorldBounds { pub left: f32, pub right: f32, pub bottom: f32, pub top: f32 }

#[derive(Resource, Default, Clone, Debug)]
pub struct DebugConfig { pub visible: bool }

#[derive(Resource, Default, Clone, Debug, Deserialize, Serialize)]
pub struct Wind { pub wind: Vec2 }  // direction + magnitude を1つの Vec2 で表現

#[derive(Resource, Default)]
pub struct Factions(pub HashMap<String, Entity>);

#[derive(Resource)]
pub struct SpatialGrid {
    cell_size: f32,
    cells: HashMap<(i32, i32), HashSet<u32>, SpatialBuildHasher>,
    // query() は HashSet<u32> (entity indices) を返す
}

#[derive(Resource, Default)]
pub struct CreaturePrefabs {
    pub prefabs: HashMap<String, CreatureDefinition>,
}
```

## イベント型

```rust
#[derive(Event)] pub struct CollisionEvent { pub entity_a: Entity, pub entity_b: Entity }
#[derive(Event)] pub struct AttackEvent { pub attacker: Entity, pub defender: Entity }
#[derive(Event)] pub struct CreatureDeathEvent { pub deceased: Entity }
#[derive(Event)] pub struct CreatureSpawnEvent { pub creature_type: String, pub transform: Transform }
```

## SystemSet

```rust
#[derive(SystemSet, Debug, Clone, PartialEq, Eq, Hash)]
pub enum GameSet {
    Perception, Decision, Behavior, Movement,
    Metabolism, Combat, Death, Spawn, Experimental,
}
```

## nalgebra → glam 変換表

| nalgebra | glam (Bevy) |
|----------|-------------|
| `Vector3<f32>` | `Vec3` |
| `Vector2<f32>` | `Vec2` |
| `v.magnitude()` | `v.length()` |
| `v.norm_squared()` | `v.length_squared()` |
| `transform.prepend_translation_x(dx)` | `transform.translation.x += dx` |
| `transform.set_rotation_2d(angle)` | `transform.rotation = Quat::from_rotation_z(angle)` |
| `transform.translation()` | `transform.translation` (Vec3 直接) |
| `transform.global_matrix().column(3).xyz()` | `global_transform.translation()` |
