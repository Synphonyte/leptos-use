#![allow(unexpected_cfgs, clippy::extra_unused_lifetimes)]
//! Reactively track window focus

mod use_window_focus;
pub use use_window_focus::*;

// Dependencies used through `crate::…` paths.
#[allow(unused_imports)]
pub(crate) use leptos_use_event_listener::*;
