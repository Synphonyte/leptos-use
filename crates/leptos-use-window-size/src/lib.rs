#![allow(unexpected_cfgs, clippy::extra_unused_lifetimes)]
//! Reactive window size

mod options;
mod return_type;
mod use_window_size;
pub use leptos_use_core::Size;
pub use options::*;
pub use return_type::*;
pub use use_window_size::*;

// Dependencies used through `crate::…` paths.
pub(crate) use leptos_use_event_listener::*;
pub(crate) use leptos_use_media_query::*;
pub(crate) use leptos_use_window::*;
