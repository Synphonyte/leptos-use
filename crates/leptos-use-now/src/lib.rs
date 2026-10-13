#![allow(unexpected_cfgs, clippy::extra_unused_lifetimes)]
//! SSR-safe `Date.now()` used by the leptos-use crates

#[allow(unused_imports)]
pub(crate) mod core {
    pub(crate) mod datetime;
    pub use datetime::*;
}
#[allow(unused_imports)]
pub use core::datetime::*;
