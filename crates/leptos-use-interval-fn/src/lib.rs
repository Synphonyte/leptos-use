#![allow(unexpected_cfgs, clippy::extra_unused_lifetimes)]
//! Wrapper for `set_interval` with controls

mod options;
mod use_interval_fn;
pub use leptos_use_filters::Pausable;
pub use options::*;
pub use use_interval_fn::*;

// Dependencies used through `crate::…` paths.
pub(crate) use sendwrap_fn::*;
