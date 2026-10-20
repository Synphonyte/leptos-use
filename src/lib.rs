#![allow(unexpected_cfgs, clippy::extra_unused_lifetimes, deprecated)]
//! Collection of essential Leptos utilities inspired by SolidJS USE / VueUse
//!
//! # Deprecated
//!
//! `leptos-use` has been split into micro crates: every function now lives in its own crate
//! (for example `use_mouse` in [`leptos-use-mouse`](https://docs.rs/leptos-use-mouse)).
//! This crate only re-exports them and will not receive new functionality.
//! Every function in here is deprecated and its deprecation note tells you which crate to
//! add instead. See <https://leptos-use.rs> for the full list.

mod deprecated;

pub mod core {
    #[cfg(feature = "element")]
    pub use crate::deprecated::leptos_use_element::element_maybe_signal::{
        el_by_sel, el_signal_by_sel,
    };
    #[cfg(feature = "element")]
    pub use crate::deprecated::leptos_use_element::elements_maybe_signal::{
        els_by_sel, els_signal_by_sel,
    };
    pub use ::leptos_use_core::*;
    #[cfg(feature = "element")]
    pub use ::leptos_use_element::{
        ElementMarker, ElementMaybeSignal, ElementMaybeSignalType, ElementsMaybeSignal,
        ElementsMaybeSignalType, IntoElementMaybeSignal, IntoElementMaybeSignalType,
        IntoElementsMaybeSignal, IntoElementsMaybeSignalType, OptionSendWrapperSignalMarker,
        OptionSignalMarker, SendWrapperSignalMarker, SignalMarker, SignalStrMarker,
        SignalVecMarker, SignalVecOptionMarker, SignalVecSendWrapperMarker, StrIterMarker,
        StrMarker, VecSignalMarker, VecSignalOptionMarker, VecSignalVecOptionMarker,
    };
    pub use ::leptos_use_maybe_rw_signal::*;
    pub use ::leptos_use_option_local_signal::*;
    pub use ::leptos_use_rw_signal::*;
}

pub mod utils {
    pub use crate::deprecated::leptos_use_filters::filters::{
        create_filter_wrapper, create_filter_wrapper_with_arg,
    };
    #[cfg(all(feature = "ssr", any(feature = "axum", feature = "actix")))]
    pub use crate::deprecated::leptos_use_header::header::header;
    pub use ::leptos_use_filters::*;
    #[cfg(feature = "is")]
    pub use ::leptos_use_on_click_outside::IS_IOS;
}

#[cfg(feature = "storage")]
pub mod storage {
    pub use crate::deprecated::leptos_use_storage::use_local_storage::{
        use_local_storage, use_local_storage_with_options,
    };
    pub use crate::deprecated::leptos_use_storage::use_session_storage::{
        use_session_storage, use_session_storage_with_options,
    };
    pub use crate::deprecated::leptos_use_storage::use_storage::{
        use_storage, use_storage_with_options,
    };
    pub use ::leptos_use_storage::*;
}

#[cfg(feature = "math")]
pub mod math {
    pub use crate::deprecated::leptos_use_math::use_abs::use_abs;
    pub use crate::deprecated::leptos_use_math::use_and::use_and;
    pub use crate::deprecated::leptos_use_math::use_ceil::use_ceil;
    pub use crate::deprecated::leptos_use_math::use_floor::use_floor;
    pub use crate::deprecated::leptos_use_math::use_max::use_max;
    pub use crate::deprecated::leptos_use_math::use_min::use_min;
    pub use crate::deprecated::leptos_use_math::use_not::use_not;
    pub use crate::deprecated::leptos_use_math::use_or::use_or;
    pub use crate::deprecated::leptos_use_math::use_round::use_round;
}

#[cfg(feature = "docs")]
pub mod docs {
    pub use crate::deprecated::leptos_use_docs::demo::demo_or_body;
    pub use ::leptos_use_docs::*;
}

pub use ::leptos_use_derive_signal::use_derive_signal;
pub use ::leptos_use_filters::filter_builder_methods;
pub use ::sendwrap_fn::sendwrap_fn;
pub use ::wasm_js_macros::{js, js_fut};
pub use core::ReconnectLimit;

#[cfg(feature = "use_active_element")]
pub use crate::deprecated::leptos_use_active_element::use_active_element::use_active_element;
#[cfg(feature = "use_breakpoints")]
pub use crate::deprecated::leptos_use_breakpoints::use_breakpoints::{
    breakpoints_ant_design, breakpoints_bootstrap_v5, breakpoints_master_css, breakpoints_material,
    breakpoints_quasar, breakpoints_semantic, breakpoints_tailwind, use_breakpoints,
};
#[cfg(feature = "use_broadcast_channel")]
pub use crate::deprecated::leptos_use_broadcast_channel::use_broadcast_channel::use_broadcast_channel;
#[cfg(feature = "use_calendar")]
pub use crate::deprecated::leptos_use_calendar::use_calendar::{
    use_calendar, use_calendar_with_options,
};
#[cfg(feature = "use_clipboard")]
pub use crate::deprecated::leptos_use_clipboard::use_clipboard::{
    use_clipboard, use_clipboard_with_options,
};
#[cfg(feature = "use_color_mode")]
pub use crate::deprecated::leptos_use_color_mode::use_color_mode::{
    use_color_mode, use_color_mode_with_options,
};
#[cfg(feature = "use_cookie")]
pub use crate::deprecated::leptos_use_cookie::use_cookie::{use_cookie, use_cookie_with_options};
#[cfg(feature = "use_css_var")]
pub use crate::deprecated::leptos_use_css_var::use_css_var::{
    use_css_var, use_css_var_with_options,
};
#[cfg(feature = "use_cycle_list")]
pub use crate::deprecated::leptos_use_cycle_list::use_cycle_list::{
    use_cycle_list, use_cycle_list_with_options,
};
#[cfg(feature = "use_device_orientation")]
pub use crate::deprecated::leptos_use_device_orientation::use_device_orientation::use_device_orientation;
#[cfg(feature = "use_device_pixel_ratio")]
pub use crate::deprecated::leptos_use_device_pixel_ratio::use_device_pixel_ratio::use_device_pixel_ratio;
#[cfg(feature = "use_display_media")]
pub use crate::deprecated::leptos_use_display_media::use_display_media::{
    use_display_media, use_display_media_with_options,
};
#[cfg(feature = "use_document")]
pub use crate::deprecated::leptos_use_document::use_document::use_document;
#[cfg(feature = "use_document_visibility")]
pub use crate::deprecated::leptos_use_document_visibility::use_document_visibility::use_document_visibility;
#[cfg(feature = "use_draggable")]
pub use crate::deprecated::leptos_use_draggable::use_draggable::{
    use_draggable, use_draggable_with_options,
};
#[cfg(feature = "use_drop_zone")]
pub use crate::deprecated::leptos_use_drop_zone::use_drop_zone::{
    use_drop_zone, use_drop_zone_with_options,
};
#[cfg(any(feature = "element", feature = "use_mutation_observer"))]
pub use crate::deprecated::leptos_use_element::element_maybe_signal::{
    el_by_sel, el_signal_by_sel,
};
#[cfg(any(feature = "element", feature = "use_mutation_observer"))]
pub use crate::deprecated::leptos_use_element::elements_maybe_signal::{
    els_by_sel, els_signal_by_sel,
};
#[cfg(any(feature = "element", feature = "use_mutation_observer"))]
pub use crate::deprecated::leptos_use_element::use_mutation_observer::{
    use_mutation_observer, use_mutation_observer_with_options,
};
#[cfg(feature = "use_element_bounding")]
pub use crate::deprecated::leptos_use_element_bounding::use_element_bounding::{
    use_element_bounding, use_element_bounding_with_options,
};
#[cfg(feature = "use_element_hover")]
pub use crate::deprecated::leptos_use_element_hover::use_element_hover::{
    use_element_hover, use_element_hover_with_options,
};
#[cfg(feature = "use_element_size")]
pub use crate::deprecated::leptos_use_element_size::use_element_size::{
    use_element_size, use_element_size_with_options,
};
#[cfg(feature = "use_element_visibility")]
pub use crate::deprecated::leptos_use_element_visibility::use_element_visibility::{
    use_element_visibility, use_element_visibility_with_options,
};
#[cfg(feature = "use_event_listener")]
pub use crate::deprecated::leptos_use_event_listener::use_event_listener::{
    use_event_listener, use_event_listener_with_options,
};
#[cfg(feature = "use_event_source")]
pub use crate::deprecated::leptos_use_event_source::use_event_source::{
    use_event_source, use_event_source_with_options,
};
#[cfg(feature = "use_favicon")]
pub use crate::deprecated::leptos_use_favicon::use_favicon::{
    use_favicon, use_favicon_with_options,
};
#[cfg(any(feature = "use_debounce_fn", feature = "use_throttle_fn"))]
pub use crate::deprecated::leptos_use_fn_debounce_throttle::use_debounce_fn::{
    use_debounce_fn, use_debounce_fn_with_arg, use_debounce_fn_with_arg_and_options,
    use_debounce_fn_with_options,
};
#[cfg(any(feature = "use_debounce_fn", feature = "use_throttle_fn"))]
pub use crate::deprecated::leptos_use_fn_debounce_throttle::use_throttle_fn::{
    use_throttle_fn, use_throttle_fn_with_arg, use_throttle_fn_with_arg_and_options,
    use_throttle_fn_with_options,
};
#[cfg(feature = "use_geolocation")]
pub use crate::deprecated::leptos_use_geolocation::use_geolocation::{
    use_geolocation, use_geolocation_with_options,
};
#[cfg(feature = "use_idle")]
pub use crate::deprecated::leptos_use_idle::use_idle::{use_idle, use_idle_with_options};
#[cfg(feature = "use_infinite_scroll")]
pub use crate::deprecated::leptos_use_infinite_scroll::use_infinite_scroll::{
    use_infinite_scroll, use_infinite_scroll_with_options,
};
#[cfg(feature = "use_intersection_observer")]
pub use crate::deprecated::leptos_use_intersection_observer::use_intersection_observer::{
    use_intersection_observer, use_intersection_observer_with_options,
};
#[cfg(feature = "use_interval")]
pub use crate::deprecated::leptos_use_interval::use_interval::{
    use_interval, use_interval_with_options,
};
#[cfg(feature = "use_interval_fn")]
pub use crate::deprecated::leptos_use_interval_fn::use_interval_fn::{
    use_interval_fn, use_interval_fn_with_options,
};
#[cfg(feature = "use_intl_datetime_format")]
pub use crate::deprecated::leptos_use_intl_datetime_format::use_intl_datetime_format::use_intl_datetime_format;
#[cfg(feature = "use_intl_number_format")]
pub use crate::deprecated::leptos_use_intl_number_format::use_intl_number_format::use_intl_number_format;
#[cfg(any(
    feature = "is_ok",
    feature = "is_err",
    feature = "is_some",
    feature = "is_none"
))]
pub use crate::deprecated::leptos_use_is::is_err::is_err;
#[cfg(any(
    feature = "is_ok",
    feature = "is_err",
    feature = "is_some",
    feature = "is_none"
))]
pub use crate::deprecated::leptos_use_is::is_none::is_none;
#[cfg(any(
    feature = "is_ok",
    feature = "is_err",
    feature = "is_some",
    feature = "is_none"
))]
pub use crate::deprecated::leptos_use_is::is_ok::is_ok;
#[cfg(any(
    feature = "is_ok",
    feature = "is_err",
    feature = "is_some",
    feature = "is_none"
))]
pub use crate::deprecated::leptos_use_is::is_some::is_some;
#[cfg(feature = "use_locale")]
pub use crate::deprecated::leptos_use_locale::use_locale::{use_locale, use_locale_with_options};
#[cfg(feature = "use_locales")]
pub use crate::deprecated::leptos_use_locales::use_locales::{
    use_locales, use_locales_with_options,
};
#[cfg(feature = "use_media_query")]
pub use crate::deprecated::leptos_use_media_query::use_media_query::use_media_query;
#[cfg(feature = "use_mouse")]
pub use crate::deprecated::leptos_use_mouse::use_mouse::{use_mouse, use_mouse_with_options};
#[cfg(feature = "use_mouse_in_element")]
pub use crate::deprecated::leptos_use_mouse_in_element::use_mouse_in_element::{
    use_mouse_in_element, use_mouse_in_element_with_options,
};
#[cfg(any(feature = "on_click_outside", feature = "is"))]
pub use crate::deprecated::leptos_use_on_click_outside::on_click_outside::{
    on_click_outside, on_click_outside_with_options,
};
#[cfg(feature = "use_permission")]
pub use crate::deprecated::leptos_use_permission::use_permission::use_permission;
#[cfg(any(
    feature = "use_preferred_dark",
    feature = "use_preferred_contrast",
    feature = "use_prefers_reduced_motion"
))]
pub use crate::deprecated::leptos_use_preferences::use_preferred_contrast::use_preferred_contrast;
#[cfg(any(
    feature = "use_preferred_dark",
    feature = "use_preferred_contrast",
    feature = "use_prefers_reduced_motion"
))]
pub use crate::deprecated::leptos_use_preferences::use_preferred_dark::{
    use_preferred_dark, use_preferred_dark_with_options,
};
#[cfg(any(
    feature = "use_preferred_dark",
    feature = "use_preferred_contrast",
    feature = "use_prefers_reduced_motion"
))]
pub use crate::deprecated::leptos_use_preferences::use_prefers_reduced_motion::{
    use_prefers_reduced_motion, use_prefers_reduced_motion_with_options,
};
#[cfg(feature = "use_raf_fn")]
pub use crate::deprecated::leptos_use_raf_fn::use_raf_fn::{use_raf_fn, use_raf_fn_with_options};
#[cfg(feature = "use_resize_observer")]
pub use crate::deprecated::leptos_use_resize_observer::use_resize_observer::{
    use_resize_observer, use_resize_observer_with_options,
};
#[cfg(feature = "use_screen_orientation")]
pub use crate::deprecated::leptos_use_screen_orientation::use_screen_orientation::use_screen_orientation;
#[cfg(feature = "use_scroll")]
pub use crate::deprecated::leptos_use_scroll::use_scroll::{use_scroll, use_scroll_with_options};
#[cfg(feature = "use_service_worker")]
pub use crate::deprecated::leptos_use_service_worker::use_service_worker::{
    use_service_worker, use_service_worker_with_options,
};
#[cfg(any(feature = "signal_debounced", feature = "signal_throttled"))]
pub use crate::deprecated::leptos_use_signal_debounce_throttle::signal_debounced::{
    signal_debounced, signal_debounced_local, signal_debounced_local_with_options,
    signal_debounced_with_options,
};
#[cfg(any(feature = "signal_debounced", feature = "signal_throttled"))]
pub use crate::deprecated::leptos_use_signal_debounce_throttle::signal_throttled::{
    signal_throttled, signal_throttled_local, signal_throttled_local_with_options,
    signal_throttled_with_options,
};
#[cfg(feature = "use_sorted")]
pub use crate::deprecated::leptos_use_sorted::use_sorted::{
    use_sorted, use_sorted_by, use_sorted_by_key,
};
#[cfg(feature = "use_supported")]
pub use crate::deprecated::leptos_use_supported::use_supported::use_supported;
#[cfg(feature = "sync_signal")]
pub use crate::deprecated::leptos_use_sync_signal::sync_signal::{
    sync_signal, sync_signal_with_options,
};
#[cfg(feature = "use_textarea_autosize")]
pub use crate::deprecated::leptos_use_textarea_autosize::use_textarea_autosize::{
    use_textarea_autosize, use_textarea_autosize_with_options,
};
#[cfg(feature = "use_timeout_fn")]
pub use crate::deprecated::leptos_use_timeout_fn::use_timeout_fn::use_timeout_fn;
#[cfg(feature = "use_timestamp")]
pub use crate::deprecated::leptos_use_timestamp::use_timestamp::{
    use_timestamp, use_timestamp_with_controls, use_timestamp_with_controls_and_options,
    use_timestamp_with_options,
};
#[cfg(feature = "use_to_string")]
pub use crate::deprecated::leptos_use_to_string::use_to_string::use_to_string;
#[cfg(feature = "use_toggle")]
pub use crate::deprecated::leptos_use_toggle::use_toggle::use_toggle;
#[cfg(feature = "use_user_media")]
pub use crate::deprecated::leptos_use_user_media::use_user_media::{
    use_user_media, use_user_media_with_options,
};
#[cfg(any(feature = "watch_debounced", feature = "watch_throttled"))]
pub use crate::deprecated::leptos_use_watch_debounce_throttle::watch_debounced::{
    watch_debounced, watch_debounced_with_options,
};
#[cfg(any(feature = "watch_debounced", feature = "watch_throttled"))]
pub use crate::deprecated::leptos_use_watch_debounce_throttle::watch_throttled::{
    watch_throttled, watch_throttled_with_options,
};
#[cfg(feature = "watch_pausable")]
pub use crate::deprecated::leptos_use_watch_pausable::watch_pausable::{
    watch_pausable, watch_pausable_with_options,
};
#[cfg(feature = "watch_with_options")]
pub use crate::deprecated::leptos_use_watch_with_options::watch_with_options::watch_with_options;
#[cfg(feature = "use_web_lock")]
#[cfg(web_sys_unstable_apis)]
pub use crate::deprecated::leptos_use_web_lock::use_web_lock::{
    use_web_lock, use_web_lock_with_options,
};
#[cfg(feature = "use_web_notification")]
pub use crate::deprecated::leptos_use_web_notification::use_web_notification::{
    use_web_notification, use_web_notification_with_options,
};
#[cfg(feature = "use_websocket")]
pub use crate::deprecated::leptos_use_websocket::use_websocket::{
    use_websocket, use_websocket_with_options,
};
#[cfg(feature = "whenever")]
pub use crate::deprecated::leptos_use_whenever::whenever::{whenever, whenever_with_options};
#[cfg(feature = "use_window")]
pub use crate::deprecated::leptos_use_window::use_window::use_window;
#[cfg(feature = "use_window_focus")]
pub use crate::deprecated::leptos_use_window_focus::use_window_focus::use_window_focus;
#[cfg(feature = "use_window_scroll")]
pub use crate::deprecated::leptos_use_window_scroll::use_window_scroll::use_window_scroll;
#[cfg(feature = "use_window_size")]
pub use crate::deprecated::leptos_use_window_size::use_window_size::{
    use_window_size, use_window_size_with_options,
};
#[cfg(feature = "use_breakpoints")]
pub use ::leptos_use_breakpoints::*;
#[cfg(feature = "use_broadcast_channel")]
pub use ::leptos_use_broadcast_channel::*;
#[cfg(feature = "use_calendar")]
pub use ::leptos_use_calendar::*;
#[cfg(feature = "use_clipboard")]
pub use ::leptos_use_clipboard::*;
#[cfg(feature = "use_color_mode")]
pub use ::leptos_use_color_mode::*;
#[cfg(feature = "use_cookie")]
pub use ::leptos_use_cookie::*;
#[cfg(feature = "use_css_var")]
pub use ::leptos_use_css_var::*;
#[cfg(feature = "use_cycle_list")]
pub use ::leptos_use_cycle_list::*;
#[cfg(feature = "use_device_orientation")]
pub use ::leptos_use_device_orientation::*;
#[cfg(feature = "use_display_media")]
pub use ::leptos_use_display_media::*;
#[cfg(feature = "use_document")]
pub use ::leptos_use_document::*;
#[cfg(feature = "use_draggable")]
pub use ::leptos_use_draggable::*;
#[cfg(feature = "use_drop_zone")]
pub use ::leptos_use_drop_zone::*;
#[cfg(any(feature = "element", feature = "use_mutation_observer"))]
pub use ::leptos_use_element::*;
#[cfg(feature = "use_element_bounding")]
pub use ::leptos_use_element_bounding::*;
#[cfg(feature = "use_element_hover")]
pub use ::leptos_use_element_hover::*;
#[cfg(feature = "use_element_size")]
pub use ::leptos_use_element_size::*;
#[cfg(feature = "use_element_visibility")]
pub use ::leptos_use_element_visibility::*;
#[cfg(feature = "use_event_listener")]
pub use ::leptos_use_event_listener::*;
#[cfg(feature = "use_event_source")]
pub use ::leptos_use_event_source::*;
#[cfg(feature = "use_favicon")]
pub use ::leptos_use_favicon::*;
#[cfg(any(feature = "use_debounce_fn", feature = "use_throttle_fn"))]
pub use ::leptos_use_fn_debounce_throttle::*;
#[cfg(feature = "use_geolocation")]
pub use ::leptos_use_geolocation::*;
#[cfg(feature = "use_idle")]
pub use ::leptos_use_idle::*;
#[cfg(feature = "use_infinite_scroll")]
pub use ::leptos_use_infinite_scroll::*;
#[cfg(feature = "use_intersection_observer")]
pub use ::leptos_use_intersection_observer::*;
#[cfg(feature = "use_interval")]
pub use ::leptos_use_interval::*;
#[cfg(feature = "use_interval_fn")]
pub use ::leptos_use_interval_fn::*;
#[cfg(feature = "use_intl_datetime_format")]
#[allow(ambiguous_glob_reexports)]
pub use ::leptos_use_intl_datetime_format::*;
#[cfg(feature = "use_intl_number_format")]
pub use ::leptos_use_intl_number_format::*;
#[cfg(feature = "use_locale")]
pub use ::leptos_use_locale::*;
#[cfg(feature = "use_locales")]
#[allow(unused_imports)]
pub use ::leptos_use_locales::*;
#[cfg(feature = "use_mouse")]
pub use ::leptos_use_mouse::*;
#[cfg(feature = "use_mouse_in_element")]
pub use ::leptos_use_mouse_in_element::*;
#[cfg(any(feature = "on_click_outside", feature = "is"))]
pub use ::leptos_use_on_click_outside::*;
#[cfg(feature = "use_permission")]
pub use ::leptos_use_permission::*;
#[cfg(any(
    feature = "use_preferred_dark",
    feature = "use_preferred_contrast",
    feature = "use_prefers_reduced_motion"
))]
pub use ::leptos_use_preferences::*;
#[cfg(feature = "use_raf_fn")]
pub use ::leptos_use_raf_fn::*;
#[cfg(feature = "use_resize_observer")]
pub use ::leptos_use_resize_observer::*;
#[cfg(feature = "use_screen_orientation")]
pub use ::leptos_use_screen_orientation::*;
#[cfg(feature = "use_scroll")]
pub use ::leptos_use_scroll::*;
#[cfg(feature = "use_service_worker")]
pub use ::leptos_use_service_worker::*;
#[cfg(feature = "sync_signal")]
pub use ::leptos_use_sync_signal::*;
#[cfg(feature = "use_textarea_autosize")]
pub use ::leptos_use_textarea_autosize::*;
#[cfg(feature = "use_timeout_fn")]
pub use ::leptos_use_timeout_fn::*;
#[cfg(feature = "use_timestamp")]
pub use ::leptos_use_timestamp::*;
#[cfg(feature = "use_toggle")]
pub use ::leptos_use_toggle::*;
#[cfg(feature = "use_user_media")]
pub use ::leptos_use_user_media::*;
#[cfg(any(feature = "watch_debounced", feature = "watch_throttled"))]
pub use ::leptos_use_watch_debounce_throttle::*;
#[cfg(feature = "watch_pausable")]
pub use ::leptos_use_watch_pausable::*;
#[cfg(feature = "watch_with_options")]
#[allow(unused_imports)]
pub use ::leptos_use_watch_with_options::*;
#[cfg(feature = "use_web_lock")]
#[cfg(web_sys_unstable_apis)]
pub use ::leptos_use_web_lock::*;
#[cfg(feature = "use_web_notification")]
pub use ::leptos_use_web_notification::*;
#[cfg(feature = "use_websocket")]
pub use ::leptos_use_websocket::*;
#[cfg(feature = "use_window")]
#[allow(unused_imports)]
pub use ::leptos_use_window::*;
#[cfg(feature = "use_window_size")]
pub use ::leptos_use_window_size::*;
#[cfg(feature = "use_web_lock")]
#[cfg(not(web_sys_unstable_apis))]
use leptos_use_web_lock as _;
