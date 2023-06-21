use crate::{watch_with_options, WatchOptions};
use leptos::*;

/// Shorthand for watching a signal to be `true`.
///
/// ## Usage
///
/// ```
/// # use leptos::*;
/// # use leptos_use::whenever;
/// #
/// # pub fn Demo(cx: Scope) -> impl IntoView {
/// let (is_ready, set_ready) = create_signal(cx, false);
///
/// whenever(cx, move || is_ready.get(), |v, _, _| log!("{}", v));
/// #
/// #     view! { cx, }
/// # }
/// ```
///
/// ### Callback Function
///
/// Same as [`watch`], the callback will be called with `callback(input, prev_input, prev_return)`.
///
/// ```
/// # use leptos::*;
/// # use leptos_use::whenever;
/// #
/// # pub fn Demo(cx: Scope) -> impl IntoView {
/// # let (is_ready, set_ready) = create_signal(cx, false);
/// whenever(cx, move || is_ready.get(), |value, prev_value, _| {
///     log!("before: {prev_value:?}; now: {value}");
/// });
/// #
/// #     view! { cx, }
/// # }
/// ```
///
/// ### Computed
///
/// Same as [`watch`], you can pass a getter function to calculate on each change.
///
/// ```
/// # use leptos::*;
/// # use leptos_use::whenever;
/// #
/// # pub fn Demo(cx: Scope) -> impl IntoView {
/// # let (counter, set_counter) = create_signal(cx, 0);
/// whenever(
///     cx,
///     move || counter.get() == 7,
///     |_, _, _| log!("counter is 7 now!"),
/// );
/// #
/// #     view! { cx, }
/// # }
/// ```
///
/// ### Options
///
/// Options and defaults are same as [`watch_with_options`].
///
/// ```
/// # use leptos::*;
/// # use leptos_use::{WatchOptions, whenever_with_options};
/// #
/// # pub fn Demo(cx: Scope) -> impl IntoView {
/// # let (counter, set_counter) = create_signal(cx, 0);
/// whenever_with_options(
///     cx,
///     move || counter.get() == 7,
///     |_, _, _| log!("counter is 7 now!"),
///     WatchOptions::default().immediate(true),
/// );
/// #
/// #     view! { cx, }
/// # }
/// ```
pub fn whenever<T, DFn, CFn>(cx: Scope, source: DFn, callback: CFn) -> impl Fn() + Clone
where
    DFn: Fn() -> bool + 'static,
    CFn: Fn(bool, Option<bool>, Option<T>) -> T + Clone + 'static,
    T: Clone + 'static,
{
    whenever_with_options(cx, source, callback, WatchOptions::default())
}

/// Version of `whenever` that accepts `WatchOptions`. See [`whenever`] for how to use.
pub fn whenever_with_options<T, DFn, CFn>(
    cx: Scope,
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
        cx,
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
