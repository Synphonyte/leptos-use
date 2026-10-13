#![allow(unexpected_cfgs, clippy::extra_unused_lifetimes)]
//! Reactive [Permissions API](https://developer.mozilla.org/en-US/docs/Web/API/Permissions_API)

mod use_permission;
pub use use_permission::*;

// Path shims so the sources keep their original `crate::…` paths.
#[allow(unused_imports)]
pub(crate) use leptos_use_event_listener::*;
#[allow(unused_imports)]
pub(crate) use wasm_js_macros::*;
