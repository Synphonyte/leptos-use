#![allow(unexpected_cfgs, clippy::extra_unused_lifetimes)]
//! Tracks the visibility of an element within the viewport

mod options;
mod use_element_visibility;
pub use options::*;
pub use use_element_visibility::*;

// Dependencies used through `crate::…` paths.
pub(crate) use leptos_use_element::*;
#[cfg(not(feature = "ssr"))]
pub(crate) use leptos_use_intersection_observer::*;

#[cfg(feature = "ssr")]
use leptos_use_intersection_observer as _;
