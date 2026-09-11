# Tomatui

[![Crates.io](https://img.shields.io/crates/v/tomatui)](https://crates.io/crates/tomatui)
[![CI](https://github.com/Hiro-Chiba/tomatui/actions/workflows/ci.yml/badge.svg)](https://github.com/Hiro-Chiba/tomatui/actions)
[![License: MIT](https://img.shields.io/badge/license-MIT-blue.svg)](LICENSE)

Keep a Pomodoro timer beside your editor, without leaving the terminal.
Use a single line in a small terminal pane, or switch to the big-text display.
Completed focus sessions are saved locally so you can review your progress.

[Download for your OS](https://github.com/Hiro-Chiba/tomatui/releases/latest) · [日本語](README_ja.md) · [Changelog](CHANGELOG.md)

![Tomatui focus timer with a large countdown and progress bar](https://raw.githubusercontent.com/Hiro-Chiba/tomatui/main/assets/work.png)

The one-line mode counts down, pauses with `p`, and resumes with `p`. Press `q` to quit.

![Tomatui one-line timer counting down, pausing, and resuming](https://raw.githubusercontent.com/Hiro-Chiba/tomatui/main/assets/minimal.gif)

```bash
tomatui start -m       # One-line timer beside your editor
tomatui start          # Big-text timer
tomatui stats history  # Review completed focus sessions
```

## Try it without Rust

Download the archive for your computer from the [latest release](https://github.com/Hiro-Chiba/tomatui/releases/latest). The binaries run without installing Rust or Cargo.

| Your computer | Archive |
| --- | --- |
| macOS, Apple Silicon | [tomatui-aarch64-apple-darwin.tar.gz](https://github.com/Hiro-Chiba/tomatui/releases/latest/download/tomatui-aarch64-apple-darwin.tar.gz) |
| macOS, Intel | [tomatui-x86_64-apple-darwin.tar.gz](https://github.com/Hiro-Chiba/tomatui/releases/latest/download/tomatui-x86_64-apple-darwin.tar.gz) |
| Linux, x86-64 | [tomatui-x86_64-unknown-linux-gnu.tar.gz](https://github.com/Hiro-Chiba/tomatui/releases/latest/download/tomatui-x86_64-unknown-linux-gnu.tar.gz) |
| Windows, x86-64 | [tomatui-x86_64-pc-windows-msvc.zip](https://github.com/Hiro-Chiba/tomatui/releases/latest/download/tomatui-x86_64-pc-windows-msvc.zip) |

The Linux binary requires glibc 2.39 or newer. For older glibc versions or musl-based systems, use the Cargo installation below to build for your environment.

On macOS or Linux, extract the archive and open a terminal in the extracted folder:

```bash
./tomatui start -m
```

On Windows, extract the ZIP, open PowerShell in the extracted folder, and run:

```powershell
.\tomatui.exe start -m
```

Press `q` to quit or `p` to pause. To run `tomatui` from any folder, move the executable to a directory on your `PATH`.

### Install with Cargo

If you already use Rust, build and install with Rust 1.93 or later:

```bash
cargo install --locked tomatui
tomatui start -m
```

## Features

- Big-text timer display with progress bar
- Minimal one-line mode (`-m`) for small terminals
- Session tracking with visual dots
- Daily, weekly, and all-time statistics with persistent storage
- Configurable work and break durations and session count

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

The development toolchain is pinned in `rust-toolchain.toml`. CI also verifies the minimum version declared in `Cargo.toml`.

```bash
cargo fmt --check
cargo clippy --locked --all-targets -- -D warnings
cargo test --locked
```

## License

MIT
