#![allow(unexpected_cfgs, clippy::extra_unused_lifetimes)]
//! Two-way Signals synchronization

mod options;
mod sync_signal;
pub use leptos_use_rw_signal::UseRwSignal;
pub use options::*;
pub use sync_signal::*;
