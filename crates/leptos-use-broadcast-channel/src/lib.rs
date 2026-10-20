#![allow(unexpected_cfgs, clippy::extra_unused_lifetimes)]
//! Reactive [BroadcastChannel API](https://developer.mozilla.org/en-US/docs/Web/API/BroadcastChannel)

mod return_type;
mod use_broadcast_channel;
pub use leptos_use_option_local_signal::OptionLocalSignal;
pub use return_type::*;
pub use use_broadcast_channel::*;

// Dependencies used through `crate::…` paths.
pub(crate) use leptos_use_event_listener::*;
pub(crate) use leptos_use_option_local_signal::*;
pub(crate) use leptos_use_supported::*;
pub(crate) use sendwrap_fn::*;
pub(crate) use wasm_js_macros::*;
