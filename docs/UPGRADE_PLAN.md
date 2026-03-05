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

## フェーズA: Bevy 0.18 へ全面移行

### 目標
Amethyst を完全に廃止し、Bevy 0.18 へ移行する。

### ステータス: 計画策定完了 → 実装待ち

---

### A-1. 基盤構築（Cargo.toml / main.rs）

**作業内容:**
- [ ] Rust を最新 stable (1.93+) にアップグレード、Edition 2024 に変更
- [ ] Cargo.toml: amethyst を削除、bevy 0.18 を追加
- [ ] 追加クレート: `bevy_common_assets` (RONアセット), `ron` (デシリアライズ)
- [ ] main.rs: `App::new().add_plugins(DefaultPlugins)` ベースに書き直し
- [ ] AppState enum 定義: Loading / Menu / InGame
- [ ] GamePlayState SubStates: Running / Paused

**Bevy Cargo.toml 設定:**
```toml
[package]
name = "evolution-island"
version = "0.3.0"
edition = "2024"

[dependencies]
bevy = { version = "0.18", features = ["3d"] }
bevy_common_assets = { version = "0.x", features = ["ron"] }
rand = "0.9"
log = "0.4"
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
- [ ] タグコンポーネント 7個を `#[derive(Component)]` に変換
- [ ] データコンポーネント 17個を `#[derive(Component)]` に変換
- [ ] `PrefabData` / `#[prefab(Component)]` をすべて削除
- [ ] `CreaturePrefabData` → `CreatureDefinition` (Deserialize + Asset + TypePath)
- [ ] `CombatPrefabData` / `DigestionPrefabData` → serde のみ
- [ ] `HasFaction<String>` / `FactionPrey<String>` のカスタム PrefabData 実装を削除

---

### A-3. リソース移行（7個）

**変換ルール:**
| Amethyst | Bevy 0.18 |
|----------|-----------|
| `world.insert(T)` | `app.insert_resource(T)` / `commands.insert_resource(T)` |
| `Read<'s, T>` | `Res<T>` |
| `Write<'s, T>` / `ReadExpect` / `WriteExpect` | `ResMut<T>` |

**作業内容:**
- [ ] WorldBounds, DebugConfig, Wind, Factions → `#[derive(Resource)]`
- [ ] SpatialGrid → `#[derive(Resource)]` (`src/utils/spatial_hash.rs` のロジックはそのまま流用)
- [ ] CreaturePrefabs → `#[derive(Resource)]` + `HashMap<String, Handle<CreatureDefinition>>`
- [ ] UiPrefabRegistry → 廃止（コードベースUIに移行）

---

### A-4. アセット/Prefabシステム再設計（最大の課題）

**現行Amethyst Prefab体系:**
```
RON → PrefabLoaderSystemDesc → PrefabData derive → 自動コンポーネント挿入
```

**Bevy移行後:**
```
RON → bevy_common_assets RonAssetPlugin → CreatureDefinition Asset → 手動 Commands::spawn()
```

**作業内容:**
- [ ] `CreatureDefinition` Asset型を定義（現行RON構造を維持）
- [ ] `RonAssetPlugin::<CreatureDefinition>::new(&["creature.ron"])` を登録
- [ ] スポーンヘルパー関数: `spawn_creature(commands, definition, asset_server)` を作成
- [ ] glTF読み込み: `AssetServer::load("path.glb")` → `SceneRoot(handle)`
- [ ] Faction初期化: RONから読み込み → `Factions` Resourceにマッピング
- [ ] `HasFaction<String>` → `HasFaction<Entity>` 変換をスポーン時に実行
- [ ] UI Prefab (5ファイル) → 廃止（A-7でコードベースUIに移行）

**RONファイルの扱い:**
- `resources/prefabs/creatures/*.ron` → 構造を維持しつつ `AssetPrefab<GltfSceneAsset>` 部分を修正
- `resources/prefabs/factions.ron` → カスタムアセットとしてロード
- `resources/prefabs/ui/*.ron` → 廃止
- `resources/wind.ron` → カスタムアセットまたは直接ロード

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
1. [ ] コアシステム: movement, collision, enforce_bounds
2. [ ] 知覚: spatial_grid, entity_detection
3. [ ] 意思決定: query_predators_and_prey, closest, seek
4. [ ] 行動: wander, ricochet, obstacle
5. [ ] 代謝: digestion, starvation
6. [ ] 戦闘: cooldown, find_attack, perform_attack
7. [ ] 死亡: death_by_health, carcass
8. [ ] スポーン: debug_spawn_trigger, swarm_spawn, topplegrass_spawn, creature_spawner
9. [ ] 実験的: toppling, gravity, out_of_bounds, wind_control, swarm_behavior, swarm_center
10. [ ] カメラ: camera_movement
11. [ ] デバッグ（7個）: DebugLinesComponent → Gizmos
12. [ ] UI: main_game_ui

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
- [ ] AppState / GamePlayState enum 定義
- [ ] LoadingState: `OnEnter` でアセットロード開始、`Update` + `run_if` でプログレス監視、完了時 `NextState::set`
- [ ] MenuState: `OnEnter` でUI構築、ボタンクリックで遷移
- [ ] MainGameState: `OnEnter` でエンティティ生成（カメラ、ライト、初期生物）、`OnExit` でクリーンアップ
- [ ] PauseMenu: `GamePlayState::Paused` + SubStates で実装
- [ ] ゲームシステムは `.run_if(in_state(GamePlayState::Running))` で条件付き実行
- [ ] `DespawnOnExitState` でステート離脱時の自動エンティティ削除

---

### A-7. UI再構築（コードベース）

**Amethystの5つのUI RON → Bevyコードに変換:**

- [ ] メニューUI: タイトル + Play/Quit ボタン
- [ ] ゲーム内UI: Pause/Speed Up/Slow Down/Menu ボタン
- [ ] ポーズメニューUI: Resume/Quit ボタン
- [ ] マーカーコンポーネントでボタン特定 (`PlayButton`, `PauseButton` 等)
- [ ] `Interaction` + `Changed` フィルタでクリック検出

---

### A-8. レンダリング・カメラ・ライト

**作業内容:**
- [ ] カメラ: `Camera3d` + `OrthographicProjection` (scaling_mode で zoom 調整)
- [ ] ライト: `DirectionalLight` + `AmbientLight`
- [ ] デバッグ描画: `DebugLinesComponent` → `Gizmos` (circle, line, sphere)
- [ ] glTFシーン: `SceneRoot(asset_server.load("path.glb"))`

---

### A-9. オーディオ

- [ ] BGM: `AudioPlayer::new(asset_server.load("ambient.ogg"))` + `PlaybackSettings::LOOP`
- [ ] macOS互換性: Bevy はwgpu/cpal経由なので CoreAudio問題が解消される可能性あり

---

### A-10. Math / Transform移行

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
