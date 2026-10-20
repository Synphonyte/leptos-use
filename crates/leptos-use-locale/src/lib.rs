#![allow(unexpected_cfgs, clippy::extra_unused_lifetimes)]
//! Reactive locale matching

mod options;
mod use_locale;
pub use options::*;
pub use use_locale::*;

// Dependencies used through `crate::…` paths.
pub(crate) use leptos_use_locales::*;
