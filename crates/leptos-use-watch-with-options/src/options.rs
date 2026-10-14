use crate::{DebounceOptions, FilterOptions, ThrottleOptions, filter_builder_methods};
use default_struct_builder::DefaultBuilder;
use leptos::prelude::*;

/// Options for `watch_with_options`
#[derive(DefaultBuilder, Default)]
pub struct WatchOptions {
    /// If `immediate` is true, the `callback` will run immediately.
    /// If it's `false, the `callback` will run only after
    /// the first change is detected of any signal that is accessed in `deps`.
    /// Defaults to `false`.
    pub(crate) immediate: bool,

    /// Allows to debounce or throttle the callback. Defaults to no filter.
    pub(crate) filter: FilterOptions,
}

impl WatchOptions {
    filter_builder_methods!(
        /// the watch callback
        filter
    );
}
