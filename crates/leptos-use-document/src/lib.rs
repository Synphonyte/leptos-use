#![allow(unexpected_cfgs, clippy::extra_unused_lifetimes)]
//! SSR safe `document()`

mod ssr_safe_method;
mod use_document;
#[allow(unused_imports)]
pub(crate) use ssr_safe_method::*;
pub use use_document::*;
