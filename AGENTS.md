# Tomatui

## Overview

Tomatui is a small terminal Pomodoro timer written in Rust. It provides a full TUI, a minimal one-line mode, and local statistics.

## Docs

- [Architecture](docs/architecture.md)
- [Releasing](docs/releasing.md)

## Rules

- Keep the application small and avoid speculative features or abstractions.
- Preserve the CLI and JSON file formats whenever possible.
- Add dependencies only when the standard library and current crates are insufficient.
- Run formatting, linting, and tests before completing a change.
