#![allow(unexpected_cfgs, clippy::extra_unused_lifetimes)]
//! Reactive counter increases on every interval

mod use_interval;
pub use use_interval::*;

// Dependencies used through `crate::…` paths.
#[allow(unused_imports)]
pub(crate) use leptos_use_filters::*;
#[allow(unused_imports)]
pub(crate) use leptos_use_interval_fn::*;
#[allow(unused_imports)]
pub(crate) use sendwrap_fn::*;
