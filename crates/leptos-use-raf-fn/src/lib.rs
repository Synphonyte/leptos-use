#![allow(unexpected_cfgs, clippy::extra_unused_lifetimes)]
//! Call function on every requestAnimationFrame

mod options;
mod use_raf_fn;
pub use leptos_use_filters::Pausable;
pub use options::*;
pub use use_raf_fn::*;

// Dependencies used through `crate::…` paths.
pub(crate) use sendwrap_fn::*;
