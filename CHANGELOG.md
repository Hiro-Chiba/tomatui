# Changelog

Notable changes to Tomatui are recorded here.

## [Unreleased]

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

[Unreleased]: https://github.com/Hiro-Chiba/tomatui/compare/v0.1.1...HEAD
[0.1.1]: https://github.com/Hiro-Chiba/tomatui/releases/tag/v0.1.1
