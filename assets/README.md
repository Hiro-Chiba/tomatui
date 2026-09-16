# Demo recordings

Run these commands from the repository root with VHS, FFmpeg, ttyd, and FiraCode Nerd Font Mono installed.

```bash
cargo build --locked --bins --examples
mkdir -p target/recordings
vhs assets/demo.tape
vhs assets/minimal.tape
python3 assets/shorten-demos.py
vhs assets/stats.tape
vhs assets/long-break.tape
```

The timer tapes expect saved settings with one minute of work, one minute of rest, four sessions, and `on_end` set to `ask`. Back up existing settings before preparing this recording configuration with `tomatui config -w 1 -b 1 -l 15 -s 4 --on-end ask`, and restore them after recording. These are sample preferences, not new application defaults.

The main recording runs the real binary with those saved settings. It shows elapsed progress, pause and resume, a one minute extension, and the confirmation after a completed break, and the blue long break after skipping the intervening phases. The source tape runs at four times the recorded speed. `shorten-demos.py` then cuts and accelerates selected sections for the final README loop. Waiting time is skipped, as disclosed in both READMEs. The recording switches away from work before it completes and exits shortly after starting the next work phase, so it does not add statistics.

The source minimal recording runs at normal speed before editing. The final loops are about eight seconds for the full display and four seconds for minimal mode. The blue long break stays visible for about two and a half seconds. Both show launch within the first second, then move quickly through the useful states. Timer tapes write source MP4 files under `target/recordings`; the editing script produces the final GIF and MP4 files under `assets`. Verify the command, phase changes, and final frame after regenerating because capture timing can vary. The terminal frame follows the Hiro Chiba profile README, while the terminal theme remains Catppuccin Mocha to preserve Tomatui’s original colors. Recordings use a width of 1600 pixels and 30 frames per second. Both recordings show the short launch commands `tomatui` and `tomatui -m` at a plain `$` prompt so viewers can see how to start the timer. Personal usernames and hostnames are omitted.

The statistics preview uses fictional data for the week ending September 16, 2026 in JST. It calls the same history renderer as the application and never reads or writes personal statistics. The preview is explicitly marked as sample data in both READMEs.

The tapes set the terminal colors explicitly for consistent recordings. The small temporary GIFs for static statistics and long break images are written under `/tmp`.
