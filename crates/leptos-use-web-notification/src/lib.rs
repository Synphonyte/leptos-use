#![allow(unexpected_cfgs, clippy::extra_unused_lifetimes)]
//! Reactive [Notification API](https://developer.mozilla.org/en-US/docs/Web/API/Notification)

mod use_web_notification;
pub use use_web_notification::*;

#[allow(unused_imports)]
pub(crate) mod core {
    pub use leptos_use_option_local_signal::*;
}

// Path shims so the sources keep their original `crate::…` paths.
#[allow(unused_imports)]
pub(crate) use leptos_use_event_listener::*;
#[allow(unused_imports)]
pub(crate) use leptos_use_option_local_signal::*;
#[allow(unused_imports)]
pub(crate) use leptos_use_supported::*;
#[allow(unused_imports)]
pub(crate) use leptos_use_window::*;
#[allow(unused_imports)]
pub(crate) use wasm_js_macros::*;
