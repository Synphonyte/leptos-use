use crate::core::ConnectionReadyState;
use crate::ReconnectLimit;
use codee::Decoder;
use default_struct_builder::DefaultBuilder;
use leptos::prelude::*;
use std::fmt::Debug;
use std::marker::PhantomData;
use std::sync::Arc;
use thiserror::Error;

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
/// # use leptos_use::{use_event_source, UseEventSourceReturn};
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
///     ready_state, data, error, close, ..
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
/// # use leptos_use::{use_event_source_with_options, UseEventSourceReturn, UseEventSourceOptions};
/// # use codee::string::FromToStringCodec;
/// #
/// # #[component]
/// # fn Demo() -> impl IntoView {
/// let UseEventSourceReturn {
///     ready_state, data, error, close, ..
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
/// You can also provide a custom handler for named events via `named_event_handler`. This handler
/// receives the `MessageEvent` and should return `true` if the event data is custom or has no data,
/// and `false` if the data is expected to be of the same type as the message event data.
/// By default, the handler returns `false`.
/// 
/// ```
/// # use leptos::prelude::*;
/// # use leptos_use::{use_event_source_with_options, UseEventSourceReturn, UseEventSourceOptions};
/// # use codee::string::FromToStringCodec;
/// #
/// # #[component]
/// # fn Demo() -> impl IntoView {
/// let named_event_handler = |e: &web_sys::MessageEvent| {
///     // Custom logic to determine if the event data is custom or has no data
///     leptos::logging::log!("Received named event: {}", e.type_());
///     // Example: return true if data is null
///     e.data().is_null()
/// };
/// let UseEventSourceReturn {
///     ready_state, data, error, close, ..
/// } = use_event_source_with_options::<bool, FromToStringCodec>(
///     "https://event-source-url",
///     UseEventSourceOptions::default()
///         .named_events(["custom_event".to_string()])
///         .named_event_handler(named_event_handler)
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
/// # use leptos_use::{use_event_source_with_options, UseEventSourceReturn, UseEventSourceOptions, ReconnectLimit};
/// # use codee::string::FromToStringCodec;
/// #
/// # #[component]
/// # fn Demo() -> impl IntoView {
/// let UseEventSourceReturn {
///     ready_state, data, error, close, ..
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
        named_event_handler,
        with_credentials,
        _marker,
    } = options;

    let (event, set_event) = signal(None::<UseEventSourceMessage<T, C>>);
    // ToDo: should we keep data? event contains data already. This would only be useful for backwards compatibility.
    let (data, set_data) = signal(None::<T>);
    let (ready_state, set_ready_state) = signal(ConnectionReadyState::Closed);
    let (error, set_error) = signal(None::<UseEventSourceError<C::Error>>);

    let open;
    let close;

    #[cfg(not(feature = "ssr"))]
    {
        use crate::{sendwrap_fn, use_event_listener};
        use std::sync::atomic::{AtomicBool, AtomicU32};
        use std::time::Duration;
        use wasm_bindgen::prelude::*;

        let (event_source, set_event_source) = signal_local(None::<web_sys::EventSource>);
        let explicitly_closed = Arc::new(AtomicBool::new(false));
        let retried = Arc::new(AtomicU32::new(0));

        let set_event_from_message_event = move |message_event: &web_sys::MessageEvent| {
            match UseEventSourceMessage::<T, C>::try_from(message_event) {
                Ok(event_msg) => {
                    set_data.set(Some(event_msg.data.clone()));
                    set_event.set(Some(event_msg));
                }
                Err(err) => {
                    set_error.set(Some(UseEventSourceError::Deserialize(err)));
                }
            }
        };

        let init = StoredValue::new(None::<Arc<dyn Fn() + Send + Sync>>);

        let set_init = {
            let explicitly_closed = Arc::clone(&explicitly_closed);
            let retried = Arc::clone(&retried);

            move |url: String| {
                init.set_value(Some(Arc::new({
                    let explicitly_closed = Arc::clone(&explicitly_closed);
                    let retried = Arc::clone(&retried);
                    let named_event_handler = Arc::clone(&named_event_handler);
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

                        let on_open = Closure::wrap(Box::new(move |_: web_sys::Event| {
                            set_ready_state.set(ConnectionReadyState::Open);
                            set_error.set(None);
                        })
                            as Box<dyn FnMut(web_sys::Event)>);
                        es.set_onopen(Some(on_open.as_ref().unchecked_ref()));
                        on_open.forget();

                        let on_error = Closure::wrap(Box::new({
                            let explicitly_closed = Arc::clone(&explicitly_closed);
                            let retried = Arc::clone(&retried);
                            let on_failed = Arc::clone(&on_failed);
                            let es = es.clone();

                            move |e: web_sys::Event| {
                                set_ready_state.set(ConnectionReadyState::Closed);
                                if let Ok(message_event) = e.dyn_into::<web_sys::MessageEvent>() {
                                    let Ok(error_msg) = UseEventSourceMessage::<String, codee::string::FromToStringCodec>::try_from(&message_event);
                                    // ToDo: should we close and reconnect, if user defined error is received?
                                    // Perhaps make this configurable with a provided error handler?
                                    // we could also reuse the named_events_handler for this.
                                    set_error.set(Some(UseEventSourceError::ServerErrorEvent(error_msg)));
                                } else {
                                    set_error.set(Some(UseEventSourceError::SseErrorEvent));
                                };

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
                                        set_timeout(
                                            move || {
                                                if let Some(init) = init.get_value() {
                                                    init();
                                                }
                                            },
                                            Duration::from_millis(reconnect_interval),
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

                        let on_message = Closure::wrap(Box::new(move |e: web_sys::MessageEvent| {
                            set_event_from_message_event(&e);
                        })
                            as Box<dyn FnMut(web_sys::MessageEvent)>);
                        es.set_onmessage(Some(on_message.as_ref().unchecked_ref()));
                        on_message.forget();

                        for event_name in named_events.clone() {
                            let event_handler = {
                                let handler = named_event_handler.clone();
                                let name = event_name.clone();

                                move |e: web_sys::Event| {
                                let e = if let Ok(message_event) = e.dyn_into::<web_sys::MessageEvent>() {
                                    message_event
                                } else {
                                    set_error.set(Some(UseEventSourceError::CastToMessageEvent(name.clone())));
                                    return;
                                };

                                if !handler.as_ref()(&e) {
                                    set_event_from_message_event(&e);
                                }

                            }};

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
        let _ = named_event_handler;
        let _ = with_credentials;

        let _ = set_event;
        let _ = set_data;
        let _ = set_ready_state;
        let _ = set_error;
        let _ = url;
    }

    UseEventSourceReturn {
        event: event.into(),
        data: data.into(),
        ready_state: ready_state.into(),
        error: error.into(),
        open,
        close,
    }
}

#[derive(Clone, PartialEq)]
pub struct UseEventSourceMessage<T, C>
where
    T: Clone + Send + Sync + 'static,
    C: Decoder<T, Encoded = str> + Send + Sync,
    C::Error: Send + Sync,
{
    pub data: T,
    pub event_type: String,
    pub last_event_id: String,
    _marker: PhantomData<C>,
}

impl<T, C> Debug for UseEventSourceMessage<T, C>
where
    T: Debug + Clone + Send + Sync + 'static,
    C: Decoder<T, Encoded = str> + Send + Sync,
    C::Error: Send + Sync,
{
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("UseEventSourceMessage")
            .field("data", &self.data)
            .field("event_type", &self.event_type)
            .field("last_event_id", &self.last_event_id)
            .finish()
    }
}

impl<T, C> TryFrom<&web_sys::MessageEvent> for UseEventSourceMessage<T, C>
where
    T: Clone + Send + Sync + 'static,
    C: Decoder<T, Encoded = str> + Send + Sync,
    C::Error: Send + Sync,
{
    type Error = C::Error;

    fn try_from(message_event: &web_sys::MessageEvent) -> Result<Self, Self::Error> {
        let data_string = message_event.data().as_string().unwrap_or_default();

        let data = C::decode(&data_string)?;

        Ok(Self {
            data,
            event_type: message_event.type_(),
            last_event_id: message_event.last_event_id(),
            _marker: PhantomData,
        })
    }
}

/// Options for [`use_event_source_with_options`].
#[derive(DefaultBuilder)]
pub struct UseEventSourceOptions<T>
where
    T: 'static,
{
    /// Retry times. Defaults to `ReconnectLimit::Limited(3)`. Use `ReconnectLimit::Infinite` for
    /// infinite retries.
    reconnect_limit: ReconnectLimit,

    /// Retry interval in ms. Defaults to 3000.
    reconnect_interval: u64,

    /// On maximum retry times reached.
    on_failed: Arc<dyn Fn() + Send + Sync>,

    /// If `true` the `EventSource` connection will immediately be opened when calling this function.
    /// If `false` you have to manually call the `open` function.
    /// Defaults to `true`.
    immediate: bool,

    /// List of named events to listen for on the `EventSource`.
    #[builder(into)]
    named_events: Vec<String>,

    /// Handling named events. Returns true, if data of named event is custom or has no data.
    /// Default handler returns false (data is expected to be of same type as message event data).
    named_event_handler: Arc<dyn Fn(&web_sys::MessageEvent) -> bool + Send + Sync>,

    /// If CORS should be set to `include` credentials. Defaults to `false`.
    with_credentials: bool,

    _marker: PhantomData<T>,
}

impl<T> Default for UseEventSourceOptions<T> {
    fn default() -> Self {
        Self {
            reconnect_limit: ReconnectLimit::default(),
            reconnect_interval: 3000,
            on_failed: Arc::new(|| {}),
            immediate: true,
            named_events: vec![],
            named_event_handler: Arc::new(|_| false),
            with_credentials: false,
            _marker: PhantomData,
        }
    }
}

/// Return type of [`use_event_source`].
pub struct UseEventSourceReturn<T, C, Err, OpenFn, CloseFn>
where
    T: Clone + Send + Sync + 'static,
    C: Decoder<T, Encoded = str> + Send + Sync,
    C::Error: Send + Sync,
    Err: Send + Sync + 'static,
    OpenFn: Fn() + Clone + Send + Sync + 'static,
    CloseFn: Fn() + Clone + Send + Sync + 'static,
{
    /// Latest data received via the `EventSource`
    pub data: Signal<Option<T>>,

    /// The current state of the connection,
    pub ready_state: Signal<ConnectionReadyState>,

    /// The latest named event
    pub event: Signal<Option<UseEventSourceMessage<T, C>>>,

    /// The current error
    pub error: Signal<Option<UseEventSourceError<Err>>>,

    /// (Re-)Opens the `EventSource` connection
    /// If the current one is active, will close it before opening a new one.
    pub open: OpenFn,

    /// Closes the `EventSource` connection
    pub close: CloseFn,
}

#[derive(Error, Debug)]
pub enum UseEventSourceError<Err> {
    #[error("SSE error event received")]
    SseErrorEvent,

    #[error("An error occurred on server: {0:?}")]
    ServerErrorEvent(UseEventSourceMessage<String, codee::string::FromToStringCodec>),

    #[error("Error decoding value")]
    Deserialize(Err),

    #[error("Error casting event '{0}' to MessageEvent")]
    CastToMessageEvent(String),
}
