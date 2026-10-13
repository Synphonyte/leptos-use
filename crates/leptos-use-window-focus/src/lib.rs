#![allow(unexpected_cfgs, clippy::extra_unused_lifetimes)]
//! Reactively track window focus

mod use_window_focus;
pub use use_window_focus::*;

// Path shims so the sources keep their original `crate::…` paths.
#[allow(unused_imports)]
pub(crate) use leptos_use_event_listener::*;
