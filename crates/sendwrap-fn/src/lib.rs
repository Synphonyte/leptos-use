#![allow(unexpected_cfgs, clippy::extra_unused_lifetimes)]
//! Macro to wrap closures in a `send_wrapper::SendWrapper` so they are `Send + Sync`

mod sendwrap_fn;
#[allow(unused_imports)]
pub use sendwrap_fn::*;
