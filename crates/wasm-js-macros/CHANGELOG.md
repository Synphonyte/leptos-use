# Changelog

All notable changes to this crate are documented in this file.

The format is based on [Keep a Changelog](https://keepachangelog.com/en/1.0.0/),
and this crate adheres to [Semantic Versioning](https://semver.org/spec/v2.0.0.html).

## [Unreleased]

## [0.20.1](https://github.com/Synphonyte/leptos-use/compare/wasm-js-macros-v0.20.0...wasm-js-macros-v0.20.1) - 2026-10-20

### Fixed

- `js!`, `js_fut!` and `js_value_from_to_string!` no longer require the calling crate to depend on `wasm-bindgen`, `js-sys` or `wasm-bindgen-futures`, or to import `JsValue`.

## [0.20.0] - 2026-10-20

### Added

- Initial release. This crate used to be part of [`leptos-use`](https://crates.io/crates/leptos-use).
