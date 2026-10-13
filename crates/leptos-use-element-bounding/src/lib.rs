#![allow(unexpected_cfgs, clippy::extra_unused_lifetimes)]
//! Reactive [bounding box](https://developer.mozilla.org/en-US/docs/Web/API/Element/getBoundingClientRect) of an HTML element

mod use_element_bounding;
pub use use_element_bounding::*;

// Dependencies used through `crate::…` paths.
#[allow(unused_imports)]
pub(crate) use leptos_use_element::*;
#[allow(unused_imports)]
pub(crate) use leptos_use_event_listener::*;
#[allow(unused_imports)]
pub(crate) use leptos_use_resize_observer::*;
#[allow(unused_imports)]
pub(crate) use leptos_use_window::*;
#[allow(unused_imports)]
pub(crate) use sendwrap_fn::*;
