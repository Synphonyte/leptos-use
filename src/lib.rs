// #![feature(doc_cfg)]
//! Collection of essential Leptos utilities inspired by SolidJS USE / VueUse

use cfg_if::cfg_if;

pub mod core;
#[cfg(feature = "docs")]
pub mod docs;
#[cfg(feature = "math")]
pub mod math;
#[cfg(feature = "storage")]
pub mod storage;
pub mod utils;

cfg_if! { if #[cfg(web_sys_unstable_apis)] {
    mod use_element_size;
    mod use_resize_observer;
    mod use_webtransport;

    pub use use_element_size::*;
    pub use use_resize_observer::*;
    pub use use_webtransport::*;
}}

mod is_err;
mod is_none;
mod is_ok;
mod is_some;
mod on_click_outside;
mod use_document;
mod use_window;
mod use_geolocation;
mod signal_debounced;
mod signal_throttled;
mod use_active_element;
mod use_breakpoints;
mod use_color_mode;
mod use_css_var;
mod use_cycle_list;
mod use_debounce_fn;
mod use_document_visibility;
mod use_draggable;
mod use_drop_zone;
mod use_element_hover;
mod use_element_visibility;
mod use_event_listener;
mod use_favicon;
mod use_intersection_observer;
mod use_interval;
mod use_interval_fn;
mod use_intl_number_format;
mod use_media_query;
mod use_mouse;
mod use_mutation_observer;
mod use_preferred_contrast;
mod use_preferred_dark;
mod use_raf_fn;
mod use_scroll;
mod use_supported;
mod use_throttle_fn;
mod use_to_string;
mod use_websocket;
mod use_window_focus;
mod use_window_scroll;
mod watch_debounced;
mod watch_pausable;
mod watch_throttled;
mod watch_with_options;
mod whenever;

pub use is_err::*;
pub use is_none::*;
pub use is_ok::*;
pub use is_some::*;
pub use on_click_outside::*;
pub use use_document::*;
pub use use_window::*;
pub use use_geolocation::*;
pub use signal_debounced::*;
pub use signal_throttled::*;
pub use use_active_element::*;
pub use use_breakpoints::*;
pub use use_color_mode::*;
pub use use_css_var::*;
pub use use_cycle_list::*;
pub use use_debounce_fn::*;
pub use use_document_visibility::*;
pub use use_draggable::*;
pub use use_drop_zone::*;
pub use use_element_hover::*;
pub use use_element_visibility::*;
pub use use_event_listener::*;
pub use use_favicon::*;
pub use use_intersection_observer::*;
pub use use_interval::*;
pub use use_interval_fn::*;
pub use use_intl_number_format::*;
pub use use_media_query::*;
pub use use_mouse::*;
pub use use_mutation_observer::*;
pub use use_preferred_contrast::*;
pub use use_preferred_dark::*;
pub use use_raf_fn::*;
pub use use_scroll::*;
pub use use_supported::*;
pub use use_throttle_fn::*;
pub use use_to_string::*;
pub use use_websocket::*;
pub use use_webtransport::*;
pub use use_window_focus::*;
pub use use_window_scroll::*;
pub use watch_debounced::*;
pub use watch_pausable::*;
pub use watch_throttled::*;
pub use watch_with_options::*;
pub use whenever::*;
