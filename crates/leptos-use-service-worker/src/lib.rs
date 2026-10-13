#![allow(unexpected_cfgs, clippy::extra_unused_lifetimes)]
//! Reactive [ServiceWorker API](https://developer.mozilla.org/en-US/docs/Web/API/Service_Worker_API)

mod use_service_worker;
pub use use_service_worker::*;

// Dependencies used through `crate::…` paths.
pub(crate) use leptos_use_window::*;
pub(crate) use sendwrap_fn::*;
pub(crate) use wasm_js_macros::*;
