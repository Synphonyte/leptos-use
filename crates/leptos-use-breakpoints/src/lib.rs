#![allow(unexpected_cfgs, clippy::extra_unused_lifetimes)]
//! Reactive viewport breakpoints

mod use_breakpoints;
pub use use_breakpoints::*;

// Dependencies used through `crate::…` paths.
#[allow(unused_imports)]
pub(crate) use leptos_use_media_query::*;
#[allow(unused_imports)]
pub(crate) use leptos_use_window::*;
