#![allow(unexpected_cfgs, clippy::extra_unused_lifetimes)]
//! Reactive [DeviceOrientationEvent](https://developer.mozilla.org/en-US/docs/Web/API/DeviceOrientationEvent)

mod return_type;
mod use_device_orientation;
pub use return_type::*;
pub use use_device_orientation::*;

// Dependencies used through `crate::…` paths.
#[cfg(not(feature = "ssr"))]
pub(crate) use leptos_use_event_listener::*;
#[cfg(not(feature = "ssr"))]
pub(crate) use leptos_use_supported::*;
#[cfg(not(feature = "ssr"))]
pub(crate) use wasm_js_macros::*;

#[cfg(feature = "ssr")]
use leptos_use_event_listener as _;
