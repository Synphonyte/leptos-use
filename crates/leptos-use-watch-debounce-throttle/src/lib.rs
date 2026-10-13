#![allow(unexpected_cfgs, clippy::extra_unused_lifetimes)]
//! Debounced and throttled watch for Leptos

mod watch_debounced;
mod watch_throttled;
pub use watch_debounced::*;
pub use watch_throttled::*;

// Dependencies used through `crate::…` paths.
#[allow(unused_imports)]
pub(crate) use leptos_use_filters::*;
#[allow(unused_imports)]
pub(crate) use leptos_use_watch_with_options::*;
