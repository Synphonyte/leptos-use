#![allow(unexpected_cfgs, clippy::extra_unused_lifetimes)]
//! Reactive window size

mod use_window_size;
pub use use_window_size::*;

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
pub(crate) use leptos_use_media_query::*;
#[allow(unused_imports)]
pub(crate) use leptos_use_window::*;
