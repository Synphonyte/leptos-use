use crate::utils::Pausable;
use crate::{sendwrap_fn, use_interval_fn_with_options, UseIntervalFnOptions};
use default_struct_builder::DefaultBuilder;
use leptos::prelude::*;
use leptos::reactive::wrappers::read::Signal;
use std::rc::Rc;

/// Reactive counter increases on every interval.
///
/// ## Demo
///
/// [Link to Demo](https://github.com/Synphonyte/leptos-use/tree/main/examples/use_interval)
///
/// ## Usage
///
/// ```
/// # use leptos::prelude::*;
/// # use leptos_use::{use_interval, UseIntervalReturn};
/// #
/// # #[component]
/// # fn Demo() -> impl IntoView {
/// let UseIntervalReturn {
///     counter,
///     reset,
///     is_active,
///     pause,
///     resume
/// }  = use_interval( 200 );
/// # view! { }
/// # }
/// ```
///
/// ## SendWrapped Return
///
/// The returned closures `pause`, `resume` and `reset` are sendwrapped functions. They can
/// only be called from the same thread that called `use_intersection_observer`.
///
/// ## Server-Side Rendering
///
/// > Make sure you follow the [instructions in Server-Side Rendering](https://leptos-use.rs/server_side_rendering.html).
///
/// On the server this function will simply be ignored.
pub fn use_interval<N>(
    interval: N,
) -> UseIntervalReturn<
    impl Fn() + Clone + Send + Sync,
    impl Fn() + Clone + Send + Sync,
    impl Fn() + Clone + Send + Sync,
>
where
    N: Into<Signal<u64>>,
{
    use_interval_with_options(interval, UseIntervalOptions::default())
}

/// Version of [`use_interval`] that takes `UseIntervalOptions`. See [`use_interval`] for how to use.
pub fn use_interval_with_options<N>(
    interval: N,
    options: UseIntervalOptions,
) -> UseIntervalReturn<
    impl Fn() + Clone + Send + Sync,
    impl Fn() + Clone + Send + Sync,
    impl Fn() + Clone + Send + Sync,
>
where
    N: Into<Signal<u64>>,
{
    let UseIntervalOptions {
        immediate,
        callback,
    } = options;

    let (counter, set_counter) = signal(0u64);

    let update = move || set_counter.update(|count| *count += 1);
    let reset = sendwrap_fn!(move || set_counter.set(0));

    let cb = move || {
        update();
        callback(counter.get());
    };

    let Pausable {
        is_active,
        pause,
        resume,
    } = use_interval_fn_with_options(
        cb,
        interval,
        UseIntervalFnOptions {
            immediate,
            immediate_callback: false,
        },
    );

    UseIntervalReturn {
        counter: counter.into(),
        reset,
        is_active,
        pause,
        resume,
    }
}

/// Options for [`use_interval_with_options`]
#[derive(DefaultBuilder)]
pub struct UseIntervalOptions {
    /// Start the timer immediately. Defaults to `true`.
    immediate: bool,

    /// Callback on every interval.
    callback: Rc<dyn Fn(u64)>,
}

impl Default for UseIntervalOptions {
    fn default() -> Self {
        Self {
            immediate: true,
            callback: Rc::new(|_: u64| {}),
        }
    }
}

/// Return type of [`use_interval`].
#[derive(DefaultBuilder)]
pub struct UseIntervalReturn<PauseFn, ResumeFn, ResetFn>
where
    PauseFn: Fn() + Clone + Send + Sync,
    ResumeFn: Fn() + Clone + Send + Sync,
    ResetFn: Fn() + Clone + Send + Sync,
{
    /// Counter signal that increases by one every interval.
    pub counter: Signal<u64>,

    /// Reset the counter to zero
    pub reset: ResetFn,

    /// A Signal that indicates whether the counter is active. `false` when paused.
    pub is_active: Signal<bool>,

    /// Temporarily pause the counter
    pub pause: PauseFn,

    /// Resume the counter
    pub resume: ResumeFn,
}
