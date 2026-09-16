# Architecture

Tomatui is a single binary. `Timer` owns the phase and countdown state, while `App` connects timer transitions to statistics and notifications. The TUI and minimal mode share the same `App` behavior. Running `tomatui` starts the TUI and `tomatui -m` starts minimal mode. Both load saved settings and accept optional overrides. The explicit `start` command remains supported. Positional durations such as `tomatui 45m 10m` override work and break for one run. Durations accept whole minutes, with optional `m`, or whole hours with `h`. Specifying the same duration positionally and by flag is an error.

## State transitions

The timer starts in Work at session 1. A completed Work phase enters Break, or Long Break when the configured session count has been reached. Break increments the session count before returning to Work. Long Break resets the session count to 1 before returning to Work.

Pause stops elapsed time from being applied until resume. Skip ends the current phase without recording a completed Work session. The `w` and `b` keys switch directly to Work and the appropriate break phase.

The `on_end` setting chooses what follows a natural completion. `start` advances immediately and remains the default for compatibility with existing JSON files. `ask` keeps the completed phase at zero until Enter or `s` advances. `quit` exits after that phase. Completion is recorded and announced only once. Skips bypass the end action and preserve pause state.

The `+` and Up keys add one minute to the remaining and total phase duration, including while paused. Both updates are checked for overflow before either is applied. During the completion prompt, pause, extension, and direct phase switches are ignored. The `q`, Escape, and Ctrl+C keys exit in both modes. Both interfaces delegate key events to `App`.

## Persistence

Configuration is stored as JSON in the OS config directory. Statistics are stored separately as JSON in the OS data directory. Missing files use defaults. A naturally completed Work phase records one pomodoro and its total planned duration, including added minutes. Skipped phases and manual phase switches do not record work. A per-run completion summary also counts finished work when persistence fails and reports how many sessions were not saved. The UI exposes the persistence error without writing into the active terminal display.

## Display colors

Keep the original phase palette. Work uses `#EB5757`, Break uses `#6FCF97`, and Long Break uses `#569CD6`. Demo recordings use the original Catppuccin Mocha terminal theme.

## Event loop

Both displays wait with `crossterm::event::poll` until the next visible second boundary. The timeout subtracts time spent drawing so rendering cannot accumulate clock drift. Paused timers and completion prompts use a 30-second idle timeout. Keyboard and resize events wake the poll immediately. The loop applies elapsed time before input, processes skips immediately, and exits without an extra wait after natural completion in quit mode. Ignored keys do not trigger redraws.
