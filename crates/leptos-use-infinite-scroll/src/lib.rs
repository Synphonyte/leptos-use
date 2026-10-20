#![allow(unexpected_cfgs, clippy::extra_unused_lifetimes)]
//! Infinite scrolling of the element

mod options;
mod use_infinite_scroll;
pub use leptos_use_core::{Direction, Directions};
pub use leptos_use_event_listener::UseEventListenerOptions;
pub use options::*;
pub use use_infinite_scroll::*;

// Dependencies used through `crate::…` paths.
pub(crate) use leptos_use_element::*;
pub(crate) use leptos_use_element_visibility::*;
pub(crate) use leptos_use_scroll::*;
