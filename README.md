# Tomatui

[![Crates.io](https://img.shields.io/crates/v/tomatui)](https://crates.io/crates/tomatui)
[![CI](https://github.com/Hiro-Chiba/tomatui/actions/workflows/ci.yml/badge.svg)](https://github.com/Hiro-Chiba/tomatui/actions)
[![License: MIT](https://img.shields.io/badge/license-MIT-blue.svg)](LICENSE)

[Japanese / 日本語](README_ja.md)

[Changelog](CHANGELOG.md)

A terminal Pomodoro timer with stats tracking.

Work | Break | Long Break
:---:|:---:|:---:
![Work](https://raw.githubusercontent.com/Hiro-Chiba/tomatui/main/assets/work.png) | ![Break](https://raw.githubusercontent.com/Hiro-Chiba/tomatui/main/assets/break.png) | ![Long Break](https://raw.githubusercontent.com/Hiro-Chiba/tomatui/main/assets/long_break.png)

## Features

- Big-text timer display with progress bar
- Minimal one-line mode (`-m`) for small terminals
- Session tracking with visual dots
- Daily, weekly, and all-time statistics with persistent storage
- Configurable work and break durations and session count

## Supported OS

Linux, macOS, and Windows are supported. Tomatui requires Rust 1.93 or later.

## Installation

```bash
cargo install --locked tomatui
```

## Usage

```bash
tomatui start              # Rich TUI
tomatui start -m           # Minimal one-line mode
tomatui start -w 30 -b 10  # Custom work/break duration
tomatui stats              # Today's stats
tomatui stats history      # Last 7 days
tomatui stats summary      # All-time summary
tomatui config -w 30       # Save settings
tomatui config --reset     # Reset to defaults
```

## Keybindings

`q`/`Esc` quit | `p`/`Space` pause | `s` skip | `w` work | `b` break

## Configuration

Defaults are 25 minutes of work, 5 minutes of break, 15 minutes of long break, and 4 sessions. The JSON fields are `work_minutes`, `break_minutes`, `long_break_minutes`, and `sessions`.

Configuration is stored in the standard OS config directory under `tomatui/config.json`. This is `$XDG_CONFIG_HOME` or `~/.config` on Linux, `~/Library/Application Support` on macOS, and `%APPDATA%` on Windows. Run `tomatui config` to see the exact path.

## Statistics

Only completed work sessions are recorded. Skipped sessions are not included. Statistics are stored in the standard OS data directory under `tomatui/stats.json`, using `$XDG_DATA_HOME` or `~/.local/share` on Linux, `~/Library/Application Support` on macOS, and `%APPDATA%` on Windows.

## Notifications

Desktop notifications are best effort on macOS and Linux. Linux requires `notify-send`. Windows and systems without desktop notification support fall back to the terminal bell.

## Development

The repository uses Rust 1.98.0 for reproducible development. CI also verifies the declared minimum version, Rust 1.93.

```bash
cargo fmt --check
cargo clippy --locked --all-targets -- -D warnings
cargo test --locked
```

## License

MIT
