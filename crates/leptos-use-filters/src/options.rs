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
