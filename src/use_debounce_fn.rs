use crate::utils::{
    create_filter_wrapper, create_filter_wrapper_with_arg, debounce_filter, DebounceOptions,
};
use leptos::MaybeSignal;

/// Debounce execution of a function.
///
/// > Debounce is an overloaded waiter: If you keep asking him your requests will be ignored until you stop and give him some time to think about your latest inquiry.
///
/// ## Demo
///
/// [Link to Demo](https://github.com/Synphonyte/leptos-use/tree/master/examples/use_debounce_fn)
///
/// ## Usage
///
/// ```
/// # use leptos::*;
/// # use leptos::ev::resize;
/// # use leptos_use::use_debounce_fn;
/// #
/// # #[component]
/// # fn Demo(cx: Scope) -> impl IntoView {
/// let mut debounced_fn = use_debounce_fn(
///     || {
///         // do something
///     },
///     1000.0,
/// );
///
/// window_event_listener(resize, move |_| debounced_fn());
/// #    view! { cx, }
/// # }
/// ```
///
/// You can also pass options to [`use_debounce_fn_with_options`] with a maximum wait time, similar to
/// [lodash debounce](https://lodash.com/docs/#debounce).
///
/// ```
/// # use leptos::*;
/// # use leptos::ev::resize;
/// # use leptos_use::use_debounce_fn_with_options;
/// # use leptos_use::utils::DebounceOptions;
/// #
/// # #[component]
/// # fn Demo(cx: Scope) -> impl IntoView {
/// let mut debounced_fn = use_debounce_fn_with_options(
///     || {
///         // do something
///     },
///     1000.0,
///     DebounceOptions {
///         max_wait: Some(5000.0),
///     }
/// );
///
/// window_event_listener(resize, move |_| debounced_fn());
/// #    view! { cx, }
/// # }
/// ```
///
/// Currently there is no way to use a function with a return value. Please open an issue if you need this.
///
/// If you want to throttle a function that takes an argument there are also the versions
/// [`use_debounce_fn_with_args`] and [`use_debounce_fn_with_args_and_options`].
///
/// ## Recommended Reading
///
/// - [**Debounce vs Throttle**: Definitive Visual Guide](https://redd.one/blog/debounce-vs-throttle)
pub fn use_debounce_fn<F>(func: F, ms: impl Into<MaybeSignal<f64>>) -> impl Fn()
where
    F: FnOnce() + Clone + 'static,
{
    use_debounce_fn_with_options(func, ms, DebounceOptions::<Option<f64>>::default())
}

/// Version of [`use_debounce_fn`] with debounce options. See the docs for [`use_debounce_fn`] for how to use.
pub fn use_debounce_fn_with_options<F, W>(
    func: F,
    ms: impl Into<MaybeSignal<f64>>,
    options: DebounceOptions<W>,
) -> impl Fn()
where
    F: FnOnce() + Clone + 'static,
    W: Into<MaybeSignal<Option<f64>>>,
{
    create_filter_wrapper(debounce_filter(ms, options), func)
}

/// Version of [`use_debounce_fn`] with an argument for the debounced function. See the docs for [`use_debounce_fn`] for how to use.
pub fn use_debounce_fn_with_arg<F, Arg>(func: F, ms: impl Into<MaybeSignal<f64>>) -> impl Fn(Arg)
where
    F: FnOnce(Arg) + Clone + 'static,
    Arg: Clone + 'static,
{
    use_debounce_fn_with_arg_and_options(func, ms, DebounceOptions::<Option<f64>>::default())
}

/// Version of [`use_debounce_fn_with_arg`] with debounce options. See the docs for [`use_debounce_fn`] for how to use.
pub fn use_debounce_fn_with_arg_and_options<F, Arg, W>(
    func: F,
    ms: impl Into<MaybeSignal<f64>>,
    options: DebounceOptions<W>,
) -> impl Fn(Arg)
where
    F: FnOnce(Arg) + Clone + 'static,
    Arg: Clone + 'static,
    W: Into<MaybeSignal<Option<f64>>>,
{
    create_filter_wrapper_with_arg(debounce_filter(ms, options), func)
}
