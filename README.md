# Pomo

[Japanese / 日本語](README_ja.md)

A terminal Pomodoro timer built with Rust.

Work | Break | Long Break
:---:|:---:|:---:
![Work](assets/work.png) | ![Break](assets/break.png) | ![Long Break](assets/long_break.png)

## Install

```bash
cargo install --path .
```

## Usage

```bash
pomo start              # Rich TUI
pomo start -m           # Minimal one-line mode
pomo start -w 30 -b 10  # Custom work/break duration
pomo stats              # Today's stats
pomo stats history      # Last 7 days
pomo stats summary      # All-time summary
pomo config -w 30       # Save settings
pomo config --reset     # Reset to defaults
```

## Keybindings

`q` quit | `p`/`Space` pause | `s` skip | `w` work | `b` break

## License

MIT
