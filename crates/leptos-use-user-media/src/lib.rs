#![allow(unexpected_cfgs, clippy::extra_unused_lifetimes)]
//! Reactive [`mediaDevices.getUserMedia`](https://developer.mozilla.org/en-US/docs/Web/API/MediaDevices/getUserMedia) streaming

mod use_user_media;
pub use use_user_media::*;

#[allow(unused_imports)]
pub(crate) mod core {
    pub use leptos_use_maybe_rw_signal::*;
    pub use leptos_use_option_local_signal::*;
}

// Path shims so the sources keep their original `crate::…` paths.
#[allow(unused_imports)]
pub(crate) use leptos_use_maybe_rw_signal::*;
#[allow(unused_imports)]
pub(crate) use leptos_use_option_local_signal::*;
#[allow(unused_imports)]
pub(crate) use leptos_use_window as use_window;
#[allow(unused_imports)]
pub(crate) use leptos_use_window::*;
#[allow(unused_imports)]
pub(crate) use wasm_js_macros::*;
