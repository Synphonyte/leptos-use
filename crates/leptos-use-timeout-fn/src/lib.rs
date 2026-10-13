#![allow(unexpected_cfgs, clippy::extra_unused_lifetimes)]
//! Wrapper for `setTimeout` with controls

mod use_timeout_fn;
pub use use_timeout_fn::*;

// Path shims so the sources keep their original `crate::…` paths.
#[allow(unused_imports)]
pub(crate) use sendwrap_fn::*;
