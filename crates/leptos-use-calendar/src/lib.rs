#![allow(unexpected_cfgs, clippy::extra_unused_lifetimes)]
//! Create bare-bone calendar data to use in your component

mod options;
mod return_type;
mod use_calendar;
pub use options::*;
pub use return_type::*;
pub use use_calendar::*;

// Dependencies used through `crate::…` paths.
pub(crate) use leptos_use_maybe_rw_signal::*;
