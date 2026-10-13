#![allow(unexpected_cfgs, clippy::extra_unused_lifetimes)]
//! UseRwSignal: a signal that is either a RwSignal or a split signal pair (Leptos)

#[allow(unused_imports)]
pub(crate) mod core {
    pub(crate) mod use_rw_signal;
    pub use use_rw_signal::*;
}
#[allow(unused_imports)]
pub use core::use_rw_signal::*;
