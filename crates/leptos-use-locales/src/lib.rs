#![allow(unexpected_cfgs, clippy::extra_unused_lifetimes)]
//! Reactive locales

mod use_locales;
pub use use_locales::*;

// Dependencies used through `crate::…` paths.
#[cfg(not(feature = "ssr"))]
pub(crate) use leptos_use_event_listener::*;
pub(crate) use leptos_use_header::*;
#[cfg(not(feature = "ssr"))]
pub(crate) use leptos_use_window::*;
