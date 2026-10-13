#![allow(unexpected_cfgs, clippy::extra_unused_lifetimes)]
//! Pausable [`watch`]

mod watch_pausable;
pub use watch_pausable::*;

// Path shims so the sources keep their original `crate::…` paths.
#[allow(unused_imports)]
pub(crate) use leptos_use_watch_with_options::*;
