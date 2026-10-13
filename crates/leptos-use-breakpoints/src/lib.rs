#![allow(unexpected_cfgs, clippy::extra_unused_lifetimes)]
//! Reactive viewport breakpoints

mod use_breakpoints;
pub use use_breakpoints::*;

// Dependencies used through `crate::…` paths.
pub(crate) use leptos_use_media_query::*;
pub(crate) use leptos_use_window::*;
