**注意:** Evoliは現在、積極的にメンテナンスされていません。より最新のサンプルゲームについては、[最近更新されたショーケースゲーム](https://github.com/search?q=topic%3Ashowcase+org%3Aamethyst&type=Repositories)のいずれかをご覧ください。

# Evoli
Amethystエンジンの公式ショーケースプロジェクトとして、段階的に設計・開発されたマイクロ生態系シミュレーションゲームです。現在のバージョン（v0.2以降）では、限られた同じ空間に生息する数種類の生物をシミュレートしています。

現在のゲームデザインや、これまでの目標と経緯の詳細については、[紹介記事](https://community.amethyst.rs/t/evoli-introduction/770)をお読みください。

## メディア

![may-10](https://raw.githubusercontent.com/amethyst/evoli/master/evoli-shot.png)

## インストール / プレイ

リポジトリをクローンする際は、ほとんどのアセットがそこに保存されているため、[Git LFS](https://git-lfs.github.com/)がインストールされていることを確認してください。

Linuxでコンパイルする場合は、まずいくつかの依存関係をインストールする必要があります。これらはAmethystエンジンのコンパイルと実行に必要です。[Amethyst README](https://github.com/amethyst/amethyst#dependencies)の手順に従ってください。

Cargoがインストールされていることを確認し（インストールされていない場合は[rustup](https://rustup.rs/)を使用してください）、以下を実行してください：

```
cargo run
```

問題が発生した場合は、こちらか http://discord.gg/amethyst の#showcase-gameチャンネルで報告してください。

## プロファイリング
Amethystが使用しているのと同じプロファイリングライブラリを使用しています。以下のコマンドでゲームを実行してください：
```
cargo run --release --features profiler
```
その後、クラッシュせずにゲームを終了すると、`thread_profile.json`というファイルが生成されます。
そのファイルの使い方については、Amethystの[エンジンのプロファイリング](https://github.com/amethyst/amethyst/blob/master/docs/CONTRIBUTING.md#profiling-the-engine)の手順を参照してください。

コードにプロファイリングマーカーを追加する方法の例については、コード内で`profile_scope`を検索してください。

## 参加するには

- [サイトマップドキュメント](https://community.amethyst.rs/t/evoli-sitemap/771) - 必読資料とコミュニケーションツールの一覧。
- [開発規約](https://community.amethyst.rs/t/evoli-development-conventions/783)
- [コントリビューションドキュメント](https://community.amethyst.rs/t/evoli-is-ready-for-contributions/815)

## ライセンス

デュアルライセンス：[ApacheライセンスまたはMITライセンス](https://github.com/amethyst/evoli/blob/master/LICENSE.md)から選択できます。
