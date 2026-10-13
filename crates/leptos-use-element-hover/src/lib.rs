#![allow(unexpected_cfgs, clippy::extra_unused_lifetimes)]
//! Reactive element's hover state

mod use_element_hover;
pub use use_element_hover::*;

// Dependencies used through `crate::…` paths.
pub(crate) use leptos_use_element::*;
pub(crate) use leptos_use_event_listener::*;
