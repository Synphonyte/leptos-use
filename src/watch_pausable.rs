use crate::{watch_with_options, WatchOptions};
use leptos::prelude::*;

/// Pausable [`watch`].
///
/// ## Demo
///
/// [Link to Demo](https://github.com/Synphonyte/leptos-use/tree/main/examples/watch_pausable)
///
/// ## Usage
///
/// ```
/// # use leptos::prelude::*;
/// # use leptos::logging::log;
/// # use leptos_use::{watch_pausable, WatchPausableReturn};
/// #
/// # pub fn Demo() -> impl IntoView {
/// let (source, set_source) = signal("foo".to_string());
///
/// let WatchPausableReturn {
///     stop,
///     pause,
///     resume,
///     ..
/// } = watch_pausable(
///     move || source.get(),
///     |v, _, _| {
///         log!("Changed to {}", v);
///     },
/// );
///
/// set_source.set("bar".to_string()); // > "Changed to bar"
///
/// pause();
///
/// set_source.set("foobar".to_string()); // (nothing happens)
///
/// resume();
///
/// set_source.set("hello".to_string()); // > "Changed to hello"
/// #    view! { }
/// # }
/// ```
///
/// There's also [`watch_pausable_with_options`] which takes the same options as [`watch`].
///
/// ## Server-Side Rendering
///
/// > Make sure you follow the [instructions in Server-Side Rendering](https://leptos-use.rs/server_side_rendering.html).
///
/// On the server this works just fine except if you throttle or debounce in which case the callback
/// will never be called except if you set `immediate` to `true` in which case the callback will be
/// called exactly once.
///
/// ## See also
///
/// * `leptos::watch`
pub fn watch_pausable<W, T, DFn, CFn>(
    deps: DFn,
    callback: CFn,
) -> WatchPausableReturn<
    impl Fn() + Clone + Send + Sync,
    impl Fn() + Clone + Send + Sync,
    impl Fn() + Clone + Send + Sync,
>
where
    DFn: Fn() -> W + 'static,
    CFn: Fn(&W, Option<&W>, Option<T>) -> T + Clone + 'static,
    W: Clone + 'static,
    T: Clone + 'static,
{
    watch_pausable_with_options(deps, callback, WatchOptions::default())
}

/// Version of `watch_pausable` that accepts `WatchOptions`. See [`watch_pausable`] for how to use.
pub fn watch_pausable_with_options<W, T, DFn, CFn>(
    deps: DFn,
    callback: CFn,
    options: WatchOptions,
) -> WatchPausableReturn<
    impl Fn() + Clone + Send + Sync,
    impl Fn() + Clone + Send + Sync,
    impl Fn() + Clone + Send + Sync,
>
where
    DFn: Fn() -> W + 'static,
    CFn: Fn(&W, Option<&W>, Option<T>) -> T + Clone + 'static,
    W: Clone + 'static,
    T: Clone + 'static,
{
    let (is_active, set_active) = signal(true);

    let pausable_callback = move |val: &W, prev_val: Option<&W>, prev_ret: Option<Option<T>>| {
        if is_active.get_untracked() {
            Some(callback(val, prev_val, prev_ret.unwrap_or(None)))
        } else {
            None
        }
    };

    let stop = watch_with_options(deps, pausable_callback, options);

    let pause = move || {
        set_active.set(false);
    };

    let resume = move || {
        set_active.set(true);
    };

    WatchPausableReturn {
        stop,
        pause,
        resume,
        is_active: is_active.into(),
    }
}

/// Return type of [`watch_pausable`]
pub struct WatchPausableReturn<StopFn, PauseFn, ResumeFn>
where
    StopFn: Fn() + Clone + Send + Sync,
    PauseFn: Fn() + Clone + Send + Sync,
    ResumeFn: Fn() + Clone + Send + Sync,
{
    /// Stops the watcher
    pub stop: StopFn,

    /// Pauses the watcher
    pub pause: PauseFn,

    /// Resumes the watcher
    pub resume: ResumeFn,

    /// Whether the watcher is active (not paused). This doesn't reflect if the watcher has been stopped
    pub is_active: Signal<bool>,
}
