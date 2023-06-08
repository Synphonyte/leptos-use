mod debounce;
mod pausable;
mod throttle;

pub use debounce::*;
pub use pausable::*;
pub use throttle::*;

use crate::utils::{CloneableFnWithArgAndReturn, CloneableFnWithReturn};
use leptos::MaybeSignal;
use std::cell::RefCell;
use std::rc::Rc;

pub trait FilterFn<R>:
    CloneableFnWithArgAndReturn<Box<dyn CloneableFnWithReturn<R>>, Rc<RefCell<Option<R>>>>
{
}

impl<R, F> FilterFn<R> for F
where
    F: Fn(Box<dyn CloneableFnWithReturn<R>>) -> Rc<RefCell<Option<R>>> + Clone + 'static,
    R: 'static,
{
}

impl<R> Clone for Box<dyn FilterFn<R>> {
    fn clone(&self) -> Self {
        (*self).clone()
    }
}

pub fn create_filter_wrapper<F, R>(
    filter: Box<dyn FilterFn<R>>,
    func: F,
) -> impl Fn() -> Rc<RefCell<Option<R>>> + Clone
where
    F: FnOnce() -> R + Clone + 'static,
    R: 'static,
{
    move || filter.clone()(Box::new(func.clone()))
}

pub fn create_filter_wrapper_with_arg<F, Arg, R>(
    filter: Box<dyn FilterFn<R>>,
    func: F,
) -> impl Fn(Arg) -> Rc<RefCell<Option<R>>> + Clone
where
    F: FnOnce(Arg) -> R + Clone + 'static,
    R: 'static,
    Arg: Clone + 'static,
{
    move |arg: Arg| {
        let func = func.clone();
        filter.clone()(Box::new(move || func(arg)))
    }
}

#[derive(Default)]
pub enum FilterOptions {
    #[default]
    None,
    Debounce {
        ms: MaybeSignal<f64>,
        options: DebounceOptions,
    },
    Throttle {
        ms: MaybeSignal<f64>,
        options: ThrottleOptions,
    },
}

impl FilterOptions {
    pub fn filter_fn<R>(&self) -> Box<dyn FilterFn<R>>
    where
        R: 'static,
    {
        match self {
            FilterOptions::Debounce { ms, options } => {
                Box::new(debounce_filter(ms.clone(), *options))
            }
            FilterOptions::Throttle { ms, options } => {
                Box::new(throttle_filter(ms.clone(), *options))
            }
            FilterOptions::None => Box::new(|invoke: Box<dyn CloneableFnWithReturn<R>>| {
                Rc::new(RefCell::new(Some(invoke())))
            }),
        }
    }
}

#[macro_export]
macro_rules! filter_builder_methods {
    (
        #[$filter_docs:meta]
        $filter_field_name:ident
    ) => {
        /// Debounce
        #[$filter_docs]
        /// by `ms` milliseconds.
        pub fn debounce(self, ms: impl Into<MaybeSignal<f64>>) -> Self {
            self.debounce_with_options(ms, DebounceOptions::default())
        }

        /// Debounce
        #[$filter_docs]
        /// by `ms` milliseconds with additional options.
        pub fn debounce_with_options(
            self,
            ms: impl Into<MaybeSignal<f64>>,
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
        pub fn throttle(self, ms: impl Into<MaybeSignal<f64>>) -> Self {
            self.throttle_with_options(ms, ThrottleOptions::default())
        }

        /// Throttle
        #[$filter_docs]
        /// by `ms` milliseconds with additional options.
        pub fn throttle_with_options(
            self,
            ms: impl Into<MaybeSignal<f64>>,
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
