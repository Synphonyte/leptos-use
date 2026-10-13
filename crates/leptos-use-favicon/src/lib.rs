#![allow(unexpected_cfgs, clippy::extra_unused_lifetimes)]
//! Reactive favicon

mod use_favicon;
pub use use_favicon::*;

#[allow(unused_imports)]
pub(crate) mod core {
    pub use leptos_use_maybe_rw_signal::*;
}

// Path shims so the sources keep their original `crate::…` paths.
#[allow(unused_imports)]
pub(crate) use leptos_use_maybe_rw_signal::*;
