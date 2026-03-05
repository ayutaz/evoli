# CLAUDE.md

This file provides guidance to Claude Code (claude.ai/code) when working with code in this repository.

## プロジェクト概要

Evoli は Amethyst エンジン (v0.15) を使ったマイクロエコシステムシミュレーションゲーム (Rust)。複数の生物種が限られた空間内で共存するシミュレーションを行う。**現在はメンテナンスされていない。**

## ビルド・実行コマンド

```bash
# macOS (Apple Silicon) でのビルド・実行 — x86_64 クロスコンパイル + Rosetta 2 で実行
CFLAGS="-arch x86_64" CXXFLAGS="-arch x86_64" cargo build --target x86_64-apple-darwin
arch -x86_64 ./target/x86_64-apple-darwin/debug/evolution-island

# リリースビルド + プロファイリング
CFLAGS="-arch x86_64" CXXFLAGS="-arch x86_64" cargo build --release --target x86_64-apple-darwin --features profiler
```

### ビルド環境の注意点

- **Rust 1.75.0** を使用（`rust-toolchain` で指定済み）
- macOS では `Cargo.toml` の features を `vulkan` から `metal` に変更済み
- Apple Silicon では aarch64 の `winit v0.19.5` BOOL 型非互換のため **x86_64 ターゲットでクロスコンパイル** が必要
- C/C++ ライブラリに `CFLAGS/CXXFLAGS="-arch x86_64"` が必須
- LFS アセットが GitHub の帯域制限で取得不可の場合、`media.githubusercontent.com` 経由でダウンロード可能

## アーキテクチャ

Amethyst の ECS (Entity Component System) パターンに従う。

### モジュール構成

- **`src/components/`** - ECSコンポーネント定義。`CreaturePrefabData` が生物の全コンポーネントをまとめた prefab データ型
- **`src/systems/`** - ゲームロジック。`behaviors/` にAI行動 (wander, seek, ricochet, obstacle avoidance)
- **`src/states/`** - ゲームステートマシン: Loading → Menu → MainGame ⇄ PauseMenu
- **`src/resources/`** - 共有リソース (WorldBounds, SpatialGrid, DebugConfig, Prefabs, Audio)
- **`src/utils/`** - spatial_hash などのユーティリティ

### ゲームループ（MainGameState）

`main_game.rs` で3つの Dispatcher を順次実行:
1. **メインディスパッチャ** - 知覚→意思決定→移動→衝突→消化→戦闘→死亡→スポーンの順
2. **デバッグディスパッチャ** - DebugConfig.visible が true の場合のみ実行
3. **UIディスパッチャ** - ゲームUI更新

### データ駆動設計

- 生物の定義は `resources/prefabs/creatures/*.ron` で RON 形式で記述
- UI も `resources/prefabs/ui/*.ron` で定義
- 3Dモデルは `resources/assets/` に glTF/GLB 形式で格納（Git LFS管理）

### 主要な生物システムの流れ

SpatialGrid → EntityDetection → QueryPredatorsAndPrey → Closest(Prey/Predator) → Seek/Avoid → Wander → Movement → Collision → Digestion → Combat → Death → Spawner

### experimental モジュール

`components/`, `systems/`, `resources/` それぞれに `experimental/` サブモジュールがあり、風 (wind)、重力 (gravity)、知覚 (perception)、topplegrass などの実験的機能を含む。
