use crate::{DebounceOptions, WatchDebouncedOptions, WatchOptions, watch_with_options};

/// A debounced version of [`watch`](leptos::prelude::watch).
///
/// ## Demo
///
/// [Link to Demo](https://github.com/Synphonyte/leptos-use/tree/main/examples/watch_debounced)
///
/// ## Usage
///
/// ```
/// # use leptos::prelude::*;
/// # use leptos::logging::log;
/// # use leptos_use_watch_debounce_throttle::watch_debounced;
/// #
/// # pub fn Demo() -> impl IntoView {
/// #     let (source, set_source) = signal(0);
/// #
/// watch_debounced(
///     move || source.get(),
///     move |_, _, _| {
///         log!("changed!");
///     },
///     500.0,
/// );
///
/// #    view! { }
/// # }
/// ```
///
/// This really is only shorthand shorthand for `watch_with_options(deps, callback, WatchOptions::default().debounce(ms))`.
///
/// Please note that if the current component is cleaned up before the debounced callback is called, the debounced callback will not be called.
///
/// There's also [`watch_debounced_with_options`] where you can specify the other watch options (except `filter`).
///
/// ```
/// # use leptos::prelude::*;
/// # use leptos::logging::log;
/// # use leptos_use_watch_debounce_throttle::{watch_debounced_with_options, WatchDebouncedOptions};
/// #
/// # pub fn Demo() -> impl IntoView {
/// #     let (source, set_source) = signal(0);
/// #
/// watch_debounced_with_options(
///     move || source.get(),
///     move |_, _, _| {
///         log!("changed!");
///     },
///     500.0,
///     WatchDebouncedOptions::default().max_wait(Some(1000.0)),
/// );
///
/// #    view! { }
/// # }
/// ```
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
/// On the server the callback
/// will never be called except if you set `immediate` to `true` in which case the callback will be
/// called exactly once.
///
/// ## See also
///
/// * `leptos::watch`
/// * [`fn@crate::watch_throttled`]
pub fn watch_debounced<W, T, DFn, CFn>(
    deps: DFn,
    callback: CFn,
    ms: f64,
) -> impl Fn() + Clone + Send + Sync
where
    DFn: Fn() -> W + 'static,
    CFn: Fn(&W, Option<&W>, Option<T>) -> T + Clone + 'static,
    W: Clone + 'static,
    T: Clone + 'static,
{
    watch_with_options(deps, callback, WatchOptions::default().debounce(ms))
}

/// Version of `watch_debounced` that accepts `WatchDebouncedOptions`.
/// See [`watch_debounced`] for how to use.
pub fn watch_debounced_with_options<W, T, DFn, CFn>(
    deps: DFn,
    callback: CFn,
    ms: f64,
    options: WatchDebouncedOptions,
) -> impl Fn() + Clone + Send + Sync
where
    DFn: Fn() -> W + 'static,
    CFn: Fn(&W, Option<&W>, Option<T>) -> T + Clone + 'static,
    W: Clone + 'static,
    T: Clone + 'static,
{
    watch_with_options(
        deps,
        callback,
        WatchOptions::default()
            .debounce_with_options(ms, DebounceOptions::default().max_wait(options.max_wait))
            .immediate(options.immediate),
    )
}
