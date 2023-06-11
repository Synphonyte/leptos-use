use crate::utils::{create_filter_wrapper, FilterOptions};
use crate::{filter_builder_methods, DebounceOptions, ThrottleOptions};
use default_struct_builder::DefaultBuilder;
use leptos::*;
use std::cell::RefCell;
use std::rc::Rc;

/// A version of `create_effect` that listens to any dependency that is accessed inside `deps`.
/// Also a stop handler is returned.
/// The return value of `deps` is passed into `callback` as an argument together with the previous value
/// and the previous value that the `callback` itself returned last time.
///
/// ## Usage
///
/// ```
/// # use std::time::Duration;
/// # use leptos::*;
/// # use leptos_use::watch;
/// #
/// # pub fn Demo(cx: Scope) -> impl IntoView {
/// let (num, set_num) = create_signal(cx, 0);
///
/// let stop = watch(
///     cx,
///     num,
///     move |num, _, _| {
///         log!("Number {}", num);
///     },
/// );
///
/// set_num(1); // > "Number 1"
///
/// set_timeout_with_handle(move || {
///     stop(); // stop watching
///
///     set_num(2); // (nothing happens)
/// }, Duration::from_millis(1000));
/// #    view! { cx, }
/// # }
/// ```
///
/// ## Immediate
///
/// If `immediate` is true, the `callback` will run immediately.
/// If it's `false, the `callback` will run only after
/// the first change is detected of any signal that is accessed in `deps`.
///
/// ```
/// # use leptos::*;
/// # use leptos_use::{watch_with_options, WatchOptions};
/// #
/// # pub fn Demo(cx: Scope) -> impl IntoView {
/// let (num, set_num) = create_signal(cx, 0);
///
/// watch_with_options(
///     cx,
///     num,
///     move |num, _, _| {
///         log!("Number {}", num);
///     },
///     WatchOptions::default().immediate(true),
/// ); // > "Number 0"
///
/// set_num(1); // > "Number 1"
/// #    view! { cx, }
/// # }
/// ```
///
/// ## Filters
///
/// The callback can be throttled or debounced. Please see [`watch_throttled`] and [`watch_debounced`] for details.
///
/// ```
/// # use leptos::*;
/// # use leptos_use::{watch_with_options, WatchOptions};
/// #
/// # pub fn Demo(cx: Scope) -> impl IntoView {
/// # let (num, set_num) = create_signal(cx, 0);
/// #
/// watch_with_options(
///     cx,
///     num,
///     move |num, _, _| {
///         log!("Number {}", num);
///     },
///     WatchOptions::default().throttle(100.0), // there's also `throttle_with_options`
/// );
/// #    view! { cx, }
/// # }
/// ```
///
/// ```
/// # use leptos::*;
/// # use leptos_use::{watch_with_options, WatchOptions};
/// #
/// # pub fn Demo(cx: Scope) -> impl IntoView {
/// # let (num, set_num) = create_signal(cx, 0);
/// #
/// watch_with_options(
///     cx,
///     num,
///     move |num, _, _| {
///         log!("number {}", num);
///     },
///     WatchOptions::default().debounce(100.0), // there's also `debounce_with_options`
/// );
/// #    view! { cx, }
/// # }
/// ```
///
/// ## See also
///
/// * [`watch_throttled`]
/// * [`watch_debounced`]
pub fn watch<W, T, DFn, CFn>(cx: Scope, deps: DFn, callback: CFn) -> impl Fn() + Clone
where
    DFn: Fn() -> W + 'static,
    CFn: Fn(&W, Option<&W>, Option<T>) -> T + Clone + 'static,
    W: Clone + 'static,
    T: Clone + 'static,
{
    watch_with_options(cx, deps, callback, WatchOptions::default())
}

/// Version of `watch` that accepts `WatchOptions`. See [`watch`] for how to use.
pub fn watch_with_options<W, T, DFn, CFn>(
    cx: Scope,
    deps: DFn,
    callback: CFn,
    options: WatchOptions,
) -> impl Fn() + Clone
where
    DFn: Fn() -> W + 'static,
    CFn: Fn(&W, Option<&W>, Option<T>) -> T + Clone + 'static,
    W: Clone + 'static,
    T: Clone + 'static,
{
    let (is_active, set_active) = create_signal(cx, true);

    let cur_deps_value: Rc<RefCell<Option<W>>> = Rc::new(RefCell::new(None));
    let prev_deps_value: Rc<RefCell<Option<W>>> = Rc::new(RefCell::new(None));
    let prev_callback_value: Rc<RefCell<Option<T>>> = Rc::new(RefCell::new(None));

    let cur_val = Rc::clone(&cur_deps_value);
    let prev_val = Rc::clone(&prev_deps_value);
    let prev_cb_val = Rc::clone(&prev_callback_value);
    let wrapped_callback = move || {
        callback(
            cur_val
                .borrow()
                .as_ref()
                .expect("this will not be called before there is deps value"),
            prev_val.borrow().as_ref(),
            prev_cb_val.take(),
        )
    };

    let filtered_callback =
        create_filter_wrapper(options.filter.filter_fn(), wrapped_callback.clone());

    create_effect(cx, move |did_run_before| {
        if !is_active() {
            return;
        }

        let deps_value = deps();

        if !options.immediate && did_run_before.is_none() {
            prev_deps_value.replace(Some(deps_value));
            return;
        }

        cur_deps_value.replace(Some(deps_value.clone()));

        let callback_value = if options.immediate && did_run_before.is_none() {
            Some(wrapped_callback())
        } else {
            filtered_callback().take()
        };

        prev_callback_value.replace(callback_value);

        prev_deps_value.replace(Some(deps_value));
    });

    move || {
        set_active(false);
    }
}

/// Options for `watch_with_options`
#[derive(DefaultBuilder, Default)]
pub struct WatchOptions {
    /// If `immediate` is true, the `callback` will run immediately.
    /// If it's `false, the `callback` will run only after
    /// the first change is detected of any signal that is accessed in `deps`.
    immediate: bool,

    /// Allows to debounce or throttle the callback
    filter: FilterOptions,
}

impl WatchOptions {
    filter_builder_methods!(
        /// the watch callback
        filter
    );
}
