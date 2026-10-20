#![allow(unexpected_cfgs, clippy::extra_unused_lifetimes)]
//! Macro to wrap closures in a `send_wrapper::SendWrapper` so they are `Send + Sync`

mod sendwrap_fn;

#[doc(hidden)]
pub mod __private {
    pub use send_wrapper::SendWrapper;
}
