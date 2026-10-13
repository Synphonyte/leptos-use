#![allow(unexpected_cfgs, clippy::extra_unused_lifetimes)]
//! Wrapper for `setTimeout` with controls

mod use_timeout_fn;
pub use use_timeout_fn::*;

// Dependencies used through `crate::…` paths.
#[cfg(not(feature = "ssr"))]
pub(crate) use sendwrap_fn::*;
