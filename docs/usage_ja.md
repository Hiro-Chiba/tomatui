# 使い方ガイド

[READMEに戻る](../README_ja.md)

## 使い始める

Rust 1.93以降をお使いなら、インストールしてすぐに起動できます。

```bash
cargo install --locked tomatui
tomatui
```

Rustを使わない場合は、[OSに合うバイナリをダウンロード](https://github.com/Hiro-Chiba/tomatui/releases/latest)してください。

<details>
<summary>バイナリを選んで実行する</summary>

| お使いの環境 | ダウンロード |
| --- | --- |
| macOS、Apple Silicon | [tomatui-aarch64-apple-darwin.tar.gz](https://github.com/Hiro-Chiba/tomatui/releases/latest/download/tomatui-aarch64-apple-darwin.tar.gz) |
| macOS、Intel | [tomatui-x86_64-apple-darwin.tar.gz](https://github.com/Hiro-Chiba/tomatui/releases/latest/download/tomatui-x86_64-apple-darwin.tar.gz) |
| Linux、x86-64 | [tomatui-x86_64-unknown-linux-gnu.tar.gz](https://github.com/Hiro-Chiba/tomatui/releases/latest/download/tomatui-x86_64-unknown-linux-gnu.tar.gz) |
| Windows、x86-64 | [tomatui-x86_64-pc-windows-msvc.zip](https://github.com/Hiro-Chiba/tomatui/releases/latest/download/tomatui-x86_64-pc-windows-msvc.zip) |

ファイルを展開し、そのフォルダでターミナルを開きます。macOS・Linuxでは`./tomatui`、WindowsではPowerShellで`.\tomatui.exe`を実行してください。`q`で終了できます。実行ファイルを`PATH`の通ったディレクトリに置くと、どのフォルダからでも`tomatui`で起動できます。

Linuxの配布版にはglibc 2.39以降が必要です。古いglibcやmuslを使う環境では、Cargoでお使いの環境向けにビルドしてください。配布済みバイナリの実行にRustやCargoは不要です。

</details>

<details>
<summary>ソースからビルドする</summary>

Rust 1.93以降で、次のコマンドを実行します。

```bash
git clone https://github.com/Hiro-Chiba/tomatui.git
cd tomatui
cargo install --path . --locked
tomatui
```

取得したソースの内容が`tomatui`コマンドとしてインストールされます。開発用toolchainは`rust-toolchain.toml`で固定しています。

</details>

## 集中のための、小さな居場所

ターミナル単体でも、エディタの隣の小さなペインでも使えます。1行表示でも、大きな時計と同じタイマー、キー操作、統計保存を利用できます。

```bash
tomatui          # 保存した設定で起動
tomatui 30m      # 30分作業
tomatui 45m 10m  # 45分作業・10分休憩
tomatui -m       # 1行表示
tomatui 1h -m    # 1行表示で1時間作業
```

![一時停止・再開・1分延長を行う Tomatui の1行表示](https://raw.githubusercontent.com/Hiro-Chiba/tomatui/main/assets/minimal.gif)

上と同じ撮影用設定で、待ち時間を省略した約4秒のデモです。[MP4で見る](https://raw.githubusercontent.com/Hiro-Chiba/tomatui/main/assets/minimal.mp4)。

## 次の休憩は、自分のタイミングで

起動するとすぐに作業が始まり、初期設定では作業と休憩が自動で進みます。フェーズが完了した`00:00`で待つなら`ask`、完了時に終了するなら`quit`を選べます。待機中は`Enter`または`s`で次のフェーズを開始します。タイマーが終わる前にもう少し続けたいときは、`+`または`↑`で1分追加できます。

好みを一度保存すれば、普段は`tomatui`または`tomatui -m`だけで起動できます。

```bash
tomatui config --on-end ask       # 一度だけ保存：区切りごとに待つ
tomatui                          # 保存済みの設定で起動
```

普段の時間を変えるなら`tomatui config -w 30 -b 10`を使います。その回だけ変える場合は、`tomatui --on-end quit`や`tomatui 30m`で保存済み設定を上書きできます。

時間は整数の分（`30`・`30m`）か時間（`1h`）で指定できます。省略した時間には保存済み設定を使い、起動コマンドで設定を書き換えることはありません。`tomatui --help`でも使用例を確認できます。

初期設定は作業25分、休憩5分、4セッションごとに長い休憩15分です。終了すると、その起動中に完了した作業のサマリーを表示します。

| キー | 操作 |
| --- | --- |
| `p` / `Space` | 一時停止・再開 |
| `+` / `↑` | 現在のフェーズに1分追加 |
| `s` | 現在のフェーズをスキップ、待機中なら次へ進む |
| `Enter` | 待機中なら次へ進む |
| `w` / `b` | 作業・休憩へ切り替え。同じフェーズのキーなら最初からやり直し |
| `q` / `Esc` / `Ctrl+C` | 終了 |

`ask`の`00:00`待機中は、次へ進む操作と終了だけが有効です。スキップすると、完了時の設定にかかわらず次のフェーズへ直接移ります。

## 終わった仕事を、振り返る

完了した作業セッションをローカルに保存します。今日の合計、直近1週間の履歴、これまでの累計をコマンドで確認できます。

```bash
tomatui stats                   # 今日の記録
tomatui stats history           # 直近7日
tomatui stats history --days 30  # 直近30日
tomatui stats summary           # 全期間のサマリー
```

![サンプルの1週間について、完了した作業セッションと集中時間を表示する Tomatui の統計](https://raw.githubusercontent.com/Hiro-Chiba/tomatui/main/assets/stats.png)

画像はサンプルデータです。完了した作業の時間には`+`で追加した分も含めます。スキップした作業や途中で終了した作業は記録しません。スキップしても作業・休憩の周期上の位置は進むため、セッション表示は完了数ではなく、その周期の何回目かを表します。

<details>
<summary>設定・保存先・通知</summary>

`tomatui config`で設定と正確な設定ファイルのパスを確認できます。`tomatui config -w 30 -b 10`で時間、`tomatui config --on-end ask`で完了時の動作を保存できます。初期設定に戻すには`tomatui config --reset`を使います。起動時のオプションは、その回だけ保存済み設定を上書きします。従来の`tomatui start`と`tomatui start -m`も引き続き使えます。

設定はOS標準の設定ディレクトリ内の`tomatui/config.json`に保存します。フィールドは`work_minutes`、`break_minutes`、`long_break_minutes`、`sessions`、`on_end`です。`on_end`は`start`（初期設定）、`ask`、`quit`から選べます。このフィールドを持たない既存の設定でも、自動遷移を維持します。

統計はOS標準のデータディレクトリ内の`tomatui/stats.json`に保存します。`tomatui stats clear`を実行すると、保存済み統計をすべて即座に削除します。

| OS | 設定ディレクトリ | データディレクトリ |
| --- | --- | --- |
| Linux | `$XDG_CONFIG_HOME`または`~/.config` | `$XDG_DATA_HOME`または`~/.local/share` |
| macOS | `~/Library/Application Support` | `~/Library/Application Support` |
| Windows | `%APPDATA%` | `%APPDATA%` |

macOS・Linuxのデスクトップ通知は、環境が対応している場合に送信します。Linuxでは`notify-send`が必要です。Windowsやデスクトップ通知を使えない環境では、ターミナルベルを使います。

</details>

<details>
<summary>開発</summary>

タイマーや保存の仕組みは[設計](architecture.md)、公開手順は[リリース](releasing.md)を参照してください。CIでは`Cargo.toml`に記載した最低対応Rustバージョンも確認します。GIFとサンプル統計画像の再生成手順は[デモの撮影方法](https://raw.githubusercontent.com/Hiro-Chiba/tomatui/main/assets/README.md)にあります。

```bash
cargo fmt --check
cargo clippy --locked --all-targets -- -D warnings
cargo test --locked
```

</details>

