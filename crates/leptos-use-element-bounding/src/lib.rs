#![allow(unexpected_cfgs, clippy::extra_unused_lifetimes)]
//! Reactive [bounding box](https://developer.mozilla.org/en-US/docs/Web/API/Element/getBoundingClientRect) of an HTML element

mod options;
mod return_type;
mod use_element_bounding;
pub use options::*;
pub use return_type::*;
pub use use_element_bounding::*;

// Dependencies used through `crate::…` paths.
pub(crate) use leptos_use_element::*;
#[cfg(not(feature = "ssr"))]
pub(crate) use leptos_use_event_listener::*;
#[cfg(not(feature = "ssr"))]
pub(crate) use leptos_use_resize_observer::*;
#[cfg(not(feature = "ssr"))]
pub(crate) use leptos_use_window::*;
#[cfg(not(feature = "ssr"))]
pub(crate) use sendwrap_fn::*;

#[cfg(feature = "ssr")]
use {leptos_use_event_listener as _, leptos_use_resize_observer as _, leptos_use_window as _};
