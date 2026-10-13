#![allow(unexpected_cfgs, clippy::extra_unused_lifetimes)]
//! A version of `leptos::watch` but with additional options

mod watch_with_options;
pub use watch_with_options::*;

// Dependencies used through `crate::…` paths.
#[allow(unused_imports)]
pub(crate) use leptos_use_filters::*;
