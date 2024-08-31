#![allow(unexpected_cfgs)]
// #![feature(doc_cfg)]
//! Collection of essential Leptos utilities inspired by SolidJS USE / VueUse

pub mod core;
#[cfg(feature = "docs")]
pub mod docs;
#[cfg(feature = "math")]
pub mod math;
#[cfg(feature = "storage")]
pub mod storage;
pub mod utils;

pub use core::ReconnectLimit;

// #[cfg(web_sys_unstable_apis)]
// mod use_webtransport;
// #[cfg(web_sys_unstable_apis)]
// pub use use_webtransport::*;

#[cfg(feature = "is_err")]
mod is_err;
#[cfg(feature = "is_none")]
mod is_none;
#[cfg(feature = "is_ok")]
mod is_ok;
#[cfg(feature = "is_some")]
mod is_some;
#[cfg(feature = "on_click_outside")]
mod on_click_outside;
#[cfg(feature = "use_window_size")]
mod use_window_size;
#[cfg(feature = "signal_debounced")]
mod signal_debounced;
#[cfg(feature = "signal_throttled")]
mod signal_throttled;
#[cfg(feature = "sync_signal")]
mod sync_signal;
#[cfg(feature = "use_active_element")]
mod use_active_element;
#[cfg(feature = "use_breakpoints")]
mod use_breakpoints;
#[cfg(feature = "use_broadcast_channel")]
mod use_broadcast_channel;
#[cfg(feature = "use_clipboard")]
mod use_clipboard;
#[cfg(feature = "use_color_mode")]
mod use_color_mode;
#[cfg(feature = "use_cookie")]
mod use_cookie;
#[cfg(feature = "use_css_var")]
mod use_css_var;
#[cfg(feature = "use_cycle_list")]
mod use_cycle_list;
#[cfg(feature = "use_debounce_fn")]
mod use_debounce_fn;
#[cfg(feature = "use_device_orientation")]
mod use_device_orientation;
#[cfg(feature = "use_device_pixel_ratio")]
mod use_device_pixel_ratio;
#[cfg(feature = "use_display_media")]
mod use_display_media;
#[cfg(feature = "use_document")]
mod use_document;
#[cfg(feature = "use_document_visibility")]
mod use_document_visibility;
#[cfg(feature = "use_draggable")]
mod use_draggable;
#[cfg(feature = "use_drop_zone")]
mod use_drop_zone;
#[cfg(feature = "use_element_bounding")]
mod use_element_bounding;
#[cfg(feature = "use_element_hover")]
mod use_element_hover;
#[cfg(feature = "use_element_size")]
mod use_element_size;
#[cfg(feature = "use_element_visibility")]
mod use_element_visibility;
#[cfg(feature = "use_event_listener")]
mod use_event_listener;
#[cfg(feature = "use_event_source")]
mod use_event_source;
#[cfg(feature = "use_favicon")]
mod use_favicon;
#[cfg(feature = "use_geolocation")]
mod use_geolocation;
#[cfg(feature = "use_idle")]
mod use_idle;
#[cfg(feature = "use_infinite_scroll")]
mod use_infinite_scroll;
#[cfg(feature = "use_intersection_observer")]
mod use_intersection_observer;
#[cfg(feature = "use_interval")]
mod use_interval;
#[cfg(feature = "use_interval_fn")]
mod use_interval_fn;
#[cfg(feature = "use_intl_number_format")]
mod use_intl_number_format;
#[cfg(feature = "use_locale")]
mod use_locale;
#[cfg(feature = "use_locales")]
mod use_locales;
#[cfg(feature = "use_media_query")]
mod use_media_query;
#[cfg(feature = "use_mouse")]
mod use_mouse;
#[cfg(feature = "use_mouse_in_element")]
mod use_mouse_in_element;
#[cfg(feature = "use_mutation_observer")]
mod use_mutation_observer;
#[cfg(feature = "use_permission")]
mod use_permission;
#[cfg(feature = "use_preferred_contrast")]
mod use_preferred_contrast;
#[cfg(feature = "use_preferred_dark")]
mod use_preferred_dark;
#[cfg(feature = "use_prefers_reduced_motion")]
mod use_prefers_reduced_motion;
#[cfg(feature = "use_raf_fn")]
mod use_raf_fn;
#[cfg(feature = "use_resize_observer")]
mod use_resize_observer;
#[cfg(feature = "use_scroll")]
mod use_scroll;
#[cfg(feature = "use_service_worker")]
mod use_service_worker;
#[cfg(feature = "use_sorted")]
mod use_sorted;
#[cfg(feature = "use_supported")]
mod use_supported;
#[cfg(feature = "use_throttle_fn")]
mod use_throttle_fn;
#[cfg(feature = "use_timeout_fn")]
mod use_timeout_fn;
#[cfg(feature = "use_timestamp")]
mod use_timestamp;
#[cfg(feature = "use_to_string")]
mod use_to_string;
#[cfg(feature = "use_toggle")]
mod use_toggle;
#[cfg(feature = "use_user_media")]
mod use_user_media;
#[cfg(feature = "use_web_notification")]
mod use_web_notification;
#[cfg(feature = "use_websocket")]
mod use_websocket;
#[cfg(feature = "use_window")]
mod use_window;
#[cfg(feature = "use_window_focus")]
mod use_window_focus;
#[cfg(feature = "use_window_scroll")]
mod use_window_scroll;
#[cfg(feature = "watch_debounced")]
mod watch_debounced;
#[cfg(feature = "watch_pausable")]
mod watch_pausable;
#[cfg(feature = "watch_throttled")]
mod watch_throttled;
#[cfg(feature = "watch_with_options")]
mod watch_with_options;
#[cfg(feature = "whenever")]
mod whenever;

#[cfg(feature = "is_err")]
pub use is_err::*;
#[cfg(feature = "is_none")]
pub use is_none::*;
#[cfg(feature = "is_ok")]
pub use is_ok::*;
#[cfg(feature = "is_some")]
pub use is_some::*;
#[cfg(feature = "on_click_outside")]
pub use on_click_outside::*;
#[cfg(feature = "use_window_size")]
pub use use_window_size::*;
#[cfg(feature = "signal_debounced")]
pub use signal_debounced::*;
#[cfg(feature = "signal_throttled")]
pub use signal_throttled::*;
#[cfg(feature = "sync_signal")]
pub use sync_signal::*;
#[cfg(feature = "use_active_element")]
pub use use_active_element::*;
#[cfg(feature = "use_breakpoints")]
pub use use_breakpoints::*;
#[cfg(feature = "use_broadcast_channel")]
pub use use_broadcast_channel::*;
#[cfg(feature = "use_clipboard")]
pub use use_clipboard::*;
#[cfg(feature = "use_color_mode")]
pub use use_color_mode::*;
#[cfg(feature = "use_cookie")]
pub use use_cookie::*;
#[cfg(feature = "use_css_var")]
pub use use_css_var::*;
#[cfg(feature = "use_cycle_list")]
pub use use_cycle_list::*;
#[cfg(feature = "use_debounce_fn")]
pub use use_debounce_fn::*;
#[cfg(feature = "use_device_orientation")]
pub use use_device_orientation::*;
#[cfg(feature = "use_device_pixel_ratio")]
pub use use_device_pixel_ratio::*;
#[cfg(feature = "use_display_media")]
pub use use_display_media::*;
#[cfg(feature = "use_document")]
pub use use_document::*;
#[cfg(feature = "use_document_visibility")]
pub use use_document_visibility::*;
#[cfg(feature = "use_draggable")]
pub use use_draggable::*;
#[cfg(feature = "use_drop_zone")]
pub use use_drop_zone::*;
#[cfg(feature = "use_element_bounding")]
pub use use_element_bounding::*;
#[cfg(feature = "use_element_hover")]
pub use use_element_hover::*;
#[cfg(feature = "use_element_size")]
pub use use_element_size::*;
#[cfg(feature = "use_element_visibility")]
pub use use_element_visibility::*;
#[cfg(feature = "use_event_listener")]
pub use use_event_listener::*;
#[cfg(feature = "use_event_source")]
pub use use_event_source::*;
#[cfg(feature = "use_favicon")]
pub use use_favicon::*;
#[cfg(feature = "use_geolocation")]
pub use use_geolocation::*;
#[cfg(feature = "use_idle")]
pub use use_idle::*;
#[cfg(feature = "use_infinite_scroll")]
pub use use_infinite_scroll::*;
#[cfg(feature = "use_intersection_observer")]
pub use use_intersection_observer::*;
#[cfg(feature = "use_interval")]
pub use use_interval::*;
#[cfg(feature = "use_interval_fn")]
pub use use_interval_fn::*;
#[cfg(feature = "use_intl_number_format")]
pub use use_intl_number_format::*;
#[cfg(feature = "use_locale")]
pub use use_locale::*;
#[cfg(feature = "use_locales")]
pub use use_locales::*;
#[cfg(feature = "use_media_query")]
pub use use_media_query::*;
#[cfg(feature = "use_mouse")]
pub use use_mouse::*;
#[cfg(feature = "use_mouse_in_element")]
pub use use_mouse_in_element::*;
#[cfg(feature = "use_mutation_observer")]
pub use use_mutation_observer::*;
#[cfg(feature = "use_permission")]
pub use use_permission::*;
#[cfg(feature = "use_preferred_contrast")]
pub use use_preferred_contrast::*;
#[cfg(feature = "use_preferred_dark")]
pub use use_preferred_dark::*;
#[cfg(feature = "use_prefers_reduced_motion")]
pub use use_prefers_reduced_motion::*;
#[cfg(feature = "use_raf_fn")]
pub use use_raf_fn::*;
#[cfg(feature = "use_resize_observer")]
pub use use_resize_observer::*;
#[cfg(feature = "use_scroll")]
pub use use_scroll::*;
#[cfg(feature = "use_service_worker")]
pub use use_service_worker::*;
#[cfg(feature = "use_sorted")]
pub use use_sorted::*;
#[cfg(feature = "use_supported")]
pub use use_supported::*;
#[cfg(feature = "use_throttle_fn")]
pub use use_throttle_fn::*;
#[cfg(feature = "use_timeout_fn")]
pub use use_timeout_fn::*;
#[cfg(feature = "use_timestamp")]
pub use use_timestamp::*;
#[cfg(feature = "use_to_string")]
pub use use_to_string::*;
#[cfg(feature = "use_toggle")]
pub use use_toggle::*;
#[cfg(feature = "use_user_media")]
pub use use_user_media::*;
#[cfg(feature = "use_web_notification")]
pub use use_web_notification::*;
#[cfg(feature = "use_websocket")]
pub use use_websocket::*;
#[cfg(feature = "use_window")]
pub use use_window::*;
#[cfg(feature = "use_window_focus")]
pub use use_window_focus::*;
#[cfg(feature = "use_window_scroll")]
pub use use_window_scroll::*;
#[cfg(feature = "watch_debounced")]
pub use watch_debounced::*;
#[cfg(feature = "watch_pausable")]
pub use watch_pausable::*;
#[cfg(feature = "watch_throttled")]
pub use watch_throttled::*;
#[cfg(feature = "watch_with_options")]
pub use watch_with_options::*;
#[cfg(feature = "whenever")]
pub use whenever::*;
