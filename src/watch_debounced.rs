use default_struct_builder::DefaultBuilder;
use leptos::*;
use leptos_use::{watch_with_options, DebounceOptions, WatchOptions};

/// A debounced version of [`watch`].
///
/// ## Demo
///
/// [Link to Demo](https://github.com/Synphonyte/leptos-use/tree/main/examples/watch_debounced)
///
/// ## Usage
///
/// ```
/// # use leptos::*;
/// # use leptos_use::watch_debounced;
/// #
/// # pub fn Demo(cx: Scope) -> impl IntoView {
/// #     let (source, set_source) = create_signal(cx, 0);
/// #
/// watch_debounced(
///     cx,
///     source,
///     move |_, _, _| {
///         log!("changed!");
///     },
///     500.0,
/// );
///
/// #    view! { cx, }
/// # }
/// ```
///
/// This really is only shorthand shorthand for `watch_with_options(cx, deps, callback, WatchOptions::default().debounce(ms))`.
///
/// There's also `watch_debounced_with_options` where you can specify the other watch options (except `filter`).
///
/// ```
/// # use leptos::*;
/// # use leptos_use::{watch_debounced_with_options, WatchDebouncedOptions};
/// #
/// # pub fn Demo(cx: Scope) -> impl IntoView {
/// #     let (source, set_source) = create_signal(cx, 0);
/// #
/// watch_debounced_with_options(
///     cx,
///     source,
///     move |_, _, _| {
///         log!("changed!");
///     },
///     500.0,
///     WatchDebouncedOptions::default().max_wait(Some(1000.0)),
/// );
///
/// #    view! { cx, }
/// # }
/// ```
///
/// ## Recommended Reading
///
/// - [**Debounce vs Throttle**: Definitive Visual Guide](https://redd.one/blog/debounce-vs-throttle)
/// - [Debouncing and Throttling Explained Through Examples](https://css-tricks.com/debouncing-throttling-explained-examples/)
///
/// ## See also
///
/// * [`watch`]
/// * [`watch_throttled`]
pub fn watch_debounced<W, T, DFn, CFn>(
    cx: Scope,
    deps: DFn,
    callback: CFn,
    ms: f64,
) -> impl Fn() + Clone
where
    DFn: Fn() -> W + 'static,
    CFn: Fn(&W, Option<&W>, Option<T>) -> T + Clone + 'static,
    W: Clone + 'static,
    T: Clone + 'static,
{
    watch_with_options(cx, deps, callback, WatchOptions::default().debounce(ms))
}

/// Version of `watch_debounced` that accepts `WatchDebouncedOptions`. See [`watch_debounced`] for how to use.
pub fn watch_debounced_with_options<W, T, DFn, CFn>(
    cx: Scope,
    deps: DFn,
    callback: CFn,
    ms: f64,
    options: WatchDebouncedOptions,
) -> impl Fn() + Clone
where
    DFn: Fn() -> W + 'static,
    CFn: Fn(&W, Option<&W>, Option<T>) -> T + Clone + 'static,
    W: Clone + 'static,
    T: Clone + 'static,
{
    watch_with_options(
        cx,
        deps,
        callback,
        WatchOptions::default()
            .debounce_with_options(ms, DebounceOptions::default().max_wait(options.max_wait))
            .immediate(options.immediate),
    )
}

/// Options for [`watch_debounced_with_options`].
#[derive(DefaultBuilder, Default)]
pub struct WatchDebouncedOptions {
    /// If `immediate` is false, the `callback` will not run immediately but only after
    /// the first change is detected of any signal that is accessed in `deps`.
    /// Defaults to `true`.
    immediate: bool,

    /// The maximum time allowed to be delayed before the callback invoked.
    /// In milliseconds.
    /// Same as [`DebounceOptions::max_wait`]
    #[builder(into)]
    pub max_wait: MaybeSignal<Option<f64>>,
}
