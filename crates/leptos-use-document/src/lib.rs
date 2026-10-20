#![allow(unexpected_cfgs, clippy::extra_unused_lifetimes)]
//! SSR safe `document()`

mod ssr_safe_method;
mod use_document;
pub use use_document::*;

#[cfg(feature = "ssr")]
use leptos as _;
