# Tomatui

**エディタの隣に、小さな集中タイマー。**

![作業・休憩・長い休憩を表示するTomatui](https://raw.githubusercontent.com/Hiro-Chiba/tomatui/main/assets/demo.gif)

[![Latest release](https://img.shields.io/github/v/release/Hiro-Chiba/tomatui?label=release)](https://github.com/Hiro-Chiba/tomatui/releases/latest)
[![Crates.io](https://img.shields.io/crates/v/tomatui)](https://crates.io/crates/tomatui)
[![CI](https://github.com/Hiro-Chiba/tomatui/actions/workflows/ci.yml/badge.svg)](https://github.com/Hiro-Chiba/tomatui/actions)
[![License: MIT](https://img.shields.io/badge/license-MIT-blue.svg)](LICENSE)

大きな時計、1行表示、集中の記録。ターミナルで使うポモドーロタイマーです。

[English](README.md) · [ダウンロード](https://github.com/Hiro-Chiba/tomatui/releases/latest) · [使い方ガイド](docs/usage_ja.md)

## インストール

```bash
cargo install --locked tomatui
```

Rust 1.93以降が必要です。Rustなしで使うなら、[macOS・Linux・Windows向けバイナリ](https://github.com/Hiro-Chiba/tomatui/releases/latest)をダウンロードできます。[詳しい手順](docs/usage_ja.md#使い始める)。

## 使い方

```bash
tomatui          # 保存した設定で起動
tomatui 30m      # 30分作業
tomatui 45m 10m  # 45分作業・10分休憩
tomatui -m       # 1行表示
```

初期設定は作業25分・休憩5分。4セッションごとに15分の長い休憩が入ります。

![Tomatuiの1行表示](https://raw.githubusercontent.com/Hiro-Chiba/tomatui/main/assets/minimal.gif)

*デモは待ち時間を省略しています。[撮影方法](assets/README.md)。*

| キー | 操作 |
| --- | --- |
| `Space` / `p` | 一時停止・再開 |
| `+` / `↑` | 1分追加 |
| `s` | スキップ |
| `q` | 終了 |

## 設定

一度保存すれば、あとは`tomatui`だけ。

```bash
tomatui config -w 30m -b 10m
tomatui config --on-end ask  # 次のフェーズの前に待つ
```

待機中は`Enter`で続行。自動で進めるなら`--on-end start`、1フェーズで終了するなら`--on-end quit`を使います。

## 統計

```bash
tomatui stats          # 今日の記録
tomatui stats history  # 直近1週間
```

![集中時間の記録例](https://raw.githubusercontent.com/Hiro-Chiba/tomatui/main/assets/stats.png)

*画像はサンプルです。完了した作業をPC内に保存し、スキップした作業は数えません。*

## Contributing

機能追加のアイデア、挙動の改善、不具合の報告・修正を歓迎しています。[Issue](https://github.com/Hiro-Chiba/tomatui/issues)や[Pull Request](https://github.com/Hiro-Chiba/tomatui/pulls)でお気軽にご提案ください。開発時は[テスト手順](docs/testing.md)をご覧ください。

<details>
<summary>詳しく見る</summary>

[全操作・設定](docs/usage_ja.md) · [変更履歴](CHANGELOG.md) · [設計](docs/architecture.md) · [リリース](docs/releasing.md)

</details>

[pomo](https://github.com/Bahaaio/pomo)を参考にしています。[MITライセンス](LICENSE)。
