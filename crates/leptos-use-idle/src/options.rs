use crate::{DebounceOptions, FilterOptions, ThrottleOptions, filter_builder_methods};
use default_struct_builder::DefaultBuilder;
use leptos::reactive::wrappers::read::Signal;

/// Options for [`use_idle_with_options`](crate::use_idle_with_options).
#[derive(DefaultBuilder)]
pub struct UseIdleOptions {
    /// Event names to listen to on `window` for detected user activity.
    /// Default: `vec!["mousemove", "mousedown", "resize", "keydown", "touchstart", "wheel"]`.
    pub(crate) events: Vec<String>,

    /// Whether to listen for document visibility change.
    /// Defaults to `true`.
    pub(crate) listen_for_visibility_change: bool,

    /// Initial state of the returned `idle`.
    /// Defaults to `false`.
    pub(crate) initial_state: bool,

    /// Allows to debounce or throttle the event listener that is called for
    /// every event (from `events`). Defaults to a throttle by 50ms.
    pub(crate) filter: FilterOptions,
}

impl Default for UseIdleOptions {
    fn default() -> Self {
        Self {
            events: vec![
                "mousemove".to_string(),
                "mousedown".to_string(),
                "resize".to_string(),
                "keydown".to_string(),
                "touchstart".to_string(),
                "wheel".to_string(),
            ],
            listen_for_visibility_change: true,
            initial_state: false,
            filter: FilterOptions::throttle(50.0),
        }
    }
}

impl UseIdleOptions {
    filter_builder_methods!(
        /// the event listener
        filter
    );
}
