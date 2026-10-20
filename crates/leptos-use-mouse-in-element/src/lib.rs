#![allow(unexpected_cfgs, clippy::extra_unused_lifetimes)]
//! Reactive mouse position related to an element

mod options;
mod return_type;
mod use_mouse_in_element;
pub use leptos_use_core::Position;
pub use leptos_use_mouse::{UseMouseCoordType, UseMouseEventExtractor, UseMouseSourceType};
pub use leptos_use_window::{UseDocument, UseWindow};
pub use options::*;
pub use return_type::*;
pub use use_mouse_in_element::*;

// Dependencies used through `crate::…` paths.
pub(crate) use leptos_use_element::*;
#[cfg(not(feature = "ssr"))]
pub(crate) use leptos_use_event_listener::*;
pub(crate) use leptos_use_mouse::*;
pub(crate) use leptos_use_window::*;
#[cfg(not(feature = "ssr"))]
pub(crate) use sendwrap_fn::*;
