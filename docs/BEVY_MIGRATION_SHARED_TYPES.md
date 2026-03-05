# Bevy 移行: 共有型定義（全エージェント共通コンテキスト）

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
```

注意: Bevy 0.15 を使用（0.18はまだリリースされていない可能性があるため、確実に存在する安定版を使用）

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
#[derive(Component, Default)] pub struct CreatureTag;
#[derive(Component, Default)] pub struct RicochetTag;
#[derive(Component, Default)] pub struct AvoidObstaclesTag;
#[derive(Component, Default)] pub struct TopplegrassTag;
#[derive(Component, Default)] pub struct FallingTag;
#[derive(Component, Default)] pub struct DespawnWhenOutOfBoundsTag;
#[derive(Component, Default)] pub struct IntelligenceTag;

// --- Data Components ---
#[derive(Component, Default, Clone, Debug, Deserialize, Serialize)]
pub struct Movement { pub velocity: Vec3, pub max_movement_speed: f32 }

#[derive(Component, Default, Clone, Debug, Deserialize, Serialize)]
pub struct Wander { pub angle: f32, pub radius: f32 }

#[derive(Component, Default, Clone, Debug, Deserialize, Serialize)]
pub struct Carcass { pub prefab_name: String }

#[derive(Component, Default, Clone, Debug, Deserialize, Serialize)]
pub struct Circle { pub radius: f32 }

// --- Combat ---
#[derive(Component, Default, Clone, Debug, Deserialize, Serialize)]
pub struct Health { pub value: f32 }

#[derive(Component, Default, Clone, Debug, Deserialize, Serialize)]
pub struct Damage { pub value: f32 }

#[derive(Component, Default, Clone, Debug, Deserialize, Serialize)]
pub struct Speed { pub value: f32 }

#[derive(Component, Default, Clone, Debug, Deserialize, Serialize)]
pub struct Cooldown { pub timer: f32, pub value: f32 }

#[derive(Component, Clone, Debug)]
pub struct HasFaction(pub Entity);

#[derive(Component, Clone, Debug)]
pub struct FactionPrey(pub Vec<Entity>);

// --- Digestion ---
#[derive(Component, Default, Clone, Debug, Deserialize, Serialize)]
pub struct Digestion { pub nutrition_burn_rate: f32 }

#[derive(Component, Default, Clone, Debug, Deserialize, Serialize)]
pub struct Fullness { pub value: f32, pub max: f32 }

#[derive(Component, Default, Clone, Debug, Deserialize, Serialize)]
pub struct Nutrition { pub value: f32 }

// --- Perception ---
#[derive(Component, Default, Clone, Debug, Deserialize, Serialize)]
pub struct Perception { pub range: f32 }

#[derive(Component, Default, Clone, Debug)]
pub struct DetectedEntities { pub entities: Vec<Entity> }

// --- Swarm ---
#[derive(Component, Default, Clone, Debug)]
pub struct SwarmCenter { pub center: Vec3 }

#[derive(Component, Default, Clone, Debug, Deserialize, Serialize)]
pub struct SwarmBehavior { pub speed: f32, pub radius: f32 }
```

## リソース型

```rust
#[derive(Resource)]
pub struct WorldBounds { pub left: f32, pub right: f32, pub bottom: f32, pub top: f32 }

#[derive(Resource, Default)]
pub struct DebugConfig { pub visible: bool }

#[derive(Resource, Default, Deserialize, Serialize)]
pub struct Wind { pub direction: Vec2, pub strength: f32 }

#[derive(Resource, Default)]
pub struct Factions(pub HashMap<String, Entity>);

#[derive(Resource)]
pub struct SpatialGrid { /* existing spatial_hash logic */ }

#[derive(Resource, Default)]
pub struct CreaturePrefabs(pub HashMap<String, CreatureDefinition>);
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
