use crate::core::now;
use crate::filter_builder_methods;
use crate::utils::{DebounceOptions, FilterOptions, ThrottleOptions};
use default_struct_builder::DefaultBuilder;
use leptos::prelude::*;
use leptos::reactive::wrappers::read::Signal;

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
/// # use leptos_use::{use_idle, UseIdleReturn};
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
/// # use leptos_use::{use_idle, UseIdleReturn};
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
        use crate::utils::create_filter_wrapper;
        use crate::{
            sendwrap_fn, use_document, use_event_listener, use_event_listener_with_options,
            UseEventListenerOptions,
        };
        use leptos::ev::{visibilitychange, Custom};
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
                        Duration::from_millis(timeout),
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

            let _ = use_event_listener(use_document(), visibilitychange, move |evt| {
                if !document().hidden() {
                    on_event(evt);
                }
            });
        }

        reset.clone()();
    }

    UseIdleReturn {
        idle: idle.into(),
        last_active: last_active.into(),
        reset,
    }
}

/// Options for [`use_idle_with_options`].
#[derive(DefaultBuilder)]
pub struct UseIdleOptions {
    /// Event names to listen to for detected user activity.
    /// Default: `vec!["mousemove", "mousedown", "resize", "keydown", "touchstart", "wheel"]`.
    events: Vec<String>,

    /// Whether to listen for document visibility change.
    /// Defaults to `true`.
    listen_for_visibility_change: bool,

    /// Initial state of the returned `idle`.
    /// Defaults to `false`.
    initial_state: bool,

    /// Allows to debounce or throttle the event listener that is called for
    /// every event (from `events`). Defaults to a throttle by 50ms.
    filter: FilterOptions,
}

impl Default for UseIdleOptions {
    fn default() -> Self {
        Self {
            events: vec![
                "mousemove".to_string(),
                "mousedown".to_string(),
                "resize".to_string(),
                "keydown".to_string(),
                "touchstart".to_string(),
                "wheel".to_string(),
            ],
            listen_for_visibility_change: true,
            initial_state: false,
            filter: FilterOptions::throttle(50.0),
        }
    }
}

impl UseIdleOptions {
    filter_builder_methods!(
        /// the event listener
        filter
    );
}

/// Return type of [`use_idle`].
pub struct UseIdleReturn<F>
where
    F: Fn() + Clone + Send + Sync,
{
    /// Wether the use has been inactive for at least `timeout` milliseconds.
    pub idle: Signal<bool>,

    /// Timestamp of last user activity.
    pub last_active: Signal<f64>,

    /// Reset function. Sets the idle state to `false`.
    pub reset: F,
}
