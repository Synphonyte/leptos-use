#![feature(doc_cfg)]
//! Collection of essential Leptos utilities inspired by SolidJS USE / VueUse

pub mod core;
#[cfg(feature = "docs")]
pub mod docs;
#[cfg(feature = "math")]
pub mod math;
mod use_debounce_fn;
#[cfg(web_sys_unstable_apis)]
mod use_element_size;
mod use_event_listener;
mod use_mouse;
#[cfg(web_sys_unstable_apis)]
mod use_resize_observer;
mod use_scroll;
mod use_supported;
mod use_throttle_fn;
pub mod utils;
mod watch;

pub use use_debounce_fn::*;
#[cfg(web_sys_unstable_apis)]
pub use use_element_size::*;
pub use use_event_listener::*;
pub use use_mouse::*;
#[cfg(web_sys_unstable_apis)]
pub use use_resize_observer::*;
pub use use_scroll::*;
pub use use_supported::*;
pub use use_throttle_fn::*;
pub use watch::*;

extern crate self as leptos_use;
