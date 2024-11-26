use crate::filter_builder_methods;
use crate::utils::{create_filter_wrapper, DebounceOptions, FilterOptions, ThrottleOptions};
use default_struct_builder::DefaultBuilder;
use leptos::prelude::*;
use std::cell::RefCell;
use std::rc::Rc;

/// A version of `leptos::watch` but with additional options.
///
/// ## Immediate
///
/// This is the same as for `leptos::watch`. But you don't have to specify it.
/// By default its set to `false`.
/// If `immediate` is `true`, the `callback` will run immediately (this is also true if throttled/debounced).
/// If it's `false`, the `callback` will run only after
/// the first change is detected of any signal that is accessed in `deps`.
///
/// ```
/// # use leptos::prelude::*;
/// # use leptos::logging::log;
/// # use leptos_use::{watch_with_options, WatchOptions};
/// #
/// # pub fn Demo() -> impl IntoView {
/// let (num, set_num) = signal(0);
///
/// watch_with_options(
///     move || num.get(),
///     move |num, _, _| {
///         log!("Number {}", num);
///     },
///     WatchOptions::default().immediate(true),
/// ); // > "Number 0"
///
/// set_num.set(1); // > "Number 1"
/// #    view! { }
/// # }
/// ```
///
/// ## Filters
///
/// The callback can be throttled or debounced. Please see [`fn@crate::watch_throttled`]
/// and [`fn@crate::watch_debounced`] for details.
///
/// ```
/// # use leptos::prelude::*;
/// # use leptos::logging::log;
/// # use leptos_use::{watch_with_options, WatchOptions};
/// #
/// # pub fn Demo() -> impl IntoView {
/// # let (num, set_num) = signal(0);
/// #
/// watch_with_options(
///     move || num.get(),
///     move |num, _, _| {
///         log!("Number {}", num);
///     },
///     WatchOptions::default().throttle(100.0), // there's also `throttle_with_options`
/// );
/// #    view! { }
/// # }
/// ```
///
/// ```
/// # use leptos::prelude::*;
/// # use leptos::logging::log;
/// # use leptos_use::{watch_with_options, WatchOptions};
/// #
/// # pub fn Demo() -> impl IntoView {
/// # let (num, set_num) = signal(0);
/// #
/// watch_with_options(
///     move || num.get(),
///     move |num, _, _| {
///         log!("number {}", num);
///     },
///     WatchOptions::default().debounce(100.0), // there's also `debounce_with_options`
/// );
/// #    view! { }
/// # }
/// ```
///
/// ## Server-Side Rendering
///
/// On the server this works just fine except if you throttle or debounce in which case the callback
/// will never be called except if you set `immediate` to `true` in which case the callback will be
/// called exactly once when `watch()` is executed.
///
/// ## See also
///
/// * [`fn@crate::watch_throttled`]
/// * [`fn@crate::watch_debounced`]
pub fn watch_with_options<W, T, DFn, CFn>(
    deps: DFn,
    callback: CFn,
    options: WatchOptions,
) -> impl Fn() + Clone + Send + Sync
where
    DFn: Fn() -> W + 'static,
    CFn: Fn(&W, Option<&W>, Option<T>) -> T + Clone + 'static,
    W: Clone + 'static,
    T: Clone + 'static,
{
    let cur_deps_value: Rc<RefCell<Option<W>>> = Rc::new(RefCell::new(None));
    let prev_deps_value: Rc<RefCell<Option<W>>> = Rc::new(RefCell::new(None));
    let prev_callback_value: Rc<RefCell<Option<T>>> = Rc::new(RefCell::new(None));

    let wrapped_callback = {
        let cur_deps_value = Rc::clone(&cur_deps_value);
        let prev_deps_value = Rc::clone(&prev_deps_value);
        let prev_callback_val = Rc::clone(&prev_callback_value);

        move || {
            #[cfg(debug_assertions)]
            let _z = leptos::reactive::diagnostics::SpecialNonReactiveZone::enter();

            let ret = callback(
                cur_deps_value
                    .borrow()
                    .as_ref()
                    .expect("this will not be called before there is deps value"),
                prev_deps_value.borrow().as_ref(),
                prev_callback_val.take(),
            );

            ret
        }
    };

    let filtered_callback =
        create_filter_wrapper(options.filter.filter_fn(), wrapped_callback.clone());

    let effect = Effect::watch(
        deps,
        move |deps_value, previous_deps_value, did_run_before| {
            cur_deps_value.replace(Some(deps_value.clone()));
            prev_deps_value.replace(previous_deps_value.cloned());

            let callback_value = if options.immediate && did_run_before.is_none() {
                Some(wrapped_callback())
            } else {
                filtered_callback().lock().unwrap().take()
            };

            prev_callback_value.replace(callback_value);
        },
        options.immediate,
    );

    move || effect.stop()

    // create_effect(move |did_run_before| {
    //     if !is_active.get() {
    //         return;
    //     }
    //
    //     let deps_value = deps();
    //
    //     if !options.immediate && did_run_before.is_none() {
    //         prev_deps_value.replace(Some(deps_value));
    //         return;
    //     }
    //
    //     cur_deps_value.replace(Some(deps_value.clone()));
    //
    //
    //     prev_deps_value.replace(Some(deps_value));
    // });
    //
    //
}

/// Options for `watch_with_options`
#[derive(DefaultBuilder, Default)]
pub struct WatchOptions {
    /// If `immediate` is true, the `callback` will run immediately.
    /// If it's `false, the `callback` will run only after
    /// the first change is detected of any signal that is accessed in `deps`.
    /// Defaults to `false`.
    immediate: bool,

    /// Allows to debounce or throttle the callback. Defaults to no filter.
    filter: FilterOptions,
}

impl WatchOptions {
    filter_builder_methods!(
        /// the watch callback
        filter
    );
}
