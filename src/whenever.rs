use crate::{watch_with_options, WatchOptions};

/// Shorthand for watching a signal to be `true`.
///
/// ## Usage
///
/// ```
/// # use leptos::prelude::*;
/// # use leptos::logging::log;
/// # use leptos_use::whenever;
/// #
/// # pub fn Demo() -> impl IntoView {
/// let (is_ready, set_ready) = signal(false);
///
/// whenever(move || is_ready.get(), |v, _, _| log!("{}", v));
/// #
/// #     view! { }
/// # }
/// ```
///
/// ### Callback Function
///
/// Same as [`watch`], the callback will be called with `callback(input, prev_input, prev_return)`.
///
/// ```
/// # use leptos::prelude::*;
/// # use leptos::logging::log;
/// # use leptos_use::whenever;
/// #
/// # pub fn Demo() -> impl IntoView {
/// # let (is_ready, set_ready) = signal(false);
/// whenever(move || is_ready.get(), |value, prev_value, _| {
///     log!("before: {prev_value:?}; now: {value}");
/// });
/// #
/// #     view! { }
/// # }
/// ```
///
/// ### Computed
///
/// Same as [`watch`], you can pass a getter function to calculate on each change.
///
/// ```
/// # use leptos::prelude::*;
/// # use leptos::logging::log;
/// # use leptos_use::whenever;
/// #
/// # pub fn Demo() -> impl IntoView {
/// # let (counter, set_counter) = signal(0);
/// whenever(
///     move || counter.get() == 7,
///     |_, _, _| log!("counter is 7 now!"),
/// );
/// #
/// #     view! { }
/// # }
/// ```
///
/// ### Options
///
/// Options and defaults are same as [`watch_with_options`].
///
/// ```
/// # use leptos::prelude::*;
/// # use leptos::logging::log;
/// # use leptos_use::{WatchOptions, whenever_with_options};
/// #
/// # pub fn Demo() -> impl IntoView {
/// # let (counter, set_counter) = signal(0);
/// whenever_with_options(
///     move || counter.get() == 7,
///     |_, _, _| log!("counter is 7 now!"),
///     WatchOptions::default().immediate(true),
/// );
/// #
/// #     view! { }
/// # }
/// ```
///
/// ## Server-Side Rendering
///
/// On the server this works just fine except if you throttle or debounce in which case the callback
/// will never be called except if you set `immediate` to `true` in which case the callback will be
/// called exactly once.
pub fn whenever<T, DFn, CFn>(source: DFn, callback: CFn) -> impl Fn() + Clone
where
    DFn: Fn() -> bool + 'static,
    CFn: Fn(bool, Option<bool>, Option<T>) -> T + Clone + 'static,
    T: Clone + 'static,
{
    whenever_with_options(source, callback, WatchOptions::default())
}

/// Version of `whenever` that accepts `WatchOptions`. See [`whenever`] for how to use.
pub fn whenever_with_options<T, DFn, CFn>(
    source: DFn,
    callback: CFn,
    options: WatchOptions,
) -> impl Fn() + Clone
where
    DFn: Fn() -> bool + 'static,
    CFn: Fn(bool, Option<bool>, Option<T>) -> T + Clone + 'static,
    T: Clone + 'static,
{
    watch_with_options(
        source,
        move |value, prev_value, prev_return| {
            if *value {
                Some(callback(
                    *value,
                    prev_value.copied(),
                    prev_return.unwrap_or_default(),
                ))
            } else {
                None
            }
        },
        options,
    )
}
