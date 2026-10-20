#![allow(unexpected_cfgs, clippy::extra_unused_lifetimes)]
//! Reactive scroll position and state

mod options;
mod return_type;
mod use_scroll;
pub use leptos_use_core::{Direction, Directions};
pub use leptos_use_event_listener::UseEventListenerOptions;
pub use options::*;
pub use return_type::*;
pub use use_scroll::*;

// Dependencies used through `crate::…` paths.
pub(crate) use leptos_use_element::*;
#[cfg(not(feature = "ssr"))]
pub(crate) use leptos_use_event_listener::*;
#[cfg(not(feature = "ssr"))]
pub(crate) use leptos_use_filters::*;
#[cfg(not(feature = "ssr"))]
pub(crate) use leptos_use_fn_debounce_throttle::*;
#[cfg(not(feature = "ssr"))]
pub(crate) use sendwrap_fn::*;

#[cfg(feature = "ssr")]
use leptos_use_fn_debounce_throttle as _;
