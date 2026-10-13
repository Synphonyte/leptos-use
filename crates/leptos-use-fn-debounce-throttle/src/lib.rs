#![allow(unexpected_cfgs, clippy::extra_unused_lifetimes)]
//! Debounced and throttled function wrappers for Leptos

mod use_debounce_fn;
mod use_throttle_fn;
pub use use_debounce_fn::*;
pub use use_throttle_fn::*;

#[allow(unused_imports)]
pub(crate) mod utils {
    pub use leptos_use_filters::*;
}

// Path shims so the sources keep their original `crate::…` paths.
#[allow(unused_imports)]
pub(crate) use leptos_use_filters::*;
