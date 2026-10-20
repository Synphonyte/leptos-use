#![allow(unexpected_cfgs, clippy::extra_unused_lifetimes)]
//! Reactive [Geolocation API](https://developer.mozilla.org/en-US/docs/Web/API/Geolocation_API)

mod options;
mod return_type;
mod use_geolocation;
pub use options::*;
pub use return_type::*;
pub use use_geolocation::*;

// Dependencies used through `crate::…` paths.
pub(crate) use leptos_use_option_local_signal::*;
#[cfg(not(feature = "ssr"))]
pub(crate) use leptos_use_window::*;
#[cfg(not(feature = "ssr"))]
pub(crate) use sendwrap_fn::*;
