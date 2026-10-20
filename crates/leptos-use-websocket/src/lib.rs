#![allow(unexpected_cfgs, clippy::extra_unused_lifetimes)]
//! Creating and managing a [Websocket](https://developer.mozilla.org/en-US/docs/Web/API/WebSocket) connection

mod options;
mod return_type;
mod use_websocket;
pub use options::*;
pub use return_type::*;
pub use use_websocket::*;

// Dependencies used through `crate::…` paths.
pub(crate) use leptos_use_core::*;
#[cfg(not(feature = "ssr"))]
pub(crate) use leptos_use_filters::*;
#[cfg(not(feature = "ssr"))]
pub(crate) use leptos_use_interval_fn::*;
#[cfg(not(feature = "ssr"))]
pub(crate) use sendwrap_fn::*;
