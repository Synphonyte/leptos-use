#![allow(unexpected_cfgs, clippy::extra_unused_lifetimes)]
//! Reactive mouse position

mod use_mouse;
pub use use_mouse::*;

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
pub(crate) use leptos_use_event_listener::*;
#[allow(unused_imports)]
pub(crate) use leptos_use_window::*;
