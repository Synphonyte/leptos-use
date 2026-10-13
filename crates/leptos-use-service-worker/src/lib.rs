#![allow(unexpected_cfgs, clippy::extra_unused_lifetimes)]
//! Reactive [ServiceWorker API](https://developer.mozilla.org/en-US/docs/Web/API/Service_Worker_API)

mod use_service_worker;
pub use use_service_worker::*;

// Dependencies used through `crate::…` paths.
#[allow(unused_imports)]
pub(crate) use leptos_use_window::*;
#[allow(unused_imports)]
pub(crate) use sendwrap_fn::*;
#[allow(unused_imports)]
pub(crate) use wasm_js_macros::*;
