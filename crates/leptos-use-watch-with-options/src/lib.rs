#![allow(unexpected_cfgs, clippy::extra_unused_lifetimes)]
//! A version of `leptos::watch` but with additional options

mod options;
mod watch_with_options;
pub use options::*;
pub use watch_with_options::*;

// needed to name the filter type of `WatchOptions`
pub use leptos_use_filters::{CallFilter, FilterOptions, NoFilter};

// Dependencies used through `crate::…` paths.
pub(crate) use leptos_use_filters::*;
