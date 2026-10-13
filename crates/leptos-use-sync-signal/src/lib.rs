#![allow(unexpected_cfgs, clippy::extra_unused_lifetimes)]
//! Two-way Signals synchronization

mod sync_signal;
pub use sync_signal::*;

// Dependencies used through `crate::…` paths.
#[allow(unused_imports)]
pub(crate) use leptos_use_rw_signal::*;
