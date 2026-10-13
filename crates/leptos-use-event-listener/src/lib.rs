#![allow(unexpected_cfgs, clippy::extra_unused_lifetimes)]
//! Use EventListener with ease

mod use_event_listener;
pub use use_event_listener::*;

#[allow(unused_imports)]
pub(crate) mod core {
    pub use leptos_use_element::*;
}

// Path shims so the sources keep their original `crate::…` paths.
#[allow(unused_imports)]
pub(crate) use leptos_use_element::*;
#[allow(unused_imports)]
pub(crate) use leptos_use_watch_with_options::*;
#[allow(unused_imports)]
pub(crate) use sendwrap_fn::*;
