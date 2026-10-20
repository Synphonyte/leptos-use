#![allow(unexpected_cfgs, clippy::extra_unused_lifetimes)]
//! SSR-safe `Date.now()` used by the leptos-use crates

mod datetime;
pub use datetime::*;
