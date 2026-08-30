# Changelog

Notable changes to Tomatui are recorded here.

## [Unreleased]

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

[Unreleased]: https://github.com/Hiro-Chiba/tomatui/compare/v0.1.2...HEAD
[0.1.2]: https://github.com/Hiro-Chiba/tomatui/releases/tag/v0.1.2
[0.1.1]: https://github.com/Hiro-Chiba/tomatui/releases/tag/v0.1.1
