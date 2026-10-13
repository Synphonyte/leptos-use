#![allow(unexpected_cfgs, clippy::extra_unused_lifetimes)]
//! Reactive is_ok / is_err / is_some / is_none signals for Leptos

mod is_err;
mod is_none;
mod is_ok;
mod is_some;
pub use is_err::*;
pub use is_none::*;
pub use is_ok::*;
pub use is_some::*;

// Path shims so the sources keep their original `crate::…` paths.
#[allow(unused_imports)]
pub(crate) use leptos_use_derive_signal::*;
