#![allow(unexpected_cfgs, clippy::extra_unused_lifetimes)]
//! Reactive [Notification API](https://developer.mozilla.org/en-US/docs/Web/API/Notification)

mod use_web_notification;
pub use use_web_notification::*;

// Dependencies used through `crate::…` paths.
#[cfg(not(feature = "ssr"))]
pub(crate) use leptos_use_event_listener::*;
pub(crate) use leptos_use_option_local_signal::*;
pub(crate) use leptos_use_supported::*;
pub(crate) use leptos_use_window::*;
#[cfg(not(feature = "ssr"))]
pub(crate) use wasm_js_macros::*;
