#![allow(unexpected_cfgs, clippy::extra_unused_lifetimes)]
//! Tracks whether the user is being inactive

mod options;
mod return_type;
mod use_idle;
pub use leptos_use_filters::{DebounceOptions, FilterOptions, ThrottleOptions};
pub use options::*;
pub use return_type::*;
pub use use_idle::*;

// Dependencies used through `crate::…` paths.
#[cfg(not(feature = "ssr"))]
pub(crate) use leptos_use_document::*;
#[cfg(not(feature = "ssr"))]
pub(crate) use leptos_use_event_listener::*;
pub(crate) use leptos_use_filters::*;
pub(crate) use leptos_use_now::*;
#[cfg(not(feature = "ssr"))]
pub(crate) use sendwrap_fn::*;

#[cfg(feature = "ssr")]
use {leptos_use_document as _, leptos_use_event_listener as _};
