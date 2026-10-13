#![allow(unexpected_cfgs, clippy::extra_unused_lifetimes)]
//! Reactive `document.activeElement`

mod use_active_element;
pub use use_active_element::*;

// Dependencies used through `crate::…` paths.
#[allow(unused_imports)]
pub(crate) use leptos_use_document::*;
#[allow(unused_imports)]
pub(crate) use leptos_use_event_listener::*;
#[allow(unused_imports)]
pub(crate) use leptos_use_option_local_signal::*;
#[allow(unused_imports)]
pub(crate) use leptos_use_window::*;
