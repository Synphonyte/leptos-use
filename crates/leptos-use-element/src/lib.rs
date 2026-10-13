#![allow(unexpected_cfgs, clippy::extra_unused_lifetimes)]
//! ElementMaybeSignal / ElementsMaybeSignal argument types and use_mutation_observer for Leptos

mod use_mutation_observer;
pub use use_mutation_observer::*;

#[allow(unused_imports)]
pub(crate) mod core {
    pub(crate) mod element_maybe_signal;
    pub(crate) mod elements_maybe_signal;
    pub use element_maybe_signal::*;
    pub use elements_maybe_signal::*;
}
#[allow(unused_imports)]
pub use core::element_maybe_signal::*;
#[allow(unused_imports)]
pub use core::elements_maybe_signal::*;

// Path shims so the sources keep their original `crate::…` paths.
#[allow(unused_imports)]
pub(crate) use leptos_use_supported::*;
#[allow(unused_imports)]
pub(crate) use sendwrap_fn::*;
#[allow(unused_imports)]
pub(crate) use wasm_js_macros::*;
