# Pomo

A terminal Pomodoro timer with a rich TUI, minimal mode, and daily statistics.

Built for developers who live in the terminal.

## Quick Start

```bash
cargo install --path .
pomo start
```

## Features

| Feature | Description |
|---|---|
| Rich TUI | Centered UI with large digit clock, progress bar, session dots |
| Minimal Mode | Single-line display for tight layouts (`-m`) |
| Persistent Config | Customize durations via `pomo config` |
| Daily Statistics | Track pomodoros, work time, and history |
| Phase Switching | Manually switch between Work/Break anytime |
| Bell Notification | Terminal bell on phase completion |

## Usage

### Start a Timer

```bash
pomo start              # Rich TUI with config defaults
pomo start -m           # Minimal one-line mode
pomo start -w 30 -b 10  # Override: 30min work, 10min break
```

### Keybindings

| Key | Action |
|---|---|
| `q` / `Esc` | Quit |
| `p` / `Space` | Pause / Resume |
| `s` | Skip current phase |
| `w` | Switch to Work |
| `b` | Switch to Break |

### Statistics

```bash
pomo stats              # Today's stats (default)
pomo stats today        # Today's pomodoros and work time
pomo stats history      # Last 7 days breakdown
pomo stats history -d 30 # Last 30 days
pomo stats summary      # All-time totals and averages
pomo stats clear        # Reset all statistics
```

### Configuration

Settings are persisted to `~/.config/pomo/config.json` (macOS: `~/Library/Application Support/pomo/`).

```bash
pomo config             # Show current settings
pomo config -w 30       # Set work to 30 minutes
pomo config -b 10       # Set break to 10 minutes
pomo config -l 20       # Set long break to 20 minutes
pomo config -s 6        # Set sessions before long break to 6
pomo config --reset     # Reset to defaults
```

**Defaults:**

| Setting | Value |
|---|---|
| Work | 25 min |
| Break | 5 min |
| Long Break | 15 min |
| Sessions | 4 |

CLI flags (`-w`, `-b`, etc.) on `pomo start` temporarily override config without changing saved settings.

## Data Storage

| Data | Path (macOS) |
|---|---|
| Config | `~/Library/Application Support/pomo/config.json` |
| Statistics | `~/Library/Application Support/pomo/stats.json` |

On Linux, paths follow XDG conventions (`~/.config/pomo/`, `~/.local/share/pomo/`).

## Tech Stack

- **Language:** Rust
- **TUI:** [ratatui](https://github.com/ratatui/ratatui) + [tui-big-text](https://github.com/joshka/tui-widgets) + crossterm
- **CLI:** clap (derive)
- **Persistence:** serde_json + dirs

## Building from Source

```bash
git clone https://github.com/Hiro-Chiba/pomo.git
cd pomo
cargo build --release
```

Binary will be at `target/release/pomo`.

## License

MIT
