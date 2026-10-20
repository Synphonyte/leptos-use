#![allow(unexpected_cfgs, clippy::extra_unused_lifetimes)]
//! Reactive [EventSource](https://developer.mozilla.org/en-US/docs/Web/API/EventSource)

mod options;
mod return_type;
mod use_event_source;
pub use leptos_use_core::{ConnectionReadyState, ReconnectLimit};
pub use options::*;
pub use return_type::*;
pub use use_event_source::*;

// Dependencies used through `crate::…` paths.
#[cfg(not(feature = "ssr"))]
pub(crate) use leptos_use_event_listener::*;
#[cfg(not(feature = "ssr"))]
pub(crate) use sendwrap_fn::*;

#[cfg(feature = "ssr")]
use leptos_use_event_listener as _;
