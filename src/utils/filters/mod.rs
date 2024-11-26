mod debounce;
mod throttle;

pub use debounce::*;
pub use throttle::*;

use crate::sendwrap_fn;
use leptos::prelude::Signal;
use std::sync::{Arc, Mutex};

macro_rules! ArcFilterFn {
    ($R:ident) => {
        Arc<dyn Fn(Arc<dyn Fn() -> $R>) -> Arc<Mutex<Option<$R>>>>
    }
}

pub fn create_filter_wrapper<F, R>(
    filter: ArcFilterFn!(R),
    func: F,
) -> impl Fn() -> Arc<Mutex<Option<R>>> + Clone + Send + Sync
where
    F: Fn() -> R + Clone + 'static,
    R: 'static,
{
    sendwrap_fn!(move || Arc::clone(&filter)(Arc::new(func.clone())))
}

pub fn create_filter_wrapper_with_arg<F, Arg, R>(
    filter: ArcFilterFn!(R),
    func: F,
) -> impl Fn(Arg) -> Arc<Mutex<Option<R>>> + Clone + Send + Sync
where
    F: Fn(Arg) -> R + Clone + 'static,
    R: 'static,
    Arg: Clone + 'static,
{
    sendwrap_fn!(move |arg: Arg| {
        let func = func.clone();
        Arc::clone(&filter)(Arc::new(move || func(arg.clone())))
    })
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
