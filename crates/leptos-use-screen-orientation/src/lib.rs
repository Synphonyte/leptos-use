#![allow(unexpected_cfgs, clippy::extra_unused_lifetimes)]
//! Reactive [Screen Orientation API](https://developer.mozilla.org/en-US/docs/Web/API/Screen_Orientation_API)

mod use_screen_orientation;
pub use use_screen_orientation::*;

// Path shims so the sources keep their original `crate::…` paths.
#[allow(unused_imports)]
pub(crate) use leptos_use_event_listener::*;
#[allow(unused_imports)]
pub(crate) use sendwrap_fn::*;
#[allow(unused_imports)]
pub(crate) use wasm_js_macros::*;
