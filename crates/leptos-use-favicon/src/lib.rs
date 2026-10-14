#![allow(unexpected_cfgs, clippy::extra_unused_lifetimes)]
//! Reactive favicon

mod options;
mod use_favicon;
pub use options::*;
pub use use_favicon::*;

// Dependencies used through `crate::…` paths.
pub(crate) use leptos_use_maybe_rw_signal::*;
