#![allow(unexpected_cfgs, clippy::extra_unused_lifetimes)]
//! SSR safe `window()`

mod use_window;
pub use use_window::*;

// Dependencies used through `crate::…` paths.
#[allow(unused_imports)]
pub(crate) use leptos_use_document::*;
