#![allow(unexpected_cfgs, clippy::extra_unused_lifetimes)]
//! Use EventListener with ease

mod options;
mod use_event_listener;
pub use options::*;
pub use use_event_listener::*;

// Dependencies used through `crate::…` paths.
pub(crate) use leptos_use_element::*;
#[cfg(not(feature = "ssr"))]
pub(crate) use leptos_use_watch_with_options::*;
#[cfg(not(feature = "ssr"))]
pub(crate) use sendwrap_fn::*;
