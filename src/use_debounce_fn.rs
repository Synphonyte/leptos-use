pub use crate::utils::DebounceOptions;
use crate::utils::{create_filter_wrapper, create_filter_wrapper_with_arg, debounce_filter};
use leptos::prelude::Signal;
use std::sync::{Arc, Mutex};

/// Debounce execution of a function.
///
/// > Debounce is an overloaded waiter: If you keep asking him your requests will be ignored until you stop and give him some time to think about your latest inquiry.
///
/// ## Demo
///
/// [Link to Demo](https://github.com/Synphonyte/leptos-use/tree/main/examples/use_debounce_fn)
///
/// ## Usage
///
/// ```
/// # use leptos::prelude::*;
/// # use leptos::ev::resize;
/// # use leptos_use::use_debounce_fn;
/// #
/// # #[component]
/// # fn Demo() -> impl IntoView {
/// let mut debounced_fn = use_debounce_fn(
///     || {
///         // do something
///     },
///     1000.0,
/// );
///
/// window_event_listener(resize, move |_| { debounced_fn(); });
/// #    view! { }
/// # }
/// ```
///
/// Please note that if the current component is cleaned up before the throttled callback is called, the throttled callback will not be called.
///
/// You can also pass options to [`use_debounce_fn_with_options`] with a maximum wait time, similar to
/// [lodash debounce](https://lodash.com/docs/#debounce).
///
/// ```
/// # use leptos::prelude::*;
/// # use leptos::ev::resize;
/// # use leptos_use::use_debounce_fn_with_options;
/// # use leptos_use::utils::DebounceOptions;
/// #
/// # #[component]
/// # fn Demo() -> impl IntoView {
/// let mut debounced_fn = use_debounce_fn_with_options(
///     || {
///         // do something
///     },
///     1000.0,
///     DebounceOptions::default()
///         .max_wait(Some(5000.0)),
/// );
///
/// window_event_listener(resize, move |_| { debounced_fn(); });
/// #    view! { }
/// # }
/// ```
///
/// Currently there is no way to use a function with a return value. Please open an issue if you need this.
///
/// If you want to throttle a function that takes an argument there are also the versions
/// [`use_debounce_fn_with_arg`] and [`use_debounce_fn_with_arg_and_options`].
///
/// ## SendWrapped Return
///
/// The returned closure is a sendwrapped function. It can
/// only be called from the same thread that called `use_debounce_...`.
///
/// ## Recommended Reading
///
/// - [**Debounce vs Throttle**: Definitive Visual Guide](https://redd.one/blog/debounce-vs-throttle)
/// - [Debouncing and Throttling Explained Through Examples](https://css-tricks.com/debouncing-throttling-explained-examples/)
///
/// ## Server-Side Rendering
///
/// > Make sure you follow the [instructions in Server-Side Rendering](https://leptos-use.rs/server_side_rendering.html).
///
/// Internally this uses `setTimeout` which is not supported on the server. So usually calling
/// a debounced function on the server will simply be ignored.
pub fn use_debounce_fn<F, R>(
    func: F,
    ms: impl Into<Signal<f64>> + 'static,
) -> impl Fn() -> Arc<Mutex<Option<R>>> + Clone + Send + Sync
where
    F: Fn() -> R + Clone + 'static,
    R: 'static,
{
    use_debounce_fn_with_options(func, ms, DebounceOptions::default())
}

/// Version of [`use_debounce_fn`] with debounce options. See the docs for [`use_debounce_fn`] for how to use.
pub fn use_debounce_fn_with_options<F, R>(
    func: F,
    ms: impl Into<Signal<f64>> + 'static,
    options: DebounceOptions,
) -> impl Fn() -> Arc<Mutex<Option<R>>> + Clone + Send + Sync
where
    F: Fn() -> R + Clone + 'static,
    R: 'static,
{
    create_filter_wrapper(Arc::new(debounce_filter(ms, options)), func)
}

/// Version of [`use_debounce_fn`] with an argument for the debounced function. See the docs for [`use_debounce_fn`] for how to use.
pub fn use_debounce_fn_with_arg<F, Arg, R>(
    func: F,
    ms: impl Into<Signal<f64>> + 'static,
) -> impl Fn(Arg) -> Arc<Mutex<Option<R>>> + Clone + Send + Sync
where
    F: Fn(Arg) -> R + Clone + 'static,
    Arg: Clone + 'static,
    R: 'static,
{
    use_debounce_fn_with_arg_and_options(func, ms, DebounceOptions::default())
}

/// Version of [`use_debounce_fn_with_arg`] with debounce options.
pub fn use_debounce_fn_with_arg_and_options<F, Arg, R>(
    func: F,
    ms: impl Into<Signal<f64>> + 'static,
    options: DebounceOptions,
) -> impl Fn(Arg) -> Arc<Mutex<Option<R>>> + Clone + Send + Sync
where
    F: Fn(Arg) -> R + Clone + 'static,
    Arg: Clone + 'static,
    R: 'static,
{
    create_filter_wrapper_with_arg(Arc::new(debounce_filter(ms, options)), func)
}
