use leptos::*;
use std::marker::PhantomData;

/// Wrapper for `setTimeout` with controls.
///
/// ## Demo
///
/// [Link to Demo](https://github.com/Synphonyte/leptos-use/tree/main/examples/use_timeout_fn)
///
/// ## Usage
///
/// ```
/// # use leptos::*;
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
///
/// ## Server-Side Rendering
///
/// On the server the callback will never be run. The returned functions are all no-ops and
/// `is_pending` will always be `false`.
pub fn use_timeout_fn<CbFn, Arg, D>(
    callback: CbFn,
    delay: D,
) -> UseTimeoutFnReturn<impl Fn(Arg) + Clone, Arg, impl Fn() + Clone>
where
    CbFn: Fn(Arg) + Clone + 'static,
    Arg: 'static,
    D: Into<MaybeSignal<f64>>,
{
    let (is_pending, set_pending) = create_signal(false);

    let start;
    let stop;

    #[cfg(not(feature = "ssr"))]
    {
        use leptos::leptos_dom::helpers::TimeoutHandle;
        use std::cell::Cell;
        use std::rc::Rc;
        use std::time::Duration;

        let delay = delay.into();

        let timer = Rc::new(Cell::new(None::<TimeoutHandle>));

        let clear = {
            let timer = Rc::clone(&timer);

            move || {
                if let Some(timer) = timer.take() {
                    timer.clear();
                }
            }
        };

        stop = {
            let clear = clear.clone();

            move || {
                set_pending.set(false);
                clear();
            }
        };

        start = {
            let timer = Rc::clone(&timer);
            let callback = callback.clone();

            move |arg: Arg| {
                set_pending.set(true);

                let handle = set_timeout_with_handle(
                    {
                        let timer = Rc::clone(&timer);
                        let callback = callback.clone();

                        move || {
                            set_pending.set(false);
                            timer.set(None);

                            #[cfg(debug_assertions)]
                            let prev = SpecialNonReactiveZone::enter();

                            callback(arg);

                            #[cfg(debug_assertions)]
                            SpecialNonReactiveZone::exit(prev);
                        }
                    },
                    Duration::from_millis(delay.get_untracked() as u64),
                )
                .ok();

                timer.set(handle);
            }
        };

        on_cleanup(clear);
    }

    #[cfg(feature = "ssr")]
    {
        let _ = set_pending;
        let _ = callback;
        let _ = delay;

        start = move |_: Arg| ();
        stop = move || ();
    }

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
