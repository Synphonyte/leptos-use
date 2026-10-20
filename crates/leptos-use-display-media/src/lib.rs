#![allow(unexpected_cfgs, clippy::extra_unused_lifetimes)]
//! Reactive [`mediaDevices.getDisplayMedia`](https://developer.mozilla.org/en-US/docs/Web/API/MediaDevices/getDisplayMedia) streaming

mod options;
mod return_type;
mod use_display_media;
pub use leptos_use_maybe_rw_signal::MaybeRwSignal;
pub use leptos_use_option_local_signal::OptionLocalSignal;
pub use options::*;
pub use return_type::*;
pub use use_display_media::*;

// Dependencies used through `crate::…` paths.
pub(crate) use leptos_use_option_local_signal::*;
#[cfg(not(feature = "ssr"))]
pub(crate) use leptos_use_window::*;
pub(crate) use sendwrap_fn::*;
#[cfg(not(feature = "ssr"))]
pub(crate) use wasm_js_macros::*;

#[cfg(feature = "ssr")]
use {leptos_use_window as _, wasm_js_macros as _};
