# Testing

Run these commands from the repository root.

```bash
cargo fmt --check
cargo clippy --locked --all-targets -- -D warnings
cargo test --locked
cargo build --locked
python3 tests/terminal_e2e.py --binary target/debug/tomatui
```

Rust tests include subprocess checks for CLI help, version, and invalid arguments on Linux, macOS, and Windows. The Python terminal suite uses only the standard library and requires a Unix PTY, so it runs on Linux and macOS. It launches the actual binary with isolated home and data directories. It does not use personal settings or statistics. Natural-completion cases use real one-minute timers rather than production test hooks.

The terminal suite covers both displays, duration arguments, pause and resume, extensions, phase changes, resizing, quit keys, terminal restoration, saved settings, and completed-work persistence. Windows console interaction is not covered by the Unix PTY suite. Rust unit tests and CLI subprocess tests still run on Windows.

## Performance checks

The timer deadline tests check the wait duration at fractional-second boundaries and after delayed drawing. The input-state test checks that paused and completed timers do not busy-poll and ignored keys do not request redraws. The terminal suite also checks ignored input while paused.

For a local comparison on macOS, optimized builds of v0.2.0 and the new implementation were run in a 100-column PTY, paused, left idle for three seconds, then sent 100 unused `x` keys and a `+` key. Unused input produced 10,800 output bytes before the change and zero afterward. Both builds responded to `+` in under one millisecond in that single run. These are local observations, not cross-platform latency guarantees. The deterministic improvement is fewer timeout wakeups and no redraw for ignored input, rather than a promised CPU percentage.

The rapid-pause regression test resumes and pauses 24 times with at least 50 milliseconds of active time per cycle. In a local before-and-after check, v0.2.0 remained at `02:00` while the corrected binary reached `01:59`.
