#![allow(unexpected_cfgs, clippy::extra_unused_lifetimes)]
//! Read request headers during SSR (axum / actix) for the leptos-use crates

#[cfg(all(feature = "ssr", any(feature = "axum", feature = "actix")))]
mod header;
mod header_macro;
#[cfg(all(feature = "ssr", any(feature = "axum", feature = "actix")))]
pub use header::*;
