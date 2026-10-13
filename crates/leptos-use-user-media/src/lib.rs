#![allow(unexpected_cfgs, clippy::extra_unused_lifetimes)]
//! Reactive [`mediaDevices.getUserMedia`](https://developer.mozilla.org/en-US/docs/Web/API/MediaDevices/getUserMedia) streaming

mod use_user_media;
pub use use_user_media::*;

// Dependencies used through `crate::…` paths.
pub(crate) use leptos_use_maybe_rw_signal::*;
pub(crate) use leptos_use_option_local_signal::*;
#[cfg(not(feature = "ssr"))]
pub(crate) use leptos_use_window::*;
#[cfg(not(feature = "ssr"))]
pub(crate) use wasm_js_macros::*;
