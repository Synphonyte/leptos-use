#![allow(unexpected_cfgs, clippy::extra_unused_lifetimes)]
//! Debounced and throttled function wrappers for Leptos

mod use_debounce_fn;
mod use_throttle_fn;
pub use leptos_use_filters::{DebounceOptions, ThrottleOptions};
pub use use_debounce_fn::*;
pub use use_throttle_fn::*;

// Dependencies used through `crate::…` paths.
pub(crate) use leptos_use_filters::*;
