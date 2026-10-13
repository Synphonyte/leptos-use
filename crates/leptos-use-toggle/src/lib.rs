#![allow(unexpected_cfgs, clippy::extra_unused_lifetimes)]
//! A boolean switcher with utility functions

mod use_toggle;
pub use use_toggle::*;

#[allow(unused_imports)]
pub(crate) mod core {
    pub use leptos_use_maybe_rw_signal::*;
}

// Path shims so the sources keep their original `crate::…` paths.
#[allow(unused_imports)]
pub(crate) use leptos_use_maybe_rw_signal::*;
