use leptos::leptos_dom::helpers::TimeoutHandle;
use leptos::prelude::diagnostics::SpecialNonReactiveZone;
use leptos::prelude::wrappers::read::Signal;
use leptos::prelude::*;
use std::marker::PhantomData;
use std::sync::{Arc, Mutex};
use std::time::Duration;

/// Wrapper for `setTimeout` with controls.
///
/// ## Demo
///
/// [Link to Demo](https://github.com/Synphonyte/leptos-use/tree/main/examples/use_timeout_fn)
///
/// ## Usage
///
/// ```
/// # use leptos::prelude::*;
/// # use leptos_use::{use_timeout_fn, UseTimeoutFnReturn};
/// #
/// # #[component]
/// # fn Demo() -> impl IntoView {
/// let UseTimeoutFnReturn { start, stop, is_pending, .. } = use_timeout_fn(
///     |i: i32| {
///         // do sth
///     },
///     3000.0
/// );
///
/// start(3);
/// #
/// # view! { }
/// # }
/// ```
pub fn use_timeout_fn<CbFn, Arg, D>(
    callback: CbFn,
    delay: D,
) -> UseTimeoutFnReturn<impl Fn(Arg) + Clone, Arg, impl Fn() + Clone>
where
    CbFn: Fn(Arg) + Clone + 'static,
    Arg: 'static,
    D: Into<MaybeSignal<f64>>,
{
    let delay = delay.into();

    let (is_pending, set_pending) = signal(false);

    let timer = Arc::new(Mutex::new(None::<TimeoutHandle>));

    let clear = {
        let timer = Arc::clone(&timer);

        move || {
            let timer = timer.lock().unwrap();
            if let Some(timer) = *timer {
                timer.clear();
            }
        }
    };

    let stop = {
        let clear = clear.clone();

        move || {
            set_pending.set(false);
            clear();
        }
    };

    let start = {
        let timer = Arc::clone(&timer);
        let callback = callback.clone();

        move |arg: Arg| {
            set_pending.set(true);

            let handle = set_timeout_with_handle(
                {
                    let timer = Arc::clone(&timer);
                    let callback = callback.clone();

                    move || {
                        set_pending.set(false);
                        *timer.lock().unwrap() = None;

                        #[cfg(debug_assertions)]
                        let _z = SpecialNonReactiveZone::enter();

                        callback(arg);
                    }
                },
                Duration::from_millis(delay.get_untracked() as u64),
            )
            .ok();

            *timer.lock().unwrap() = handle;
        }
    };

    on_cleanup(clear);

    UseTimeoutFnReturn {
        is_pending: is_pending.into(),
        start,
        stop,
        _marker: PhantomData,
    }
}

/// Return type of [`use_timeout_fn`].
pub struct UseTimeoutFnReturn<StartFn, StartArg, StopFn>
where
    StartFn: Fn(StartArg) + Clone,
    StopFn: Fn() + Clone,
{
    /// Whether the timeout is pending. When the `callback` is called this is set to `false`.
    pub is_pending: Signal<bool>,

    /// Start the timeout. The `callback` will be called after `delay` milliseconds.
    pub start: StartFn,

    /// Stop the timeout. If the timeout was still pending the `callback` is not called.
    pub stop: StopFn,

    _marker: PhantomData<StartArg>,
}
