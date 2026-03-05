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

## フェーズA: Bevy 0.18 へ全面移行（フェーズB完了後に実施）

### 目標
Amethyst を完全に廃止し、Bevy 0.18 へ移行する。

### 作業項目

| フェーズ | 作業内容 | 推定工数 |
|---------|---------|---------|
| A-1. 基盤構築 | Cargo.toml書き換え、Bevy 0.18導入、Edition 2024、Rust最新化 | 0.5日 |
| A-2. ECSコンポーネント | 24コンポーネントをBevy ECS形式に書き換え | 2-3日 |
| A-3. システム移行 | 41システムを関数ベースに書き換え | 3-5日 |
| A-4. State管理 | Loading/Menu/MainGame/Pause をBevy States形式に | 1-2日 |
| A-5. Prefab/アセット | RON PrefabをBevy Scene/Assetシステムに | 2-3日 |
| A-6. レンダリング/UI | Rendy→wgpu、Amethyst UI→Bevy UI | 2-3日 |
| A-7. オーディオ | bevy_audio への移行 | 0.5日 |
| A-8. テスト・調整 | 動作確認、デバッグ | 2-3日 |

### Bevy移行の主なリスク
- Bevy自体がpre-1.0でbreaking changesが定期的に発生
- Amethyst→Bevyの公式移行ガイドは存在しない
- PrefabData + RON駆動の設計はBevyに直接対応する仕組みがない
- glTF/GLBアセットはBevyでもネイティブサポート（再作成不要）

### ステータス: 未着手（フェーズB完了待ち）

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

### Amethyst vs Bevy 比較

| 項目 | Amethyst (0.15) | Bevy (0.18) |
|------|----------------|-------------|
| ECS | specs (構造体ベースSystem) | Bevy ECS (関数ベースSystem) |
| Prefab | RON + カスタム PrefabData | Scene / Asset システム |
| State管理 | State マシン (push/pop) | States + Run Conditions |
| レンダリング | Rendy (独自) | wgpu ベース |
| メンテナンス | 停止 (2022年~) | 活発 |
