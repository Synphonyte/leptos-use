# Changelog

All notable changes to this crate are documented in this file.

The format is based on [Keep a Changelog](https://keepachangelog.com/en/1.0.0/),
and this crate adheres to [Semantic Versioning](https://semver.org/spec/v2.0.0.html).

## [Unreleased]

## [0.20.2](https://github.com/Synphonyte/leptos-use/compare/leptos-use-websocket-v0.20.1...leptos-use-websocket-v0.20.2) - 2026-10-21

### Fixed

- `reconnect_limit` now limits consecutive reconnect attempts instead of capping them over the hook's lifetime.
- `ready_state` is updated before `on_open`/`on_error`/`on_close` run, so sends from `on_open` go through, and a callback that calls `open()` no longer has its new connection replaced after `reconnect_interval`.
- Event handler closures are handed to the JS GC instead of leaked.

## [0.20.1](https://github.com/Synphonyte/leptos-use/compare/leptos-use-websocket-v0.20.0...leptos-use-websocket-v0.20.1) - 2026-10-20

### Added

- Re-export the shared types used in the crate's public API, so they don't need another crate to be named.

## [0.20.0] - 2026-10-20

### Added

- Initial release. This crate used to be part of [`leptos-use`](https://crates.io/crates/leptos-use) (feature `use_websocket`).
