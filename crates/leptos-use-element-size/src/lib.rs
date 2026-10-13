#![allow(unexpected_cfgs, clippy::extra_unused_lifetimes)]
//! Reactive size of an HTML element

mod use_element_size;
pub use use_element_size::*;

// Dependencies used through `crate::…` paths.
pub(crate) use leptos_use_core::*;
pub(crate) use leptos_use_element::*;
#[cfg(not(feature = "ssr"))]
pub(crate) use leptos_use_resize_observer::*;
#[cfg(not(feature = "ssr"))]
pub(crate) use leptos_use_watch_with_options::*;
