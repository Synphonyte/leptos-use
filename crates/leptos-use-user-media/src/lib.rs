#![allow(unexpected_cfgs, clippy::extra_unused_lifetimes)]
//! Reactive [`mediaDevices.getUserMedia`](https://developer.mozilla.org/en-US/docs/Web/API/MediaDevices/getUserMedia) streaming

mod options;
mod return_type;
mod use_user_media;
pub use options::*;
pub use return_type::*;
pub use use_user_media::*;

// Dependencies used through `crate::…` paths.
pub(crate) use leptos_use_maybe_rw_signal::*;
pub(crate) use leptos_use_option_local_signal::*;
#[cfg(not(feature = "ssr"))]
pub(crate) use leptos_use_window::*;
#[cfg(not(feature = "ssr"))]
pub(crate) use wasm_js_macros::*;

#[cfg(feature = "ssr")]
use {leptos_use_window as _, wasm_js_macros as _};
