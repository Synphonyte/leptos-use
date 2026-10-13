#![allow(unexpected_cfgs, clippy::extra_unused_lifetimes)]
//! MaybeRwSignal: accept either a read-only or a read-write signal (Leptos)

mod maybe_rw_signal;
pub use maybe_rw_signal::*;

// Dependencies used through `crate::…` paths.
#[cfg(feature = "reactive_stores")]
pub(crate) use leptos_use_sync_signal::*;
