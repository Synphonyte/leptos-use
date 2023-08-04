mod debounce;
mod throttle;

pub use debounce::*;
pub use throttle::*;

use leptos::MaybeSignal;
use std::cell::RefCell;
use std::rc::Rc;

macro_rules! RcFilterFn {
    ($R:ident) => {
        Rc<dyn Fn(Rc<dyn Fn() -> $R>) -> Rc<RefCell<Option<$R>>>>
    }
}

pub fn create_filter_wrapper<F, R>(
    filter: RcFilterFn!(R),
    func: F,
) -> impl Fn() -> Rc<RefCell<Option<R>>> + Clone
where
    F: Fn() -> R + Clone + 'static,
    R: 'static,
{
    move || Rc::clone(&filter)(Rc::new(func.clone()))
}

pub fn create_filter_wrapper_with_arg<F, Arg, R>(
    filter: RcFilterFn!(R),
    func: F,
) -> impl Fn(Arg) -> Rc<RefCell<Option<R>>> + Clone
where
    F: Fn(Arg) -> R + Clone + 'static,
    R: 'static,
    Arg: Clone + 'static,
{
    move |arg: Arg| {
        let func = func.clone();
        Rc::clone(&filter)(Rc::new(move || func(arg.clone())))
    }
}

/// Specify a debounce or throttle filter with their respective options or no filter
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
    pub fn filter_fn<R>(&self) -> RcFilterFn!(R)
    where
        R: 'static,
    {
        match self {
            FilterOptions::Debounce { ms, options } => Rc::new(debounce_filter(*ms, *options)),
            FilterOptions::Throttle { ms, options } => Rc::new(throttle_filter(*ms, *options)),
            FilterOptions::None => {
                Rc::new(|invoke: Rc<dyn Fn() -> R>| Rc::new(RefCell::new(Some(invoke()))))
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
