# Architecture

Tomatui is a single binary. `Timer` owns the phase and countdown state, while `App` connects timer transitions to statistics and notifications. The TUI and minimal mode share the same `App` behavior.

## State transitions

The timer starts in Work at session 1. A completed Work phase enters Break, or Long Break when the configured session count has been reached. Break increments the session count before returning to Work. Long Break resets the session count to 1 before returning to Work.

Pause stops elapsed time from being applied until resume. Skip ends the current phase without recording a completed Work session. The `w` and `b` keys switch directly to Work and the appropriate break phase.

## Persistence

Configuration is stored as JSON in the OS config directory. Statistics are stored separately as JSON in the OS data directory. Missing files use defaults. A naturally completed Work phase records one pomodoro and its configured work duration, while skipped phases do not.
