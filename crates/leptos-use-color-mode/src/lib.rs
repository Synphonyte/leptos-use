#![allow(unexpected_cfgs, clippy::extra_unused_lifetimes)]
//! Reactive color mode (dark / light / customs) with auto data persistence

mod options;
mod return_type;
mod url;
mod use_color_mode;
pub use options::*;
pub use return_type::*;
pub use use_color_mode::*;

// Dependencies used through `crate::…` paths.
pub(crate) use leptos_use_cookie::*;
pub(crate) use leptos_use_element::*;
pub(crate) use leptos_use_header::*;
pub(crate) use leptos_use_maybe_rw_signal::*;
pub(crate) use leptos_use_preferences::*;
pub(crate) use leptos_use_storage::*;
pub(crate) use leptos_use_sync_signal::*;
