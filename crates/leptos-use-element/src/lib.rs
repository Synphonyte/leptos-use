#![allow(unexpected_cfgs, clippy::extra_unused_lifetimes)]
//! ElementMaybeSignal / ElementsMaybeSignal argument types and use_mutation_observer for Leptos

mod element_maybe_signal;
mod elements_maybe_signal;
mod use_mutation_observer;
pub use element_maybe_signal::*;
pub use elements_maybe_signal::*;
pub use use_mutation_observer::*;

// Dependencies used through `crate::…` paths.
#[allow(unused_imports)]
pub(crate) use leptos_use_supported::*;
#[allow(unused_imports)]
pub(crate) use sendwrap_fn::*;
#[allow(unused_imports)]
pub(crate) use wasm_js_macros::*;
