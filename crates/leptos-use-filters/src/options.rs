use crate::{debounce_filter, throttle_filter};
use default_struct_builder::DefaultBuilder;
use leptos::prelude::Signal;
use std::sync::{Arc, Mutex};

#[derive(Copy, Clone, DefaultBuilder, Default)]
pub struct DebounceOptions {
    /// The maximum time allowed to be delayed before it's invoked.
    /// In milliseconds.
    #[builder(into)]
    pub max_wait: Signal<Option<f64>>,
}

/// A filter that wraps the calls of a function, like a debounce or throttle filter.
///
/// Options that support filters are generic over this, so a filter's code is only compiled in when
/// it's used: [`NoFilter`] is the default and [`FilterOptions`] selects a filter at runtime.
pub trait CallFilter: 'static {
    /// Returns a function that calls the given function, filtered.
    fn filter_fn<R>(&self) -> ArcFilterFn!(R)
    where
        R: 'static;
}

/// Calls functions directly. This is the default filter of options that support filters.
#[derive(Copy, Clone, Default, Debug)]
pub struct NoFilter;

impl CallFilter for NoFilter {
    fn filter_fn<R>(&self) -> ArcFilterFn!(R)
    where
        R: 'static,
    {
        Arc::new(|invoke: Arc<dyn Fn() -> R>| Arc::new(Mutex::new(Some(invoke()))))
    }
}

impl CallFilter for FilterOptions {
    fn filter_fn<R>(&self) -> ArcFilterFn!(R)
    where
        R: 'static,
    {
        FilterOptions::filter_fn(self)
    }
}

/// Specify a debounce or throttle filter with their respective options or no filter
#[derive(Default)]
pub enum FilterOptions {
    #[default]
    None,
    Debounce {
        ms: Signal<f64>,
        options: DebounceOptions,
    },
    Throttle {
        ms: Signal<f64>,
        options: ThrottleOptions,
    },
}

impl FilterOptions {
    pub fn debounce(ms: impl Into<Signal<f64>>) -> Self {
        Self::Debounce {
            ms: ms.into(),
            options: DebounceOptions::default(),
        }
    }

    pub fn throttle(ms: impl Into<Signal<f64>>) -> Self {
        Self::Throttle {
            ms: ms.into(),
            options: ThrottleOptions::default(),
        }
    }

    pub fn filter_fn<R>(&self) -> ArcFilterFn!(R)
    where
        R: 'static,
    {
        match self {
            FilterOptions::Debounce { ms, options } => Arc::new(debounce_filter(*ms, *options)),
            FilterOptions::Throttle { ms, options } => Arc::new(throttle_filter(*ms, *options)),
            FilterOptions::None => {
                Arc::new(|invoke: Arc<dyn Fn() -> R>| Arc::new(Mutex::new(Some(invoke()))))
            }
        }
    }
}

/// Defines builder methods to define filter options without having to use nested methods
#[macro_export]
macro_rules! filter_builder_methods {
    (
        #[$filter_docs:meta]
        $filter_field_name:ident
    ) => {
        /// Debounce
        #[$filter_docs]
        /// by `ms` milliseconds.
        pub fn debounce(self, ms: impl Into<Signal<f64>>) -> Self {
            self.debounce_with_options(ms, DebounceOptions::default())
        }

        /// Debounce
        #[$filter_docs]
        /// by `ms` milliseconds with additional options.
        pub fn debounce_with_options(
            self,
            ms: impl Into<Signal<f64>>,
            options: DebounceOptions,
        ) -> Self {
            Self {
                $filter_field_name: FilterOptions::Debounce {
                    ms: ms.into(),
                    options,
                },
                ..self
            }
        }

        /// Throttle
        #[$filter_docs]
        /// by `ms` milliseconds.
        pub fn throttle(self, ms: impl Into<Signal<f64>>) -> Self {
            self.throttle_with_options(ms, ThrottleOptions::default())
        }

        /// Throttle
        #[$filter_docs]
        /// by `ms` milliseconds with additional options.
        pub fn throttle_with_options(
            self,
            ms: impl Into<Signal<f64>>,
            options: ThrottleOptions,
        ) -> Self {
            Self {
                $filter_field_name: FilterOptions::Throttle {
                    ms: ms.into(),
                    options,
                },
                ..self
            }
        }
    };
}

#[derive(Copy, Clone, DefaultBuilder)]
pub struct ThrottleOptions {
    /// Invoke on the trailing edge of the timeout. Defaults to `true`.
    pub trailing: bool,
    /// Invoke on the leading edge of the timeout (=immediately). Defaults to `true`.
    pub leading: bool,
}

impl Default for ThrottleOptions {
    fn default() -> Self {
        Self {
            trailing: true,
            leading: true,
        }
    }
}
