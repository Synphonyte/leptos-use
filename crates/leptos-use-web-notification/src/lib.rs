#![allow(unexpected_cfgs, clippy::extra_unused_lifetimes)]
//! Reactive [Notification API](https://developer.mozilla.org/en-US/docs/Web/API/Notification)

mod options;
mod return_type;
mod use_web_notification;
pub use leptos_use_option_local_signal::OptionLocalSignal;
pub use options::*;
pub use return_type::*;
pub use use_web_notification::*;

// Dependencies used through `crate::…` paths.
#[cfg(not(feature = "ssr"))]
pub(crate) use leptos_use_event_listener::*;
pub(crate) use leptos_use_option_local_signal::*;
pub(crate) use leptos_use_supported::*;
pub(crate) use leptos_use_window::*;
#[cfg(not(feature = "ssr"))]
pub(crate) use wasm_js_macros::*;

#[cfg(feature = "ssr")]
use leptos_use_event_listener as _;
