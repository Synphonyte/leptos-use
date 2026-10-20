#![allow(unexpected_cfgs, clippy::extra_unused_lifetimes)]
//! Reactive favicon

mod options;
mod use_favicon;
pub use leptos_use_maybe_rw_signal::MaybeRwSignal;
pub use options::*;
pub use use_favicon::*;
