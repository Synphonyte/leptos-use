#![allow(unexpected_cfgs, clippy::extra_unused_lifetimes)]
//! Reactive locales

mod use_locales;
pub use use_locales::*;

#[allow(unused_imports)]
pub(crate) mod utils {
    pub use leptos_use_header::*;
}

// Path shims so the sources keep their original `crate::…` paths.
#[allow(unused_imports)]
pub(crate) use leptos_use_event_listener::*;
#[allow(unused_imports)]
pub(crate) use leptos_use_header::*;
#[allow(unused_imports)]
pub(crate) use leptos_use_window::*;
