#![allow(unexpected_cfgs, clippy::extra_unused_lifetimes)]
//! Reactive [Clipboard API](https://developer.mozilla.org/en-US/docs/Web/API/Clipboard_API)

mod use_clipboard;
pub use use_clipboard::*;

// Dependencies used through `crate::…` paths.
#[allow(unused_imports)]
pub(crate) use leptos_use_event_listener::*;
#[allow(unused_imports)]
pub(crate) use leptos_use_supported::*;
#[allow(unused_imports)]
pub(crate) use leptos_use_timeout_fn::*;
#[allow(unused_imports)]
pub(crate) use sendwrap_fn::*;
#[allow(unused_imports)]
pub(crate) use wasm_js_macros::*;
