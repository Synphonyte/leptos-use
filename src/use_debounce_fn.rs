use crate::utils::{
    create_filter_wrapper, create_filter_wrapper_with_arg, debounce_filter, DebounceOptions,
};
use leptos::MaybeSignal;

pub fn use_debounce_fn<F>(func: F, ms: impl Into<MaybeSignal<f64>>) -> impl FnMut()
where
    F: FnOnce() + Clone + 'static,
{
    use_debounce_fn_with_options(func, ms, Default::default())
}

pub fn use_debounce_fn_with_options<F>(
    func: F,
    ms: impl Into<MaybeSignal<f64>>,
    options: DebounceOptions,
) -> impl FnMut()
where
    F: FnOnce() + Clone + 'static,
{
    create_filter_wrapper(debounce_filter(ms, options), func)
}

pub fn use_debounce_fn_with_arg<F, Arg>(func: F, ms: impl Into<MaybeSignal<f64>>) -> impl FnMut(Arg)
where
    F: FnOnce(Arg) + Clone + 'static,
    Arg: Clone + 'static,
{
    use_debounce_fn_with_arg_and_options(func, ms, Default::default())
}

pub fn use_debounce_fn_with_arg_and_options<F, Arg>(
    func: F,
    ms: impl Into<MaybeSignal<f64>>,
    options: DebounceOptions,
) -> impl FnMut(Arg)
where
    F: FnOnce(Arg) + Clone + 'static,
    Arg: Clone + 'static,
{
    create_filter_wrapper_with_arg(debounce_filter(ms, options), func)
}
