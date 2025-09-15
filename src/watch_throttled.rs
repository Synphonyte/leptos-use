use crate::{utils::ThrottleOptions, watch_with_options, WatchOptions};
use default_struct_builder::DefaultBuilder;

/// A throttled version of `leptos::watch`.
///
/// ## Demo
///
/// [Link to Demo](https://github.com/Synphonyte/leptos-use/tree/main/examples/watch_throttled)
///
/// ## Usage
///
/// ```
/// # use leptos::prelude::*;
/// # use leptos::logging::log;
/// # use leptos_use::watch_throttled;
/// #
/// # pub fn Demo() -> impl IntoView {
/// #     let (source, set_source) = signal(0);
/// #
/// watch_throttled(
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
/// This really is only shorthand shorthand for `watch_with_options(deps, callback, WatchOptions::default().throttle(ms))`.
///
/// Please note that if the current component is cleaned up before the throttled callback is called, the throttled callback will not be called.
///
/// There's also `watch_throttled_with_options` where you can specify the other watch options (except `filter`).
///
/// ```
/// # use leptos::prelude::*;
/// # use leptos::logging::log;
/// # use leptos_use::{watch_throttled_with_options, WatchThrottledOptions};
/// #
/// # pub fn Demo() -> impl IntoView {
/// #     let (source, set_source) = signal(0);
/// #
/// watch_throttled_with_options(
///     move || source.get(),
///     move |_, _, _| {
///         log!("changed!");
///     },
///     500.0,
///     WatchThrottledOptions::default().leading(true).trailing(false),
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
/// * [`fn@crate::watch_debounced`]
pub fn watch_throttled<W, T, DFn, CFn>(
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
    watch_with_options(deps, callback, WatchOptions::default().throttle(ms))
}

/// Version of [`fn@watch_throttled`] that accepts `WatchThrottledOptions`. See [`watch_throttled`] for how to use.
pub fn watch_throttled_with_options<W, T, DFn, CFn>(
    deps: DFn,
    callback: CFn,
    ms: f64,
    options: WatchThrottledOptions,
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
            .throttle_with_options(
                ms,
                ThrottleOptions::default()
                    .leading(options.leading)
                    .trailing(options.trailing),
            )
            .immediate(options.immediate),
    )
}

/// Options for [`watch_throttled_with_options`].
#[derive(DefaultBuilder)]
pub struct WatchThrottledOptions {
    /// If `immediate` is false, the `callback` will not run immediately but only after
    /// the first change is detected of any signal that is accessed in `deps`.
    /// Defaults to `true`.
    immediate: bool,

    /// Invoke on the trailing edge of the timeout. Defaults to `true`.
    pub trailing: bool,
    /// Invoke on the leading edge of the timeout. Defaults to `true`.
    pub leading: bool,
}

impl Default for WatchThrottledOptions {
    fn default() -> Self {
        Self {
            immediate: false,
            trailing: true,
            leading: true,
        }
    }
}
