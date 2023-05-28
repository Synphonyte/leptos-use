//! Collection of essential Leptos utilities inspired by SolidJS USE / VueUse

pub mod core;
mod use_debounce_fn;
mod use_event_listener;
mod use_scroll;
mod use_throttle_fn;
pub mod utils;

#[cfg(feature = "docs")]
pub mod docs;

pub use use_debounce_fn::*;
pub use use_event_listener::*;
pub use use_scroll::*;
pub use use_throttle_fn::*;

extern crate self as leptos_use;
