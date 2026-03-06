# Evoli

Bevy エンジン (v0.15) を使ったマイクロ生態系シミュレーションゲームです。限られた同じ空間に生息する数種類の生物をシミュレートしています。

元は Amethyst エンジンで構築されたプロジェクトで、Bevy 0.15 への移行が完了しています。

## メディア

![evoli-screenshot](evoli-shot.png)

## インストール / プレイ

### 必要なもの

- [Git LFS](https://git-lfs.github.com/) - 3Dモデル等のアセットが Git LFS で管理されています
- Rust stable (1.82.0+) - インストールされていない場合は [rustup](https://rustup.rs/) を使用してください

### 実行

```bash
git clone https://github.com/nickmass/evoli.git
cd evoli
cargo run
```

リリースビルド（パフォーマンス最適化）で実行する場合：

```bash
cargo run --release
```

## プロファイリング

プロファイリング機能を有効にしてゲームを実行できます：

```bash
cargo run --release --features profiler
```

## ライセンス

デュアルライセンス：[ApacheライセンスまたはMITライセンス](LICENSE.md)から選択できます。
