#![allow(unexpected_cfgs, clippy::extra_unused_lifetimes)]
//! Shorthand for watching a signal to be `true`

mod whenever;
pub use leptos_use_watch_with_options::{
    CallFilter, DebounceOptions, FilterOptions, NoFilter, ThrottleOptions, WatchOptions,
};
pub use whenever::*;

// Dependencies used through `crate::…` paths.
pub(crate) use leptos_use_watch_with_options::*;
