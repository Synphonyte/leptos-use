#![allow(unexpected_cfgs, clippy::extra_unused_lifetimes)]
//! SSR-friendly and reactive cookie access

mod use_cookie;
pub use use_cookie::*;

// Dependencies used through `crate::…` paths.
#[cfg(not(feature = "ssr"))]
pub(crate) use leptos_use_broadcast_channel::*;
pub(crate) use leptos_use_header::*;
pub(crate) use leptos_use_now::*;
#[cfg(not(feature = "ssr"))]
pub(crate) use leptos_use_watch_with_options::*;
