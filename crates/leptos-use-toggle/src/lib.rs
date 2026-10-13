#![allow(unexpected_cfgs, clippy::extra_unused_lifetimes)]
//! A boolean switcher with utility functions

mod use_toggle;
pub use use_toggle::*;

// Dependencies used through `crate::…` paths.
pub(crate) use leptos_use_maybe_rw_signal::*;
