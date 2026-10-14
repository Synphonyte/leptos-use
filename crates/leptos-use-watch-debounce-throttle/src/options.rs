use default_struct_builder::DefaultBuilder;
use leptos::prelude::*;

/// Options for [`watch_debounced_with_options`](crate::watch_debounced_with_options).
#[derive(DefaultBuilder, Default)]
pub struct WatchDebouncedOptions {
    /// If `immediate` is false, the `callback` will not run immediately but only after
    /// the first change is detected of any signal that is accessed in `deps`.
    /// Defaults to `false`.
    pub(crate) immediate: bool,

    /// The maximum time allowed to be delayed before the callback invoked.
    /// In milliseconds.
    /// Same as [`DebounceOptions::max_wait`](crate::DebounceOptions::max_wait)
    #[builder(into)]
    pub max_wait: Signal<Option<f64>>,
}

/// Options for [`watch_throttled_with_options`](crate::watch_throttled_with_options).
#[derive(DefaultBuilder)]
pub struct WatchThrottledOptions {
    /// If `immediate` is false, the `callback` will not run immediately but only after
    /// the first change is detected of any signal that is accessed in `deps`.
    /// Defaults to `false`.
    pub(crate) immediate: bool,

    /// Invoke on the trailing edge of the timeout. Defaults to `true`.
    pub trailing: bool,
    /// Invoke on the leading edge of the timeout. Defaults to `true`.
    pub leading: bool,
}

impl Default for WatchThrottledOptions {
    fn default() -> Self {
        Self {
            immediate: false,
            trailing: true,
            leading: true,
        }
    }
}
