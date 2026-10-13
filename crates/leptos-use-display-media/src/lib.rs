#![allow(unexpected_cfgs, clippy::extra_unused_lifetimes)]
//! Reactive [`mediaDevices.getDisplayMedia`](https://developer.mozilla.org/en-US/docs/Web/API/MediaDevices/getDisplayMedia) streaming

mod use_display_media;
pub use use_display_media::*;

// Dependencies used through `crate::…` paths.
#[allow(unused_imports)]
pub(crate) use leptos_use_maybe_rw_signal::*;
#[allow(unused_imports)]
pub(crate) use leptos_use_option_local_signal::*;
#[allow(unused_imports)]
pub(crate) use leptos_use_window::*;
#[allow(unused_imports)]
pub(crate) use sendwrap_fn::*;
#[allow(unused_imports)]
pub(crate) use wasm_js_macros::*;
