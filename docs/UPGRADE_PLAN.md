# バージョンアップグレード計画

調査日: 2026-03-05

## 現在のバージョン状況

| 項目 | 現在 | 最新 (2026年3月) | 差分 |
|------|------|-----------------|------|
| Rust | 1.75.0 | 1.93.1 | 18バージョン遅れ |
| Edition | 2018 | 2024 | 2世代遅れ (2018 → 2021 → 2024) |
| Amethyst | 0.15.0 | 0.15.3 (アーカイブ済) | メンテナンス終了 |
| Bevy (後継) | - | 0.18 | 新規移行が必要 |

### 直接依存クレート

| クレート | 現在 | 最新 | 備考 |
|---------|------|------|------|
| amethyst | 0.15.0 | 0.15.3 | 2022年4月にGitHubリポジトリがアーカイブ |
| rand | 0.7.3 | 0.9系 | メジャーバージョンアップでAPI変更あり |
| log | 0.4.8 | 0.4.2x | マイナーアップデート、互換性あり |
| serde | 1.0.114 | 1.0.200+ | マイナーアップデート、互換性あり |
| smart-default | 0.6.0 | 0.7系 | マイナーアップデート |
| thread_profiler | 0.3.0 | 0.3.x | 変更なし |

### プロジェクト規模（影響範囲）

- ソースコード: 51ファイル / 4,172行
- ECSコンポーネント: 24個（すべてAmethyst PrefabData使用）
- システム: 41個（すべてAmethyst System trait実装）
- RON Prefabファイル: 16個
- Amethyst import: 47箇所
- リソース: 7個

---

## フェーズB: 最小限アップデート（現行Amethyst維持）

### 目標
Amethyst 0.15 エコシステム内で可能な限りの依存関係アップデートを行う。

### 作業内容

- [x] Amethyst 0.15.0 → 0.15.3 へアップデート
- [x] log 0.4.8 → 0.4.29, serde 1.0.114 → 1.0.228, smart-default 0.6.0 → 0.7.1 に更新
- [x] rand 0.7.3 は維持（Amethyst内部依存との整合性のため）
- [x] Cargo.lock の推移的依存を `cargo update` で更新（約170クレート）
- [x] MSRV非互換クレートをピン留め（backtrace→0.3.71, rayon→1.10.0, rayon-core→1.12.1, coreaudio-sys→0.2.16）
- [x] Camera API変更対応（Projection削除→Camera::orthographic直接呼び出し）
- [x] cargo fix による warning 自動修正（swarm_behavior, perception, main, combat）
- [x] ビルド確認（x86_64-apple-darwin ターゲット）
- [x] 実行確認（Amethyst 0.15.3 として正常起動）

### 制約
- Rust 1.75.0 維持（Amethyst 0.15.x + 推移的依存のMSRV制約）
- Edition 2018 維持（Amethyst 0.15 が 2018 Edition 前提）
- rand 0.7.3 維持（Amethyst内部で rand 0.7 に依存）
- coreaudio-sys 0.2.16 にピン留め（0.2.17 は Edition 2024 必須）
- backtrace 0.3.71 にピン留め（0.3.76 は Rust 1.82+ 必須）
- rayon 1.10.0 / rayon-core 1.12.1 にピン留め（1.11+ は Rust 1.80+ 必須）

### ステータス: 完了

---

## フェーズA: Bevy へ全面移行

### 目標
Amethyst を完全に廃止し、Bevy へ移行する。

### ステータス: 完了（2026-03-05）

> **計画との差分:**
> - Bevy **0.15** を使用（計画時点では 0.18 を想定していたが、移行開始時の安定版 0.15 を採用）
> - Edition **2021** を使用（2024 ではなく）
> - Rust stable **1.82.0+**（1.93+ ではなく、Bevy 0.15 の MSRV に準拠）
> - `bevy_common_assets` は不使用（`ron` クレートで直接デシリアライズ）
> - `rand` は **0.8** を使用（0.9 ではなく）
> - オーディオ（A-9）は**未実装**（BgmHandle は削除済み、将来対応）

---

### A-1. 基盤構築（Cargo.toml / main.rs）

**作業内容:**
- [x] Rust を stable (1.82.0+) にアップグレード、Edition 2021 に変更
- [x] Cargo.toml: amethyst を削除、bevy 0.15 を追加
- [x] 追加クレート: `ron` (デシリアライズ) ※ `bevy_common_assets` は不使用
- [x] main.rs: `App::new().add_plugins(DefaultPlugins)` ベースに書き直し
- [x] AppState enum 定義: Loading / Menu / InGame
- [x] GamePlayState SubStates: Running / Paused

**実際の Cargo.toml 設定:**
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

---

### A-2. コンポーネント移行（24個）

**変換ルール:**
| Amethyst | Bevy 0.18 |
|----------|-----------|
| `impl Component for T { type Storage = NullStorage<Self> }` | `#[derive(Component)]` |
| `impl Component for T { type Storage = DenseVecStorage<Self> }` | `#[derive(Component)]` |
| `impl Component for T { type Storage = HashMapStorage<Self> }` | `#[derive(Component)] #[component(storage = "SparseSet")]` |
| `#[derive(PrefabData)] #[prefab(Component)]` | 削除（`#[derive(Component, Deserialize)]` のみ） |

**作業内容:**
- [x] タグコンポーネント 7個を `#[derive(Component)]` に変換
- [x] データコンポーネント 17個を `#[derive(Component)]` に変換
- [x] `PrefabData` / `#[prefab(Component)]` をすべて削除
- [x] `CreaturePrefabData` → `CreatureDefinition` (Deserialize のみ、serde で直接デシリアライズ)
- [x] `CombatPrefabData` / `DigestionPrefabData` → serde のみ
- [x] `HasFaction<String>` / `FactionPrey<String>` → `HasFaction(Entity)` / `FactionPrey(Vec<Entity>)` タプル構造体に変換

---

### A-3. リソース移行（7個）

**変換ルール:**
| Amethyst | Bevy 0.18 |
|----------|-----------|
| `world.insert(T)` | `app.insert_resource(T)` / `commands.insert_resource(T)` |
| `Read<'s, T>` | `Res<T>` |
| `Write<'s, T>` / `ReadExpect` / `WriteExpect` | `ResMut<T>` |

**作業内容:**
- [x] WorldBounds, DebugConfig, Wind, Factions → `#[derive(Resource)]`
- [x] SpatialGrid → `#[derive(Resource)]` (`src/utils/spatial_hash.rs` のロジックはそのまま流用)
- [x] CreaturePrefabs → `#[derive(Resource)]` + `HashMap<String, CreatureDefinition>`（Handle不使用、直接保持）
- [x] UiPrefabRegistry → 廃止（コードベースUIに移行）

---

### A-4. アセット/Prefabシステム再設計（最大の課題）

**現行Amethyst Prefab体系:**
```
RON → PrefabLoaderSystemDesc → PrefabData derive → 自動コンポーネント挿入
```

**Bevy移行後（実際）:**
```
RON → ron::from_str → CreatureDefinition → CreaturePrefabs Resource → 手動 Commands::spawn()
```

**作業内容:**
- [x] `CreatureDefinition` serde型を定義（RON構造をフラット化して維持）
- [x] `ron` クレートで直接デシリアライズ（`bevy_common_assets` / `RonAssetPlugin` は不使用）
- [x] スポーンヘルパー: `spawn_creature()` で `CreatureDefinition` からコンポーネントを手動挿入
- [x] glTF読み込み: `AssetServer::load("path.glb")` → `SceneRoot(handle)`
- [x] Faction初期化: RONから読み込み → `Factions` Resourceにマッピング
- [x] `HasFaction(Entity)` / `FactionPrey(Vec<Entity>)` をスポーン時に設定
- [x] UI Prefab (5ファイル) → 廃止（A-7でコードベースUIに移行）

**RONファイルの扱い（実績）:**
- `resources/prefabs/creatures/*.ron` → フラット構造に簡略化（PrefabData/GltfSceneAsset 部分を削除）
- `resources/prefabs/factions.ron` → `ron::from_str` で直接ロード
- `resources/prefabs/ui/*.ron` → 廃止
- `resources/wind.ron` → `ron::from_str` で直接ロード

---

### A-5. システム移行（41個）

**変換ルール:**
| Amethyst | Bevy 0.18 |
|----------|-----------|
| `impl<'s> System<'s> for T { type SystemData; fn run() }` | `fn system_name(query: Query<...>, res: Res<...>)` |
| `(storages).join()` | `for (a, b) in &query` |
| `Entities<'s>` | `Query<Entity>` |
| `EventChannel<T>` + `ReaderId` | `EventReader<T>` / `EventWriter<T>` (Bevy 0.17では Message に改名) |
| `LazyUpdate` | `Commands` |
| `BitSet` | `EntityHashSet` |
| `ClosestSystem::<T>` (ジェネリック) | `fn closest_system::<T>(...)` 関数 |

**SystemSet定義（Dispatcher→SystemSet変換）:**
```rust
#[derive(SystemSet, Debug, Clone, PartialEq, Eq, Hash)]
enum GameSet {
    Perception,    // SpatialGrid → EntityDetection
    Decision,      // QueryPredatorsAndPrey → Closest → Seek
    Behavior,      // Ricochet → Wander
    Movement,      // Movement → Collision → EnforceBounds
    Metabolism,    // Digestion → Starvation
    Combat,        // Cooldown → FindAttack → PerformAttack
    Death,         // DeathByHealth → Carcass
    Spawn,         // SpawnTriggers → CreatureSpawner
    Experimental,  // Topplegrass, Gravity, Wind, Swarm, OutOfBounds
}

#[derive(SystemSet, Debug, Clone, PartialEq, Eq, Hash)]
enum DebugSet { Debug }

#[derive(SystemSet, Debug, Clone, PartialEq, Eq, Hash)]
enum UiSet { GameUi }
```

**作業内容（優先順位付き）:**
1. [x] コアシステム: movement, collision, enforce_bounds
2. [x] 知覚: spatial_grid, entity_detection
3. [x] 意思決定: query_predators_and_prey, closest, seek
4. [x] 行動: wander, ricochet, obstacle
5. [x] 代謝: digestion, starvation
6. [x] 戦闘: cooldown, find_attack, perform_attack
7. [x] 死亡: death_by_health, carcass
8. [x] スポーン: debug_spawn_trigger, swarm_spawn, topplegrass_spawn, creature_spawner
9. [x] 実験的: toppling, gravity, out_of_bounds, wind_control, swarm_behavior, swarm_center
10. [x] カメラ: camera_movement
11. [x] デバッグ（7個）: DebugLinesComponent → Gizmos
12. [x] UI: main_game_ui

**イベント移行:**
| イベント | Amethyst | Bevy 0.18 |
|---------|----------|-----------|
| CollisionEvent | `EventChannel` | `Event` / `EventWriter` / `EventReader` |
| AttackEvent | `EventChannel` | `Event` |
| CreatureDeathEvent | `EventChannel` | `Event` |
| CreatureSpawnEvent | `EventChannel` | `Event` |

---

### A-6. ステートマシン移行

**現行 (Amethyst SimpleState):**
```
LoadingState → MenuState → MainGameState ⇄ PauseMenuState
```

**Bevy移行後:**
```rust
#[derive(States, Default, Debug, Clone, PartialEq, Eq, Hash)]
enum AppState {
    #[default]
    Loading,
    Menu,
    InGame,
}

#[derive(SubStates, Default, Debug, Clone, PartialEq, Eq, Hash)]
#[source(AppState = AppState::InGame)]
enum GamePlayState {
    #[default]
    Running,
    Paused,
}
```

**作業内容:**
- [x] AppState / GamePlayState enum 定義
- [x] LoadingState: `OnEnter` でアセットロード開始、`Update` + `run_if` でプログレス監視、完了時 `NextState::set`
- [x] MenuState: `OnEnter` でUI構築、ボタンクリックで遷移
- [x] MainGameState: `OnEnter` でエンティティ生成（カメラ、ライト、初期生物）、`OnExit` でクリーンアップ
- [x] PauseMenu: `GamePlayState::Paused` + SubStates で実装
- [x] ゲームシステムは `.run_if(in_state(GamePlayState::Running))` で条件付き実行
- [x] `StateScoped` でステート離脱時の自動エンティティ削除

---

### A-7. UI再構築（コードベース）

**Amethystの5つのUI RON → Bevyコードに変換:**

- [x] メニューUI: タイトル + Play/Quit ボタン
- [x] ゲーム内UI: Pause/Speed Up/Slow Down/Menu ボタン
- [x] ポーズメニューUI: Resume/Quit ボタン
- [x] マーカーコンポーネントでボタン特定 (`PlayButton`, `PauseButton` 等)
- [x] `Interaction` + `Changed` フィルタでクリック検出

---

### A-8. レンダリング・カメラ・ライト

**作業内容:**
- [x] カメラ: `Camera3d` + `OrthographicProjection` (scaling_mode で zoom 調整)
- [x] ライト: `DirectionalLight` + `AmbientLight`
- [x] デバッグ描画: `DebugLinesComponent` → `Gizmos` (circle, line, sphere)
- [x] glTFシーン: `SceneRoot(asset_server.load("path.glb"))`

---

### A-9. オーディオ（未実装）

- [ ] BGM: `AudioPlayer::new(asset_server.load("ambient.ogg"))` + `PlaybackSettings::LOOP`
- [ ] macOS互換性: Bevy はwgpu/cpal経由なので CoreAudio問題が解消される可能性あり

> **注記:** オーディオは移行時に未実装のまま。Amethyst時代の `BgmHandle` は削除済み。`resources/assets/ambient.ogg` は残存しており、将来的に Bevy のオーディオ API で実装可能。

---

### A-10. Math / Transform移行（完了）

**nalgebra → glam 変換:**
| nalgebra (Amethyst) | glam (Bevy) |
|---------------------|-------------|
| `Vector3<f32>` | `Vec3` |
| `Vector2<f32>` | `Vec2` |
| `Point3<f32>` | `Vec3` |
| `Rotation3<f32>` | `Quat` |
| `Matrix4<f32>` | `Mat4` |
| `v.magnitude()` | `v.length()` |
| `v.normalize()` | `v.normalize()` (同じ) |
| `v.dot(&w)` | `v.dot(w)` |
| `transform.prepend_translation_x(dx)` | `transform.translation.x += dx` |
| `transform.set_rotation_2d(angle)` | `transform.rotation = Quat::from_rotation_z(angle)` |
| `transform.global_matrix()` | `GlobalTransform::compute_matrix()` |

---

### 主なリスク

1. **Bevy pre-1.0**: breaking changesが3-4ヶ月ごとに発生（公式Migration Guide提供あり）
2. **Amethyst→Bevy公式移行ガイドなし**: Thetawave, krABMagaの先行事例を参考にする
3. **Prefab再設計が最大の工数不確定要素**: bevy_common_assetsで既存RON構造を活用する方針で軽減
4. **nalgebra→glam変換**: 数学ライブラリの違いによるバグ混入リスク

### 参考プロジェクト
- [Thetawave](https://github.com/thetawavegame/thetawave) — Amethyst→Bevy完全移行の実例
- [krABMaga](https://github.com/krABMaga/krABMaga) — ABMフレームワーク、Amethyst→Bevy移行PR #9
- [rust-ecosystem-simulation](https://github.com/bones-ai/rust-ecosystem-simulation) — Bevyエコシステムシミュレーション
- [bevy_boids](https://github.com/tandalesc/bevy_boids) — Bevy + Quadtree空間分割

### 推奨追加クレート
- `bevy_common_assets` — RONファイルからカスタムアセット読み込み
- `bevy_spatial` or 独自spatial_hash — 空間インデックス（既存コードを移植可能）
- `avian` (オプション) — ECSネイティブの物理/衝突検出

---

## 技術調査メモ

### Amethyst の現状
- 最終バージョン: 0.15.3
- GitHubリポジトリ: 2022年4月18日にアーカイブ（読み取り専用）
- ECSは specs を使用（Legion への移行は未完了のまま停止）
- 元開発者の一部はBevyに合流

### Bevy 0.18 の主な特徴
- ECSの人間工学的優位性: システムは普通のRust関数として記述
- wgpuベースレンダリング
- コードファースト設計
- 活発なコミュニティ（Rustゲームエンジン最大）
- 3-4ヶ月ごとのリリースサイクル
- Feature Collections (0.18): `3d`, `2d`, `ui` などの高レベルfeature
- MSRV: 最新stable Rust (1.83+程度)
- Edition: 2024

### Amethyst vs Bevy API対応表

| 概念 | Amethyst (specs) | Bevy 0.18 |
|------|----------------|-------------|
| Component定義 | `impl Component { type Storage }` | `#[derive(Component)]` |
| System定義 | `impl System<'s> { type SystemData; fn run() }` | `fn system(query: Query<...>)` |
| Resource | `world.insert(T)` + `Read<'s, T>` | `app.insert_resource(T)` + `Res<T>` |
| Entity生成 | `world.create_entity().with(C).build()` | `commands.spawn((C1, C2))` |
| イテレーション | `(s1, s2).join()` | `for (a, b) in &query` |
| イベント | `EventChannel<T>` + `ReaderId` | `EventReader<T>` / `EventWriter<T>` |
| 遅延操作 | `LazyUpdate` | `Commands` |
| ステート | `SimpleState` + `Trans` | `States` + `NextState<T>` |
| Prefab | `PrefabData` + RON | `Asset` + `AssetLoader` + `Commands` |
| レンダリング | Rendy (独自) | wgpu ベース |
| UI | RON Prefab + UiBundle | コードベース (`Node`, `Button`, `Text`) |
| デバッグ描画 | `DebugLinesComponent` | `Gizmos` |
| 数学 | nalgebra | glam |
| メンテナンス | 停止 (2022年~) | 活発 |
