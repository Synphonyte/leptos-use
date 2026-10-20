#![allow(unexpected_cfgs, clippy::extra_unused_lifetimes)]
//! Reactive [Screen Orientation API](https://developer.mozilla.org/en-US/docs/Web/API/Screen_Orientation_API)

mod return_type;
mod use_screen_orientation;
pub use return_type::*;
pub use use_screen_orientation::*;

// Dependencies used through `crate::…` paths.
#[cfg(not(feature = "ssr"))]
pub(crate) use leptos_use_event_listener::*;
#[cfg(not(feature = "ssr"))]
pub(crate) use sendwrap_fn::*;
#[cfg(not(feature = "ssr"))]
pub(crate) use wasm_js_macros::*;
