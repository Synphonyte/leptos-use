#![cfg_attr(feature = "ssr", allow(unused_variables, unused_imports))]

use crate::utils::Pausable;
use default_struct_builder::DefaultBuilder;
use leptos::leptos_dom::helpers::IntervalHandle;
use leptos::prelude::*;
use send_wrapper::SendWrapper;
use std::cell::Cell;
use std::sync::Arc;
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
/// # use leptos::prelude::*;
/// # use leptos_use::use_interval_fn;
/// # use leptos_use::utils::Pausable;
/// #
/// # #[component]
/// # fn Demo() -> impl IntoView {
/// let Pausable { pause, resume, is_active } = use_interval_fn(
///     || {
///         // do something
///     },
///     1000,
/// );
/// # view! { }
/// # }
/// ```
///
/// ## Server-Side Rendering
///
/// On the server this function will simply be ignored.
pub fn use_interval_fn<CbFn, N>(
    callback: CbFn,
    interval: N,
) -> Pausable<impl Fn() + Clone + Send + Sync, impl Fn() + Clone + Send + Sync>
where
    CbFn: Fn() + Clone + 'static + Send + Sync,
    N: Into<MaybeSignal<u64>>,
{
    use_interval_fn_with_options(callback, interval, UseIntervalFnOptions::default())
}

/// Version of [`use_interval_fn`] that takes `UseIntervalFnOptions`. See [`use_interval_fn`] for how to use.
pub fn use_interval_fn_with_options<CbFn, N>(
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

    let timer: Arc<SendWrapper<Cell<Option<IntervalHandle>>>> =
        Arc::new(SendWrapper::new(Cell::new(None)));

    let (is_active, set_active) = signal(false);

    let clean = {
        let timer = Arc::clone(&timer);

        move || {
            if let Some(handle) = Cell::take(&timer) {
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
        #[cfg(not(feature = "ssr"))]
        {
            let interval_value = interval.get();
            if interval_value == 0 {
                return;
            }

            set_active.set(true);

            let callback = {
                let callback = callback.clone();

                move || {
                    #[cfg(debug_assertions)]
                    let _z = leptos::reactive::diagnostics::SpecialNonReactiveZone::enter();

                    callback();
                }
            };

            if immediate_callback {
                callback.clone()();
            }
            clean();

            timer.set(
                set_interval_with_handle(callback.clone(), Duration::from_millis(interval_value))
                    .ok(),
            );
        }
    };

    if immediate {
        resume();
    }

    if matches!(interval, MaybeSignal::Dynamic(_)) {
        #[allow(clippy::clone_on_copy)]
        let resume = resume.clone();

        let effect = Effect::watch(
            move || interval.get(),
            move |_, _, _| {
                if is_active.get() {
                    resume();
                }
            },
            false,
        );
        on_cleanup(move || effect.stop());
    }

    on_cleanup({
        let pause = SendWrapper::new(pause.clone());
        #[allow(clippy::redundant_closure)]
        move || pause()
    });

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
