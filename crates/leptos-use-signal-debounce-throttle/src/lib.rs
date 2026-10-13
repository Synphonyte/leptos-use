#![allow(unexpected_cfgs, clippy::extra_unused_lifetimes)]
//! Debounced and throttled signals for Leptos

mod signal_debounced;
mod signal_throttled;
pub use signal_debounced::*;
pub use signal_throttled::*;

#[allow(unused_imports)]
pub(crate) mod utils {
    pub(crate) mod signal_filtered;
    pub use leptos_use_filters::*;
    pub use signal_filtered::*;
}

// Path shims so the sources keep their original `crate::…` paths.
#[allow(unused_imports)]
pub(crate) use leptos_use_filters::*;
#[allow(unused_imports)]
pub(crate) use leptos_use_fn_debounce_throttle::*;
