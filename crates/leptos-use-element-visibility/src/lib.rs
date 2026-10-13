#![allow(unexpected_cfgs, clippy::extra_unused_lifetimes)]
//! Tracks the visibility of an element within the viewport

mod use_element_visibility;
pub use use_element_visibility::*;

// Dependencies used through `crate::…` paths.
#[allow(unused_imports)]
pub(crate) use leptos_use_element::*;
#[allow(unused_imports)]
pub(crate) use leptos_use_intersection_observer::*;
