# Changelog

Notable changes to Tomatui are recorded here.

## [Unreleased]

### Added

- Launch the full timer with `tomatui` or one-line mode with `tomatui -m`. Existing `start` commands remain supported.
- Choose `ask`, `start`, or `quit` when a phase ends, with `--on-end` when launching or saving configuration. Existing settings keep automatic transitions.
- Add one minute to the current phase with `+` or the up arrow.
- Show a summary of completed work when the timer exits.

### Changed

- Simplified the full timer display and added a compact layout for smaller terminals.
- Use a thick block progress bar while preserving the original red, green, and blue phase colors.
- Refreshed both READMEs with a full timer demo, a standalone one-line demo, a statistics example, and clearer installation paths.

### Fixed

- Center the visible clock glyphs, labels, and controls consistently.
- Keep statistics errors visible and report them after restoring the terminal.

## [0.1.5] - 2026-09-12

### Changed

- Added a recording of the one-line timer, including pause, resume, and quit, to both READMEs.
- Documented the Linux binary's glibc 2.39 requirement and the Cargo installation alternative.

## [0.1.4] - 2026-09-12

### Added

- Prebuilt binaries for macOS (Apple Silicon and Intel), Linux (x86-64), and Windows (x86-64), with SHA256 checksums.

### Changed

- Put the editor-side timer workflow and Rust-free installation instructions at the start of both READMEs.

## [0.1.3] - 2026-09-12

### Changed

- Updated the development and release toolchain to Rust 1.98.1.

## [0.1.2] - 2026-08-30

### Fixed

- Limited statistics history requests to a safe ten-year range.
- Restored minimal-mode terminal settings before reporting an unexpected panic.

### Changed

- Updated all direct dependencies, including Ratatui 0.30, Crossterm 0.29, dirs 6, and tui-big-text 0.8.
- Migrated the crate to Rust 2024 while keeping Rust 1.93 as the tested minimum version.
- Pinned development and release checks to Rust 1.98.0 and updated GitHub Actions.
- Replaced duplicated application values with named constants without changing the CLI or configuration format.
- Standardized GitHub Releases with installation instructions, curated highlights, and generated change lists.

## [0.1.1] - 2026-08-30

### Fixed

- Corrected skip, pause, resume, manual phase switching, and session count transitions.
- Prevented invalid timer values and corrupt statistics from causing data loss or excessive work.
- Restored terminal state after normal I/O failures.

### Changed

- Reduced unnecessary TUI redraws and safely reaped notification processes.
- Added isolated persistence tests and expanded the test suite to 51 cases.
- Added Linux, macOS, and Windows CI coverage.
- Documented supported platforms, configuration, statistics, and development commands.

## [0.1.0]

- Initial crates.io release.

[Unreleased]: https://github.com/Hiro-Chiba/tomatui/compare/v0.1.5...HEAD
[0.1.5]: https://github.com/Hiro-Chiba/tomatui/releases/tag/v0.1.5
[0.1.4]: https://github.com/Hiro-Chiba/tomatui/releases/tag/v0.1.4
[0.1.3]: https://github.com/Hiro-Chiba/tomatui/releases/tag/v0.1.3
[0.1.2]: https://github.com/Hiro-Chiba/tomatui/releases/tag/v0.1.2
[0.1.1]: https://github.com/Hiro-Chiba/tomatui/releases/tag/v0.1.1
