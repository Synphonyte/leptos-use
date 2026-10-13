#![allow(unexpected_cfgs, clippy::extra_unused_lifetimes)]
//! Reactive [BroadcastChannel API](https://developer.mozilla.org/en-US/docs/Web/API/BroadcastChannel)

mod use_broadcast_channel;
pub use use_broadcast_channel::*;

// Dependencies used through `crate::…` paths.
#[allow(unused_imports)]
pub(crate) use leptos_use_event_listener::*;
#[allow(unused_imports)]
pub(crate) use leptos_use_option_local_signal::*;
#[allow(unused_imports)]
pub(crate) use leptos_use_supported::*;
#[allow(unused_imports)]
pub(crate) use sendwrap_fn::*;
#[allow(unused_imports)]
pub(crate) use wasm_js_macros::*;
