#![allow(unexpected_cfgs, clippy::extra_unused_lifetimes)]
//! OptionLocalSignal / OptionLocalRwSignal for Leptos

#[allow(unused_imports)]
pub(crate) mod core {
    pub(crate) mod option_local_signal;
    pub use option_local_signal::*;
}
#[allow(unused_imports)]
pub use core::option_local_signal::*;
