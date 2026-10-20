#![allow(unexpected_cfgs, clippy::extra_unused_lifetimes)]
//! Reactive [Permissions API](https://developer.mozilla.org/en-US/docs/Web/API/Permissions_API)

mod use_permission;
pub use use_permission::*;

// Dependencies used through `crate::…` paths.
#[cfg(not(feature = "ssr"))]
pub(crate) use leptos_use_event_listener::*;
#[cfg(not(feature = "ssr"))]
pub(crate) use wasm_js_macros::*;

#[cfg(feature = "ssr")]
use leptos_use_event_listener as _;
