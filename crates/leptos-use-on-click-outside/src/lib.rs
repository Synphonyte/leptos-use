#![allow(unexpected_cfgs, clippy::extra_unused_lifetimes)]
//! Listen for clicks outside an element

mod is;
mod on_click_outside;
mod options;
pub use is::*;
pub use leptos_use_element::ElementsMaybeSignal;
pub use on_click_outside::*;
pub use options::*;

// Dependencies used through `crate::…` paths.
pub(crate) use leptos_use_element::*;
#[cfg(not(feature = "ssr"))]
pub(crate) use leptos_use_event_listener::*;
pub(crate) use leptos_use_window::*;
#[cfg(not(feature = "ssr"))]
pub(crate) use sendwrap_fn::*;

#[cfg(feature = "ssr")]
use leptos_use_event_listener as _;
