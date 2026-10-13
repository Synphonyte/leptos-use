#![allow(unexpected_cfgs, clippy::extra_unused_lifetimes)]
//! Reactive locale matching

mod use_locale;
pub use use_locale::*;

// Path shims so the sources keep their original `crate::…` paths.
#[allow(unused_imports)]
pub(crate) use leptos_use_locales::*;
