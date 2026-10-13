#![allow(unexpected_cfgs, clippy::extra_unused_lifetimes)]
//! Reactive color mode (dark / light / customs) with auto data persistence

mod url;
mod use_color_mode;
pub use use_color_mode::*;

// Dependencies used through `crate::…` paths.
#[allow(unused_imports)]
pub(crate) use leptos_use_cookie::*;
#[allow(unused_imports)]
pub(crate) use leptos_use_element::*;
#[allow(unused_imports)]
pub(crate) use leptos_use_header::*;
#[allow(unused_imports)]
pub(crate) use leptos_use_maybe_rw_signal::*;
#[allow(unused_imports)]
pub(crate) use leptos_use_preferences::*;
#[allow(unused_imports)]
pub(crate) use leptos_use_storage::*;
#[allow(unused_imports)]
pub(crate) use leptos_use_sync_signal::*;
