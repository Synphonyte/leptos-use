#![allow(unexpected_cfgs, clippy::extra_unused_lifetimes)]
//! Macro to create helper functions that derive a Leptos signal

#[allow(unused_imports)]
pub(crate) mod utils {
    pub(crate) mod use_derive_signal;
    pub use use_derive_signal::*;
}
#[allow(unused_imports)]
pub use utils::use_derive_signal::*;
