#![allow(unexpected_cfgs, clippy::extra_unused_lifetimes)]
//! Pausable [`watch`]

mod return_type;
mod watch_pausable;
pub use leptos_use_watch_with_options::{
    CallFilter, DebounceOptions, FilterOptions, NoFilter, ThrottleOptions, WatchOptions,
};
pub use return_type::*;
pub use watch_pausable::*;

// Dependencies used through `crate::…` paths.
pub(crate) use leptos_use_watch_with_options::*;
