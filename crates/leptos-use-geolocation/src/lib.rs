#![allow(unexpected_cfgs, clippy::extra_unused_lifetimes)]
//! Reactive [Geolocation API](https://developer.mozilla.org/en-US/docs/Web/API/Geolocation_API)

mod use_geolocation;
pub use use_geolocation::*;

// Dependencies used through `crate::…` paths.
#[allow(unused_imports)]
pub(crate) use leptos_use_core::*;
#[allow(unused_imports)]
pub(crate) use leptos_use_option_local_signal::*;
#[allow(unused_imports)]
pub(crate) use leptos_use_window::*;
#[allow(unused_imports)]
pub(crate) use sendwrap_fn::*;
