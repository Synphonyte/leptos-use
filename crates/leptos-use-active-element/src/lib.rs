#![allow(unexpected_cfgs, clippy::extra_unused_lifetimes)]
//! Reactive `document.activeElement`

mod use_active_element;
pub use use_active_element::*;

// Dependencies used through `crate::…` paths.
pub(crate) use leptos_use_document::*;
pub(crate) use leptos_use_event_listener::*;
pub(crate) use leptos_use_option_local_signal::*;
pub(crate) use leptos_use_window::*;
