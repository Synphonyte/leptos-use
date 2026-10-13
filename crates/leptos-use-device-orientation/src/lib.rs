#![allow(unexpected_cfgs, clippy::extra_unused_lifetimes)]
//! Reactive [DeviceOrientationEvent](https://developer.mozilla.org/en-US/docs/Web/API/DeviceOrientationEvent)

mod use_device_orientation;
pub use use_device_orientation::*;

// Path shims so the sources keep their original `crate::…` paths.
#[allow(unused_imports)]
pub(crate) use leptos_use_event_listener::*;
#[allow(unused_imports)]
pub(crate) use leptos_use_supported::*;
#[allow(unused_imports)]
pub(crate) use wasm_js_macros::*;
