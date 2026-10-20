#![allow(unexpected_cfgs, clippy::extra_unused_lifetimes)]
//! Reactive [`window.devicePixelRatio`](https://developer.mozilla.org/en-US/docs/Web/API/Window/devicePixelRatio)

mod use_device_pixel_ratio;
pub use use_device_pixel_ratio::*;

// Dependencies used through `crate::…` paths.
#[cfg(not(feature = "ssr"))]
pub(crate) use leptos_use_event_listener::*;
