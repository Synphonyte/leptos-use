#![allow(unexpected_cfgs, clippy::extra_unused_lifetimes)]
//! Reactive scroll position and state

mod use_scroll;
pub use use_scroll::*;

#[allow(unused_imports)]
pub(crate) mod core {
    pub use leptos_use_core::*;
    pub use leptos_use_element::*;
}

// Path shims so the sources keep their original `crate::…` paths.
#[allow(unused_imports)]
pub(crate) use leptos_use_core::*;
#[allow(unused_imports)]
pub(crate) use leptos_use_element::*;
#[allow(unused_imports)]
pub(crate) use leptos_use_event_listener as use_event_listener;
#[allow(unused_imports)]
pub(crate) use leptos_use_event_listener::*;
#[allow(unused_imports)]
pub(crate) use leptos_use_filters::*;
#[allow(unused_imports)]
pub(crate) use leptos_use_fn_debounce_throttle::*;
#[allow(unused_imports)]
pub(crate) use sendwrap_fn::*;
