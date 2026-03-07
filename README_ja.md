# Pomo

ターミナルで動作するポモドーロタイマー。リッチTUI、ミニマルモード、日別統計機能を搭載。

ターミナルに住む開発者のために作りました。

## クイックスタート

```bash
cargo install --path .
pomo start
```

## 機能一覧

| 機能 | 説明 |
|---|---|
| リッチTUI | 大きな数字時計、プログレスバー、セッションドット付きの中央配置UI |
| ミニマルモード | コンパクトな1行表示 (`-m`) |
| 設定の永続化 | `pomo config` でタイマー時間をカスタマイズ |
| 日別統計 | ポモドーロ数・作業時間の記録と履歴表示 |
| フェーズ切替 | Work/Breakをいつでも手動で切り替え可能 |
| ベル通知 | フェーズ完了時にターミナルベルで通知 |

## 使い方

### タイマーの起動

```bash
pomo start              # リッチTUI（設定ファイルのデフォルト値）
pomo start -m           # ミニマルモード（1行表示）
pomo start -w 30 -b 10  # 一時的に作業30分・休憩10分で起動
```

### キー操作

| キー | 動作 |
|---|---|
| `q` / `Esc` | 終了 |
| `p` / `Space` | 一時停止 / 再開 |
| `s` | 現在のフェーズをスキップ |
| `w` | Workフェーズに切替 |
| `b` | Breakフェーズに切替 |

### 統計

```bash
pomo stats              # 今日の統計（デフォルト）
pomo stats today        # 今日のポモドーロ数と作業時間
pomo stats history      # 直近7日の日別履歴
pomo stats history -d 30 # 直近30日の日別履歴
pomo stats summary      # 全期間の合計・平均
pomo stats clear        # 統計データをリセット
```

### 設定

設定ファイルは `~/.config/pomo/config.json`（macOS: `~/Library/Application Support/pomo/`）に保存されます。

```bash
pomo config             # 現在の設定を表示
pomo config -w 30       # 作業時間を30分に変更
pomo config -b 10       # 休憩時間を10分に変更
pomo config -l 20       # 長休憩を20分に変更
pomo config -s 6        # 長休憩までのセッション数を6に変更
pomo config --reset     # デフォルトに戻す
```

**デフォルト値:**

| 設定 | 値 |
|---|---|
| 作業時間 | 25分 |
| 休憩時間 | 5分 |
| 長休憩 | 15分 |
| セッション数 | 4 |

`pomo start` の CLIフラグ（`-w`, `-b` 等）は保存された設定を変更せず、一時的にオーバーライドします。

## データ保存先

| データ | パス (macOS) |
|---|---|
| 設定 | `~/Library/Application Support/pomo/config.json` |
| 統計 | `~/Library/Application Support/pomo/stats.json` |

Linux では XDG 規約に従います（`~/.config/pomo/`, `~/.local/share/pomo/`）。

## 技術スタック

- **言語:** Rust
- **TUI:** [ratatui](https://github.com/ratatui/ratatui) + [tui-big-text](https://github.com/joshka/tui-widgets) + crossterm
- **CLI:** clap (derive)
- **永続化:** serde_json + dirs

## ソースからビルド

```bash
git clone https://github.com/Hiro-Chiba/pomo.git
cd pomo
cargo build --release
```

バイナリは `target/release/pomo` に出力されます。

## ライセンス

MIT
