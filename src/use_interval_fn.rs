use crate::utils::Pausable;
use crate::watch;
use default_struct_builder::DefaultBuilder;
use leptos::leptos_dom::helpers::IntervalHandle;
use leptos::*;
use std::cell::Cell;
use std::rc::Rc;
use std::time::Duration;

/// Wrapper for `set_interval` with controls.
///
/// ## Demo
///
/// [Link to Demo](https://github.com/Synphonyte/leptos-use/tree/main/examples/use_interval_fn)
///
/// ## Usage
///
/// ```
/// # use leptos::*;
/// # use leptos_use::use_interval_fn;
/// # use leptos_use::utils::Pausable;
/// #
/// # #[component]
/// # fn Demo(cx: Scope) -> impl IntoView {
/// let Pausable { pause, resume, is_active } = use_interval_fn(
///     cx,
///     || {
///         // do something
///     },
///     1000,
/// );
/// # view! { cx, }
/// # }
/// ```
pub fn use_interval_fn<CbFn, N>(
    cx: Scope,
    callback: CbFn,
    interval: N,
) -> Pausable<impl Fn() + Clone, impl Fn() + Clone>
where
    CbFn: Fn() + Clone + 'static,
    N: Into<MaybeSignal<u64>>,
{
    use_interval_fn_with_options(cx, callback, interval, UseIntervalFnOptions::default())
}

/// Version of [`use_interval_fn`] that takes `UseIntervalFnOptions`. See [`use_interval_fn`] for how to use.
pub fn use_interval_fn_with_options<CbFn, N>(
    cx: Scope,
    callback: CbFn,
    interval: N,
    options: UseIntervalFnOptions,
) -> Pausable<impl Fn() + Clone, impl Fn() + Clone>
where
    CbFn: Fn() + Clone + 'static,
    N: Into<MaybeSignal<u64>>,
{
    let UseIntervalFnOptions {
        immediate,
        immediate_callback,
    } = options;

    let timer: Rc<Cell<Option<IntervalHandle>>> = Rc::new(Cell::new(None));

    let (is_active, set_active) = create_signal(cx, false);

    let clean = {
        let timer = Rc::clone(&timer);

        move || {
            if let Some(handle) = timer.take() {
                handle.clear();
            }
        }
    };

    let pause = {
        let clean = clean.clone();

        move || {
            set_active.set(false);
            clean();
        }
    };

    let interval = interval.into();

    let resume = move || {
        let interval_value = interval.get();
        if interval_value == 0 {
            return;
        }

        set_active.set(true);

        if immediate_callback {
            callback.clone()();
        }
        clean();

        timer.set(
            set_interval_with_handle(callback.clone(), Duration::from_millis(interval_value)).ok(),
        );
    };

    if immediate {
        resume();
    }

    if matches!(interval, MaybeSignal::Dynamic(_)) {
        let resume = resume.clone();

        let stop_watch = watch(cx, move || interval.get(), move |_, _, _| {
            if is_active.get() {
                resume();
            }
        });
        on_cleanup(cx, stop_watch);
    }

    on_cleanup(cx, pause.clone());

    Pausable {
        is_active: is_active.into(),
        pause,
        resume,
    }
}

/// Options for [`use_interval_fn_with_options`]
#[derive(DefaultBuilder)]
pub struct UseIntervalFnOptions {
    /// Start the timer immediately. Defaults to `true`.
    pub immediate: bool,

    /// Execute the callback immediate after calling this function. Defaults to `false`
    pub immediate_callback: bool,
}

impl Default for UseIntervalFnOptions {
    fn default() -> Self {
        Self {
            immediate: true,
            immediate_callback: false,
        }
    }
}
