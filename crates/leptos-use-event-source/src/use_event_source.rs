#[cfg(not(feature = "ssr"))]
use crate::UseEventSourceOnEventReturn;
use crate::{
    ConnectionReadyState, UseEventSourceError, UseEventSourceMessage, UseEventSourceOptions,
    UseEventSourceReturn,
};
use codee::Decoder;
use leptos::prelude::*;
#[cfg(not(feature = "ssr"))]
use std::sync::Arc;

/// Reactive [EventSource](https://developer.mozilla.org/en-US/docs/Web/API/EventSource)
///
/// An [EventSource](https://developer.mozilla.org/en-US/docs/Web/API/EventSource) or
/// [Server-Sent-Events](https://developer.mozilla.org/en-US/docs/Web/API/Server-sent_events)
/// instance opens a persistent connection to an HTTP server,
/// which sends events in text/event-stream format.
///
/// ## Usage
///
/// Values are decoded via the given decoder. You can use any of the string codecs or a
/// binary codec wrapped in `Base64`.
///
/// > Please check [the codec chapter](https://leptos-use.rs/codecs.html) to see what codecs are
/// > available and what feature flags they require.
///
/// ```
/// # use leptos::prelude::*;
/// # use leptos_use_event_source::{use_event_source, UseEventSourceReturn};
/// # use codee::string::JsonSerdeCodec;
/// # use serde::{Deserialize, Serialize};
/// #
/// #[derive(Serialize, Deserialize, Clone, PartialEq)]
/// pub struct EventSourceData {
///     pub message: String,
///     pub priority: u8,
/// }
///
/// # #[component]
/// # fn Demo() -> impl IntoView {
/// let UseEventSourceReturn {
///     ready_state, message, error, close, ..
/// } = use_event_source::<EventSourceData, JsonSerdeCodec>("https://event-source-url");
/// #
/// # view! { }
/// # }
/// ```
///
/// ### Named Events
///
/// You can define named events when using `use_event_source_with_options`.
///
/// ```
/// # use leptos::prelude::*;
/// # use leptos_use_event_source::{use_event_source_with_options, UseEventSourceReturn, UseEventSourceOptions};
/// # use codee::string::FromToStringCodec;
/// #
/// # #[component]
/// # fn Demo() -> impl IntoView {
/// let UseEventSourceReturn {
///     ready_state, message, error, close, ..
/// } = use_event_source_with_options::<String, FromToStringCodec>(
///     "https://event-source-url",
///     UseEventSourceOptions::default()
///         .named_events(["notice".to_string(), "update".to_string()])
/// );
/// #
/// # view! { }
/// # }
/// ```
///
/// ### Custom Event Handler
///
/// You can provide a custom `on_event` handler using `use_event_source_with_options`.
/// `on_event` wil be run for every received event, including the built-in `open`, `error`,
/// and `message` events, as well as any named events you have specified.
///
/// With the return value of `on_event` you can control, whether `message` and named events
/// should be further processed by `use_event_source` (`UseEventSourceOnEventReturn::ProcessMessage`)
/// or ignored (`UseEventSourceOnEventReturn::IgnoreProcessingMessage`).
///
/// By default, the handler returns `UseEventSourceOnEventReturn::ProcessMessage`.
///
/// ```
/// # use leptos::prelude::*;
/// # use leptos_use_event_source::{use_event_source_with_options, UseEventSourceReturn, UseEventSourceOptions, UseEventSourceMessage, UseEventSourceOnEventReturn};
/// # use codee::string::FromToStringCodec;
/// #
/// # #[component]
/// # fn Demo() -> impl IntoView {
/// // Custom example handler: log event name and check for named `custom_error` event
/// let custom_event_handler = |e: &web_sys::Event| {
///     leptos::logging::log!("Received event: {}", e.type_());
///     if e.type_() == "custom_error" {
///         if let Ok(error_message) = UseEventSourceMessage::<String, FromToStringCodec>::try_from(e.clone()) {
///             // Decoded successfully, log the error message
///             leptos::logging::log!("Error message: {}", error_message.data);
///             // skip processing this message event further
///             return UseEventSourceOnEventReturn::IgnoreProcessingMessage;
///         }
///     }
///     // Process other message events normally
///     UseEventSourceOnEventReturn::ProcessMessage
/// };
/// let UseEventSourceReturn {
///     ready_state, message, error, close, ..
/// } = use_event_source_with_options::<String, FromToStringCodec>(
///     "https://event-source-url",
///     UseEventSourceOptions::default()
///         .named_events(["custom_error".to_string()])
///         .on_event(custom_event_handler)
/// );
/// #
/// # view! { }
/// # }
/// ```
///
/// ### Immediate
///
/// Auto-connect (enabled by default).
///
/// This will call `open()` automatically for you, and you don't need to call it by yourself.
///
/// ### Auto-Reconnection
///
/// Reconnect on errors automatically (enabled by default).
///
/// You can control the number of reconnection attempts by setting `reconnect_limit` and the
/// interval between them by setting `reconnect_interval`.
///
/// ```
/// # use leptos::prelude::*;
/// # use leptos_use_event_source::{use_event_source_with_options, UseEventSourceReturn, UseEventSourceOptions};
/// # use leptos_use_core::ReconnectLimit;
/// # use codee::string::FromToStringCodec;
/// #
/// # #[component]
/// # fn Demo() -> impl IntoView {
/// let UseEventSourceReturn {
///     ready_state, message, error, close, ..
/// } = use_event_source_with_options::<bool, FromToStringCodec>(
///     "https://event-source-url",
///     UseEventSourceOptions::default()
///         .reconnect_limit(ReconnectLimit::Limited(5))         // at most 5 attempts
///         .reconnect_interval(2000)   // wait for 2 seconds between attempts
/// );
/// #
/// # view! { }
/// # }
/// ```
///
///
/// ## SendWrapped Return
///
/// The returned closures `open` and `close` are sendwrapped functions. They can
/// only be called from the same thread that called `use_event_source`.
///
/// To disable auto-reconnection, set `reconnect_limit` to `0`.
///
/// ## Server-Side Rendering
///
/// > Make sure you follow the [instructions in Server-Side Rendering](https://leptos-use.rs/server_side_rendering.html).
///
/// On the server-side, `use_event_source` will always return `ready_state` as `ConnectionReadyState::Closed`,
/// `data`, `event` and `error` will always be `None`, and `open` and `close` will do nothing.
pub fn use_event_source<T, C>(
    url: impl Into<Signal<String>>,
) -> UseEventSourceReturn<
    T,
    C,
    C::Error,
    impl Fn() + Clone + Send + Sync + 'static,
    impl Fn() + Clone + Send + Sync + 'static,
>
where
    T: Clone + PartialEq + Send + Sync + 'static,
    C: Decoder<T, Encoded = str> + Send + Sync,
    C::Error: Send + Sync,
{
    use_event_source_with_options::<T, C>(url, UseEventSourceOptions::<T>::default())
}

/// Version of [`use_event_source`] that takes a `UseEventSourceOptions`. See [`use_event_source`] for how to use.
pub fn use_event_source_with_options<T, C>(
    url: impl Into<Signal<String>>,
    options: UseEventSourceOptions<T>,
) -> UseEventSourceReturn<
    T,
    C,
    C::Error,
    impl Fn() + Clone + Send + Sync + 'static,
    impl Fn() + Clone + Send + Sync + 'static,
>
where
    T: Clone + PartialEq + Send + Sync + 'static,
    C: Decoder<T, Encoded = str> + Send + Sync,
    C::Error: Send + Sync,
{
    let UseEventSourceOptions {
        reconnect_limit,
        reconnect_interval,
        on_failed,
        immediate,
        named_events,
        on_event,
        with_credentials,
        _marker,
    } = options;

    let (message, set_message) = signal(None::<UseEventSourceMessage<T, C>>);
    let (ready_state, set_ready_state) = signal(ConnectionReadyState::Closed);
    let (error, set_error) = signal(None::<UseEventSourceError<C::Error>>);

    let open;
    let close;

    #[cfg(not(feature = "ssr"))]
    {
        use crate::{sendwrap_fn, use_event_listener};
        use leptos::leptos_dom::helpers::TimeoutHandle;
        use std::sync::atomic::{AtomicBool, AtomicU32};
        use std::time::Duration;
        use wasm_bindgen::prelude::*;

        let (event_source, set_event_source) = signal_local(None::<web_sys::EventSource>);
        let explicitly_closed = Arc::new(AtomicBool::new(false));
        let retried = Arc::new(AtomicU32::new(0));

        let on_event_return = move |e: &web_sys::Event| {
            // make sure handler doesn't create reactive dependencies
            #[cfg(debug_assertions)]
            let _ = leptos::reactive::diagnostics::SpecialNonReactiveZone::enter();

            on_event(e)
        };

        let on_message_event = {
            let on_event_return = on_event_return.clone();
            move |e: &web_sys::Event| {
                match on_event_return(e) {
                    UseEventSourceOnEventReturn::IgnoreProcessingMessage => {
                        // skip processing message event!
                    }
                    UseEventSourceOnEventReturn::ProcessMessage => {
                        let message_event = e
                            .dyn_ref::<web_sys::MessageEvent>()
                            .expect("Event is not a MessageEvent");

                        match UseEventSourceMessage::<T, C>::try_from(message_event) {
                            Ok(event_msg) => {
                                set_message.set(Some(event_msg));
                            }
                            Err(err) => {
                                set_error.set(Some(err));
                            }
                        }
                    }
                }
            }
        };

        let init = StoredValue::new(None::<Arc<dyn Fn() + Send + Sync>>);

        let reconnect_timer: StoredValue<Option<TimeoutHandle>> = StoredValue::new(None);

        let clear_reconnect_timer = move || {
            reconnect_timer.update_value(|timer| {
                if let Some(timer) = timer.take() {
                    timer.clear();
                }
            });
        };

        let set_init = {
            let explicitly_closed = Arc::clone(&explicitly_closed);
            let retried = Arc::clone(&retried);

            move |url: String| {
                init.set_value(Some(Arc::new({
                    let explicitly_closed = Arc::clone(&explicitly_closed);
                    let retried = Arc::clone(&retried);
                    let on_event_return = on_event_return.clone();
                    let on_message_event = on_message_event.clone();
                    let named_events = named_events.clone();
                    let on_failed = Arc::clone(&on_failed);

                    move || {
                        if explicitly_closed.load(std::sync::atomic::Ordering::Relaxed) {
                            return;
                        }

                        let event_src_opts = web_sys::EventSourceInit::new();
                        event_src_opts.set_with_credentials(with_credentials);

                        let es = web_sys::EventSource::new_with_event_source_init_dict(
                            &url,
                            &event_src_opts,
                        )
                        .unwrap_throw();

                        set_ready_state.set(ConnectionReadyState::Connecting);

                        set_event_source.set(Some(es.clone()));

                        let on_open = Closure::wrap(Box::new({
                            let on_event_return = on_event_return.clone();
                            move |e: web_sys::Event| {
                                on_event_return(&e);
                                set_ready_state.set(ConnectionReadyState::Open);
                                set_error.set(None);
                            }})
                                as Box<dyn FnMut(web_sys::Event)>);
                        es.set_onopen(Some(on_open.as_ref().unchecked_ref()));
                        on_open.forget();

                        let on_error = Closure::wrap(Box::new({
                            let on_event_return = on_event_return.clone();
                            let explicitly_closed = Arc::clone(&explicitly_closed);
                            let retried = Arc::clone(&retried);
                            let on_failed = Arc::clone(&on_failed);
                            let es = es.clone();

                            move |e: web_sys::Event| {
                                on_event_return(&e);
                                set_ready_state.set(ConnectionReadyState::Closed);
                                set_error.set(Some(UseEventSourceError::ErrorEvent));

                                // only reconnect if EventSource isn't reconnecting by itself
                                // this is the case when the connection is closed (readyState is 2)
                                if es.ready_state() == 2
                                    && !explicitly_closed.load(std::sync::atomic::Ordering::Relaxed)
                                {
                                    es.close();

                                    let retried_value = retried
                                        .fetch_add(1, std::sync::atomic::Ordering::Relaxed)
                                        + 1;

                                    if !reconnect_limit.is_exceeded_by(retried_value as u64) {
                                        clear_reconnect_timer();

                                        reconnect_timer.set_value(
                                            set_timeout_with_handle(
                                                move || {
                                                    reconnect_timer.set_value(None);

                                                    if let Some(init) = init.get_value() {
                                                        init();
                                                    }
                                                },
                                                Duration::from_millis(reconnect_interval),
                                            )
                                            .ok(),
                                        );
                                    } else {
                                        #[cfg(debug_assertions)]
                                        let _z =
                                            leptos::reactive::diagnostics::SpecialNonReactiveZone::enter();

                                        on_failed();
                                    }
                                }
                            }
                        })
                            as Box<dyn FnMut(web_sys::Event)>);
                        es.set_onerror(Some(on_error.as_ref().unchecked_ref()));
                        on_error.forget();

                        let on_message = Closure::wrap(Box::new({
                            let on_message_event = on_message_event.clone();
                            move |e: web_sys::MessageEvent| {
                                let e: &web_sys::Event = e.as_ref();
                                on_message_event(e);
                            }})
                                as Box<dyn FnMut(web_sys::MessageEvent)>);
                        es.set_onmessage(Some(on_message.as_ref().unchecked_ref()));
                        on_message.forget();

                        for event_name in named_events.clone() {
                            let event_handler = {
                                let on_message_event = on_message_event.clone();
                                move |e: web_sys::Event| {
                                    on_message_event(&e);
                                }
                            };

                            let _ = use_event_listener(
                                es.clone(),
                                leptos::ev::Custom::<leptos::ev::Event>::new(event_name),
                                event_handler,
                            );
                        }
                    }
                })))
            }
        };

        close = {
            let explicitly_closed = Arc::clone(&explicitly_closed);

            sendwrap_fn!(move || {
                // A reconnect scheduled by an earlier error would otherwise still fire and
                // open a second connection behind the caller's back.
                clear_reconnect_timer();

                if let Some(event_source) = event_source.get_untracked() {
                    event_source.close();
                    set_event_source.set(None);
                    set_ready_state.set(ConnectionReadyState::Closed);
                    explicitly_closed.store(true, std::sync::atomic::Ordering::Relaxed);
                }
            })
        };

        let url: Signal<String> = url.into();

        open = {
            let close = close.clone();
            let explicitly_closed = Arc::clone(&explicitly_closed);
            let retried = Arc::clone(&retried);
            let set_init = set_init.clone();

            sendwrap_fn!(move || {
                close();
                explicitly_closed.store(false, std::sync::atomic::Ordering::Relaxed);
                retried.store(0, std::sync::atomic::Ordering::Relaxed);
                if init.get_value().is_none() && !url.get_untracked().is_empty() {
                    set_init(url.get_untracked());
                }
                if let Some(init) = init.get_value() {
                    init();
                }
            })
        };

        {
            let close = close.clone();
            let open = open.clone();
            let set_init = set_init.clone();
            Effect::watch(
                move || url.get(),
                move |url, prev_url, _| {
                    if url.is_empty() {
                        close();
                    } else if Some(url) != prev_url {
                        close();
                        set_init(url.to_owned());
                        open();
                    }
                },
                immediate,
            );
        }

        on_cleanup(close.clone());
    }

    #[cfg(feature = "ssr")]
    {
        open = move || {};
        close = move || {};

        let _ = reconnect_limit;
        let _ = reconnect_interval;
        let _ = on_failed;
        let _ = immediate;
        let _ = named_events;
        let _ = on_event;
        let _ = with_credentials;

        let _ = set_message;
        let _ = set_ready_state;
        let _ = set_error;
        let _ = url;
    }

    UseEventSourceReturn {
        message: message.into(),
        ready_state: ready_state.into(),
        error: error.into(),
        open,
        close,
    }
}
