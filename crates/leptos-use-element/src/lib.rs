#![allow(unexpected_cfgs, clippy::extra_unused_lifetimes)]
//! ElementMaybeSignal / ElementsMaybeSignal argument types and use_mutation_observer for Leptos

mod element_maybe_signal;
mod elements_maybe_signal;
mod options;
mod return_type;
mod use_mutation_observer;
pub use element_maybe_signal::*;
pub use elements_maybe_signal::*;
pub use options::*;
pub use return_type::*;
pub use use_mutation_observer::*;

// Dependencies used through `crate::…` paths.
#[cfg(not(feature = "ssr"))]
pub(crate) use leptos_use_supported::*;
#[cfg(not(feature = "ssr"))]
pub(crate) use sendwrap_fn::*;
#[cfg(not(feature = "ssr"))]
pub(crate) use wasm_js_macros::*;
