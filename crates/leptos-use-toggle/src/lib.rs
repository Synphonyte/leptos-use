#![allow(unexpected_cfgs, clippy::extra_unused_lifetimes)]
//! A boolean switcher with utility functions

mod return_type;
mod use_toggle;
pub use return_type::*;
pub use use_toggle::*;

// Dependencies used through `crate::…` paths.
pub(crate) use leptos_use_maybe_rw_signal::*;
