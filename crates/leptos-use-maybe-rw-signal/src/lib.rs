#![allow(unexpected_cfgs, clippy::extra_unused_lifetimes)]
//! MaybeRwSignal: accept either a read-only or a read-write signal (Leptos)

#[allow(unused_imports)]
pub(crate) mod core {
    pub(crate) mod maybe_rw_signal;
    pub use maybe_rw_signal::*;
}
#[allow(unused_imports)]
pub use core::maybe_rw_signal::*;

// Path shims so the sources keep their original `crate::…` paths.
#[cfg(feature = "reactive_stores")]
#[allow(unused_imports)]
pub(crate) use leptos_use_sync_signal::*;
