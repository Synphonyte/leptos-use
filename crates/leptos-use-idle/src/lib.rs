#![allow(unexpected_cfgs, clippy::extra_unused_lifetimes)]
//! Tracks whether the user is being inactive

mod use_idle;
pub use use_idle::*;

// Dependencies used through `crate::…` paths.
#[cfg(not(feature = "ssr"))]
pub(crate) use leptos_use_document::*;
#[cfg(not(feature = "ssr"))]
pub(crate) use leptos_use_event_listener::*;
pub(crate) use leptos_use_filters::*;
pub(crate) use leptos_use_now::*;
#[cfg(not(feature = "ssr"))]
pub(crate) use sendwrap_fn::*;
