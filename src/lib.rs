#![feature(doc_cfg)]
//! Collection of essential Leptos utilities inspired by SolidJS USE / VueUse

pub mod core;
#[cfg(feature = "docs")]
pub mod docs;
#[cfg(feature = "math")]
pub mod math;
#[cfg(feature = "storage")]
pub mod storage;
pub mod utils;

#[cfg(web_sys_unstable_apis)]
mod use_element_size;
#[cfg(web_sys_unstable_apis)]
mod use_resize_observer;

#[cfg(web_sys_unstable_apis)]
pub use use_element_size::*;
#[cfg(web_sys_unstable_apis)]
pub use use_resize_observer::*;

mod on_click_outside;
mod use_breakpoints;
mod use_css_var;
mod use_debounce_fn;
mod use_element_visibility;
mod use_event_listener;
mod use_favicon;
mod use_intersection_observer;
mod use_interval;
mod use_interval_fn;
mod use_media_query;
mod use_mouse;
mod use_mutation_observer;
mod use_preferred_contrast;
mod use_preferred_dark;
mod use_scroll;
mod use_supported;
mod use_throttle_fn;
mod watch;
mod watch_debounced;
mod watch_pausable;
mod watch_throttled;
mod whenever;

pub use on_click_outside::*;
pub use use_breakpoints::*;
pub use use_css_var::*;
pub use use_debounce_fn::*;
pub use use_element_visibility::*;
pub use use_event_listener::*;
pub use use_favicon::*;
pub use use_intersection_observer::*;
pub use use_interval::*;
pub use use_interval_fn::*;
pub use use_media_query::*;
pub use use_mouse::*;
pub use use_mutation_observer::*;
pub use use_preferred_contrast::*;
pub use use_preferred_dark::*;
pub use use_scroll::*;
pub use use_supported::*;
pub use use_throttle_fn::*;
pub use watch::*;
pub use watch_debounced::*;
pub use watch_pausable::*;
pub use watch_throttled::*;
pub use whenever::*;
