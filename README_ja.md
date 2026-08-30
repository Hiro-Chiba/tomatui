# Tomatui

[![Crates.io](https://img.shields.io/crates/v/tomatui)](https://crates.io/crates/tomatui)
[![CI](https://github.com/Hiro-Chiba/tomatui/actions/workflows/ci.yml/badge.svg)](https://github.com/Hiro-Chiba/tomatui/actions)
[![License: MIT](https://img.shields.io/badge/license-MIT-blue.svg)](LICENSE)

[English](README.md)

[変更履歴](CHANGELOG.md)

Rust製のターミナルポモドーロタイマー。

Work | Break | Long Break
:---:|:---:|:---:
![Work](https://raw.githubusercontent.com/Hiro-Chiba/tomatui/main/assets/work.png) | ![Break](https://raw.githubusercontent.com/Hiro-Chiba/tomatui/main/assets/break.png) | ![Long Break](https://raw.githubusercontent.com/Hiro-Chiba/tomatui/main/assets/long_break.png)

## 特徴

- ビッグテキスト表示とプログレスバー
- 小さいターミナル向けのミニマル1行モード (`-m`)
- セッション進捗をドットで可視化
- 日別、週別、全期間の統計と永続保存
- 作業時間、休憩時間、セッション数のカスタマイズ

## 対応OS

Linux、macOS、Windowsに対応しています。Rust 1.93以降が必要です。

## インストール

```bash
cargo install --locked tomatui
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

`q`/`Esc` 終了 | `p`/`Space` 一時停止 | `s` スキップ | `w` Work | `b` Break

## 設定

デフォルトは作業25分、休憩5分、長い休憩15分、4セッションです。JSONのフィールドは`work_minutes`、`break_minutes`、`long_break_minutes`、`sessions`です。

設定はOS標準の設定ディレクトリ内の`tomatui/config.json`に保存されます。Linuxでは`$XDG_CONFIG_HOME`または`~/.config`、macOSでは`~/Library/Application Support`、Windowsでは`%APPDATA%`です。正確なパスは`tomatui config`で確認できます。

## 統計

完了したWorkセッションだけを記録し、スキップしたセッションは含みません。統計はOS標準のデータディレクトリ内の`tomatui/stats.json`に保存されます。Linuxでは`$XDG_DATA_HOME`または`~/.local/share`、macOSでは`~/Library/Application Support`、Windowsでは`%APPDATA%`です。

## 通知

macOSとLinuxのデスクトップ通知はbest effortです。Linuxでは`notify-send`が必要です。Windowsおよびデスクトップ通知を利用できない環境では、ターミナルベルを使用します。

## 開発

```bash
cargo fmt --check
cargo clippy --all-targets -- -D warnings
cargo test
```

## ライセンス

MIT
