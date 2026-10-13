#![allow(unexpected_cfgs, clippy::extra_unused_lifetimes)]
//! Reactive [`mediaDevices.getDisplayMedia`](https://developer.mozilla.org/en-US/docs/Web/API/MediaDevices/getDisplayMedia) streaming

mod use_display_media;
pub use use_display_media::*;

// Dependencies used through `crate::…` paths.
pub(crate) use leptos_use_maybe_rw_signal::*;
pub(crate) use leptos_use_option_local_signal::*;
#[cfg(not(feature = "ssr"))]
pub(crate) use leptos_use_window::*;
pub(crate) use sendwrap_fn::*;
#[cfg(not(feature = "ssr"))]
pub(crate) use wasm_js_macros::*;
