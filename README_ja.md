# Tomatui

Rust製のターミナルポモドーロタイマー。

Work | Break | Long Break
:---:|:---:|:---:
![Work](assets/work.png) | ![Break](assets/break.png) | ![Long Break](assets/long_break.png)

## インストール

```bash
cargo install --path .
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
