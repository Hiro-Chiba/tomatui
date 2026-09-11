# Tomatui

[![Crates.io](https://img.shields.io/crates/v/tomatui)](https://crates.io/crates/tomatui)
[![CI](https://github.com/Hiro-Chiba/tomatui/actions/workflows/ci.yml/badge.svg)](https://github.com/Hiro-Chiba/tomatui/actions)
[![License: MIT](https://img.shields.io/badge/license-MIT-blue.svg)](LICENSE)

エディタの隣に置いて、ターミナルを離れず集中できるポモドーロタイマーです。
小さなペインでは1行表示、広い画面では大きな時計を使えます。
完了した集中セッションはローカルに保存され、あとから振り返れます。

[OS別ダウンロード](https://github.com/Hiro-Chiba/tomatui/releases/latest) · [English](README.md) · [変更履歴](CHANGELOG.md)

![大きな残り時間とプログレスバーを表示する Tomatui](https://raw.githubusercontent.com/Hiro-Chiba/tomatui/main/assets/work.png)

1行表示でもカウントダウンでき、`p` で一時停止・再開、`q` で終了できます。

![1行表示の Tomatui がカウントダウンし、一時停止してから再開する実演](https://raw.githubusercontent.com/Hiro-Chiba/tomatui/main/assets/minimal.gif)

```bash
tomatui start -m       # エディタの隣で使う1行タイマー
tomatui start          # 大きな時計で表示
tomatui stats history  # 完了した集中セッションを振り返る
```

## Rust なしで試す

[最新リリース](https://github.com/Hiro-Chiba/tomatui/releases/latest)から、お使いの環境に合うファイルをダウンロードしてください。配布版の実行には Rust や Cargo のインストールは不要です。

| お使いの環境 | ダウンロード |
| --- | --- |
| macOS、Apple Silicon | [tomatui-aarch64-apple-darwin.tar.gz](https://github.com/Hiro-Chiba/tomatui/releases/latest/download/tomatui-aarch64-apple-darwin.tar.gz) |
| macOS、Intel | [tomatui-x86_64-apple-darwin.tar.gz](https://github.com/Hiro-Chiba/tomatui/releases/latest/download/tomatui-x86_64-apple-darwin.tar.gz) |
| Linux、x86-64 | [tomatui-x86_64-unknown-linux-gnu.tar.gz](https://github.com/Hiro-Chiba/tomatui/releases/latest/download/tomatui-x86_64-unknown-linux-gnu.tar.gz) |
| Windows、x86-64 | [tomatui-x86_64-pc-windows-msvc.zip](https://github.com/Hiro-Chiba/tomatui/releases/latest/download/tomatui-x86_64-pc-windows-msvc.zip) |

Linux の配布版には glibc 2.39 以降が必要です。古い glibc や musl を使う環境では、以下の Cargo の手順でお使いの環境向けにビルドしてください。

macOS・Linux はファイルを展開し、そのフォルダでターミナルを開いて実行します。

```bash
./tomatui start -m
```

Windows は ZIP を展開し、そのフォルダで PowerShell を開いて実行します。

```powershell
.\tomatui.exe start -m
```

`q` で終了、`p` で一時停止できます。どのフォルダからでも `tomatui` と入力して使いたい場合は、実行ファイルを `PATH` の通ったディレクトリに配置してください。

### Cargo でインストール

Rust をお使いの場合は、Rust 1.93 以降でビルドしてインストールできます。

```bash
cargo install --locked tomatui
tomatui start -m
```

## 特徴

- ビッグテキスト表示とプログレスバー
- 小さいターミナル向けのミニマル1行モード (`-m`)
- セッション進捗をドットで可視化
- 日別、週別、全期間の統計と永続保存
- 作業時間、休憩時間、セッション数のカスタマイズ

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

開発用toolchainは`rust-toolchain.toml`で固定しています。CIでは`Cargo.toml`に記載した最低対応バージョンも確認します。

```bash
cargo fmt --check
cargo clippy --locked --all-targets -- -D warnings
cargo test --locked
```

## ライセンス

MIT
