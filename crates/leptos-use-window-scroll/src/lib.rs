#![allow(unexpected_cfgs, clippy::extra_unused_lifetimes)]
//! Reactive window scroll

mod use_window_scroll;
pub use use_window_scroll::*;

// Path shims so the sources keep their original `crate::…` paths.
#[allow(unused_imports)]
pub(crate) use leptos_use_event_listener::*;
#[allow(unused_imports)]
pub(crate) use leptos_use_window::*;
