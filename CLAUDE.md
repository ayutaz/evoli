# CLAUDE.md

This file provides guidance to Claude Code (claude.ai/code) when working with code in this repository.

## プロジェクト概要

Evoli は Bevy エンジン (v0.15) を使ったマイクロエコシステムシミュレーションゲーム (Rust)。複数の生物種が限られた空間内で共存するシミュレーションを行う。元は Amethyst エンジンで構築され、Bevy 0.15 へ移行済み。

## ビルド・実行コマンド

```bash
# ビルド・実行（Apple Silicon ネイティブ対応）
cargo build
cargo run

# テスト
cargo test

# リリースビルド
cargo build --release
```

### ビルド環境の注意点

- **Rust stable** (1.82.0+) が必要（Bevy 0.15 の MSRV）
- macOS Apple Silicon でネイティブ動作（x86_64 クロスコンパイル不要）
- LFS アセットが GitHub の帯域制限で取得不可の場合、`media.githubusercontent.com` 経由でダウンロード可能

## アーキテクチャ

Bevy の ECS (Entity Component System) パターンに従う。

### モジュール構成

- **`src/components/`** - ECSコンポーネント定義。`CreatureDefinition` が生物の全コンポーネントをまとめた定義型
- **`src/systems/`** - ゲームロジック。`behaviors/` にAI行動 (wander, seek, ricochet, obstacle avoidance)
- **`src/states/`** - ゲームステートマシン: Loading → Menu → InGame ⇄ Paused
- **`src/resources/`** - 共有リソース (WorldBounds, SpatialGrid, DebugConfig, Prefabs)
- **`src/utils/`** - spatial_hash などのユーティリティ

### ステートマシン

- `AppState`: Loading → Menu → InGame
- `GamePlayState` (SubState of InGame): Running ⇄ Paused
- 各ステートは Plugin パターン（`LoadingPlugin`, `MenuPlugin`, `MainGamePlugin`, `PauseMenuPlugin`）

### ゲームループ（GameSet）

`main_game.rs` で `GameSet` SystemSet を chain() で順序定義:
Perception → Decision → Behavior → Movement → Metabolism → Combat → Death → Spawn → Experimental

### データ駆動設計

- 生物の定義は `resources/prefabs/creatures/*.ron` で RON 形式で記述
- 3Dモデルは `resources/assets/` に glTF/GLB 形式で格納（Git LFS管理）

### 主要な生物システムの流れ

SpatialGrid → EntityDetection → QueryPredatorsAndPrey → Closest(Prey/Predator) → Seek/Avoid → Wander → Movement → Collision → Digestion → Combat → Death → Spawner

### experimental モジュール

`components/`, `systems/`, `resources/` それぞれに `experimental/` サブモジュールがあり、風 (wind)、重力 (gravity)、知覚 (perception)、topplegrass などの実験的機能を含む。
