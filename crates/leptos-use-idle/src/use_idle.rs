use crate::{UseIdleOptions, UseIdleReturn, now};
use leptos::prelude::*;

/// Tracks whether the user is being inactive.
///
/// ## Demo
///
/// [Link to Demo](https://github.com/Synphonyte/leptos-use/tree/main/examples/use_idle)
///
/// ## Usage
///
/// ```
/// # use leptos::prelude::*;
/// # use leptos::logging::log;
/// # use leptos_use_idle::{use_idle, UseIdleReturn};
/// #
/// # #[component]
/// # fn Demo() -> impl IntoView {
/// let UseIdleReturn {
///     idle, last_active, ..
/// } = use_idle(5 * 60 * 1000); // 5 minutes
///
/// log!("{}", idle.get()); // true or false
/// #
/// # view! { }
/// # }
/// ```
///
/// Programatically resetting:
///
/// ```
/// # use std::time::Duration;
/// use leptos::prelude::*;
/// # use leptos::logging::log;
/// # use leptos_use_idle::{use_idle, UseIdleReturn};
/// #
/// # #[component]
/// # fn Demo() -> impl IntoView {
/// let UseIdleReturn {
///     idle, last_active, reset
/// } = use_idle(5 * 60 * 1000); // 5 minutes
///
/// reset(); // restarts the idle timer. Does not change the `last_active` value.
/// #
/// # view! { }
/// # }
/// ```
///
/// ## SendWrapped Return
///
/// The returned closure `reset` is a sendwrapped function. It can
/// only be called from the same thread that called `use_idle`.
///
/// ## Server-Side Rendering
///
/// > Make sure you follow the [instructions in Server-Side Rendering](https://leptos-use.rs/server_side_rendering.html).
///
/// On the server this will always return static signals
///
/// ```ignore
/// UseIdleReturn{
///     idle: Signal(initial_state),
///     last_active: Signal(now),
///     reset: || {}
/// }
/// ```
pub fn use_idle(timeout: u64) -> UseIdleReturn<impl Fn() + Clone + Send + Sync> {
    use_idle_with_options(timeout, UseIdleOptions::default())
}

/// Version of [`use_idle`] that takes a `UseIdleOptions`. See [`use_idle`] for how to use.
pub fn use_idle_with_options(
    timeout: u64,
    options: UseIdleOptions,
) -> UseIdleReturn<impl Fn() + Clone + Send + Sync> {
    let UseIdleOptions {
        events,
        listen_for_visibility_change,
        initial_state,
        filter,
    } = options;

    let (idle, set_idle) = signal(initial_state);
    let (last_active, set_last_active) = signal(now());

    let reset;

    #[cfg(feature = "ssr")]
    {
        reset = || ();
        let _ = timeout;
        let _ = events;
        let _ = listen_for_visibility_change;
        let _ = filter;
        let _ = set_last_active;
        let _ = set_idle;
    }

    #[cfg(not(feature = "ssr"))]
    {
        use crate::create_filter_wrapper;
        use crate::{
            UseEventListenerOptions, sendwrap_fn, use_document, use_event_listener_with_options,
        };
        use leptos::ev::{Custom, visibilitychange};
        use leptos::leptos_dom::helpers::TimeoutHandle;
        use std::cell::Cell;
        use std::rc::Rc;
        use std::time::Duration;

        let timer = Rc::new(Cell::new(None::<TimeoutHandle>));

        reset = {
            let timer = Rc::clone(&timer);

            sendwrap_fn!(move || {
                set_idle.set(false);
                if let Some(timer) = timer.replace(
                    set_timeout_with_handle(
                        move || set_idle.set(true),
                        Duration::from_millis(timeout.min(i32::MAX as u64)),
                    )
                    .ok(),
                ) {
                    timer.clear();
                }
            })
        };

        let on_event = {
            let reset = reset.clone();

            let filtered_callback = create_filter_wrapper(filter.filter_fn(), move || {
                set_last_active.set(js_sys::Date::now());
                reset();
            });

            move |_: web_sys::Event| {
                filtered_callback();
            }
        };

        let listener_options = UseEventListenerOptions::default().passive(true);
        for event in events {
            let _ = use_event_listener_with_options(
                use_document(),
                Custom::new(event),
                on_event.clone(),
                listener_options,
            );
        }

        if listen_for_visibility_change {
            let on_event = on_event.clone();

            let _ = use_event_listener_with_options(
                use_document(),
                visibilitychange,
                move |evt| {
                    if !document().hidden() {
                        on_event(evt);
                    }
                },
                listener_options,
            );
        }

        reset.clone()();
    }

    UseIdleReturn {
        idle: idle.into(),
        last_active: last_active.into(),
        reset,
    }
}
