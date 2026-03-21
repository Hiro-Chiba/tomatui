# Tomatui

[![Crates.io](https://img.shields.io/crates/v/tomatui)](https://crates.io/crates/tomatui)
[![CI](https://github.com/Hiro-Chiba/tomatui/actions/workflows/ci.yml/badge.svg)](https://github.com/Hiro-Chiba/tomatui/actions)
[![License: MIT](https://img.shields.io/badge/license-MIT-blue.svg)](LICENSE)

[Japanese / 日本語](README_ja.md)

A terminal Pomodoro timer with stats tracking.

Work | Break | Long Break
:---:|:---:|:---:
![Work](assets/work.png) | ![Break](assets/break.png) | ![Long Break](assets/long_break.png)

## Features

- Big-text timer display with progress bar
- Minimal one-line mode (`-m`) for small terminals
- Session tracking with visual dots
- Daily/weekly/all-time statistics with persistent storage
- Desktop notifications (macOS, Linux)
- Configurable work/break durations and session count

## Install

```bash
cargo install tomatui
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

`q` quit | `p`/`Space` pause | `s` skip | `w` work | `b` break

## License

MIT
