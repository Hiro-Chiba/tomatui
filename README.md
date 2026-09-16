# Tomatui

**A little focus timer beside your editor.**

![Tomatui: work, break, and long break](https://raw.githubusercontent.com/Hiro-Chiba/tomatui/main/assets/demo.gif)

[![Latest release](https://img.shields.io/github/v/release/Hiro-Chiba/tomatui?label=release)](https://github.com/Hiro-Chiba/tomatui/releases/latest)
[![Crates.io](https://img.shields.io/crates/v/tomatui)](https://crates.io/crates/tomatui)
[![CI](https://github.com/Hiro-Chiba/tomatui/actions/workflows/ci.yml/badge.svg)](https://github.com/Hiro-Chiba/tomatui/actions)
[![License: MIT](https://img.shields.io/badge/license-MIT-blue.svg)](LICENSE)

A terminal Pomodoro timer with a big clock, one-line mode, and local focus history.

[日本語](README_ja.md) · [Download](https://github.com/Hiro-Chiba/tomatui/releases/latest) · [User guide](docs/usage.md)

## Install

```bash
cargo install --locked tomatui
```

Requires Rust 1.93+. No Rust? [Download a binary](https://github.com/Hiro-Chiba/tomatui/releases/latest) for macOS, Linux, or Windows. [Installation details](docs/usage.md#get-started).

## Usage

```bash
tomatui          # Start with saved settings
tomatui 30m      # 30 minutes of work
tomatui 45m 10m  # 45 minutes of work, 10 minutes of rest
tomatui -m       # One-line mode
```

Defaults to 25-minute work, 5-minute breaks, and a 15-minute long break every four sessions.

![One-line timer](https://raw.githubusercontent.com/Hiro-Chiba/tomatui/main/assets/minimal.gif)

*Demos skip waiting time. [Recording details](assets/README.md).*

| Key | Action |
| --- | --- |
| `Space` / `p` | Pause or resume |
| `+` / `↑` | Add one minute |
| `s` | Skip |
| `q` | Quit |

## Settings

Save once, then launch with `tomatui`:

```bash
tomatui config -w 30m -b 10m
tomatui config --on-end ask  # Wait before the next phase
```

Press `Enter` to continue when waiting. Use `--on-end start` for automatic transitions or `--on-end quit` to finish after one phase.

## Statistics

```bash
tomatui stats          # Today
tomatui stats history  # Past week
```

![Sample focus history](https://raw.githubusercontent.com/Hiro-Chiba/tomatui/main/assets/stats.png)

*Sample data. Completed work is saved locally; skipped work is not counted.*

<details>
<summary>More</summary>

[All controls and settings](docs/usage.md) · [Changelog](CHANGELOG.md) · [Architecture](docs/architecture.md) · [Releasing](docs/releasing.md)

</details>

Inspired by [pomo](https://github.com/Bahaaio/pomo). [MIT license](LICENSE).
