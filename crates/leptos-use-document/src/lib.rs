#![allow(unexpected_cfgs, clippy::extra_unused_lifetimes)]
//! SSR safe `document()`

mod use_document;
pub use use_document::*;

#[allow(unused_imports)]
pub(crate) mod core {
    pub(crate) mod ssr_safe_method;
    pub use ssr_safe_method::*;
}
