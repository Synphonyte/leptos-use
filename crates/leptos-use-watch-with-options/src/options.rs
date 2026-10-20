use crate::{DebounceOptions, FilterOptions, NoFilter, ThrottleOptions};
use leptos::prelude::*;

/// Options for `watch_with_options`
///
/// The filter is a type parameter so that the debounce and throttle code is only compiled into
/// your app if you use it.
pub struct WatchOptions<F = NoFilter> {
    /// If `immediate` is true, the `callback` will run immediately.
    /// If it's `false, the `callback` will run only after
    /// the first change is detected of any signal that is accessed in `deps`.
    /// Defaults to `false`.
    pub(crate) immediate: bool,

    /// Allows to debounce or throttle the callback with a [`FilterOptions`]. Defaults to no filter.
    pub(crate) filter: F,
}

impl Default for WatchOptions {
    fn default() -> Self {
        Self {
            immediate: false,
            filter: NoFilter,
        }
    }
}

impl<F> WatchOptions<F> {
    /// If `immediate` is true, the `callback` will run immediately.
    /// If it's `false, the `callback` will run only after
    /// the first change is detected of any signal that is accessed in `deps`.
    /// Defaults to `false`.
    pub fn immediate(self, immediate: bool) -> Self {
        Self { immediate, ..self }
    }

    /// Allows to debounce or throttle the callback with a [`FilterOptions`]. Defaults to no filter.
    pub fn filter<NewF>(self, filter: NewF) -> WatchOptions<NewF> {
        WatchOptions {
            immediate: self.immediate,
            filter,
        }
    }

    /// Debounce the watch callback
    pub fn debounce(self, ms: impl Into<Signal<f64>>) -> WatchOptions<FilterOptions> {
        self.debounce_with_options(ms, DebounceOptions::default())
    }

    /// Debounce the watch callback
    pub fn debounce_with_options(
        self,
        ms: impl Into<Signal<f64>>,
        options: DebounceOptions,
    ) -> WatchOptions<FilterOptions> {
        self.filter(FilterOptions::Debounce {
            ms: ms.into(),
            options,
        })
    }

    /// Throttle the watch callback
    pub fn throttle(self, ms: impl Into<Signal<f64>>) -> WatchOptions<FilterOptions> {
        self.throttle_with_options(ms, ThrottleOptions::default())
    }

    /// Throttle the watch callback
    pub fn throttle_with_options(
        self,
        ms: impl Into<Signal<f64>>,
        options: ThrottleOptions,
    ) -> WatchOptions<FilterOptions> {
        self.filter(FilterOptions::Throttle {
            ms: ms.into(),
            options,
        })
    }
}
