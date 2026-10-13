#![allow(unexpected_cfgs, clippy::extra_unused_lifetimes)]
//! Reactive [EventSource](https://developer.mozilla.org/en-US/docs/Web/API/EventSource)

mod use_event_source;
pub use use_event_source::*;

#[allow(unused_imports)]
pub(crate) mod core {
    pub use leptos_use_core::*;
}

// Path shims so the sources keep their original `crate::…` paths.
#[allow(unused_imports)]
pub(crate) use leptos_use_core::*;
#[allow(unused_imports)]
pub(crate) use leptos_use_event_listener::*;
#[allow(unused_imports)]
pub(crate) use sendwrap_fn::*;
