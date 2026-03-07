# Tomatui

[![Crates.io](https://img.shields.io/crates/v/tomatui)](https://crates.io/crates/tomatui)
[![CI](https://github.com/Hiro-Chiba/tomatui/actions/workflows/ci.yml/badge.svg)](https://github.com/Hiro-Chiba/tomatui/actions)
[![License: MIT](https://img.shields.io/badge/license-MIT-blue.svg)](LICENSE)

Rust製のターミナルポモドーロタイマー。

Work | Break | Long Break
:---:|:---:|:---:
![Work](assets/work.png) | ![Break](assets/break.png) | ![Long Break](assets/long_break.png)

## 特徴

- ビッグテキスト表示とプログレスバーのリッチTUI
- 小さいターミナル向けのミニマル1行モード (`-m`)
- セッション進捗をドットで可視化
- 日別/週別/全期間の統計と永続保存
- macOS ネイティブ通知
- 作業/休憩時間・セッション数のカスタマイズ

## インストール

```bash
cargo install tomatui
```

## 使い方

```bash
tomatui start              # リッチTUI
tomatui start -m           # ミニマル1行モード
tomatui start -w 30 -b 10  # 作業/休憩時間を指定
tomatui stats              # 今日の統計
tomatui stats history      # 直近7日の履歴
tomatui stats summary      # 全期間サマリー
tomatui config -w 30       # 設定を保存
tomatui config --reset     # デフォルトに戻す
```

## キー操作

`q` 終了 | `p`/`Space` 一時停止 | `s` スキップ | `w` Work | `b` Break

## ライセンス

MIT
