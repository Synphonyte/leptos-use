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
/// # use leptos_use::{use_event_source_with_options, UseEventSourceReturn, UseEventSourceOptions, UseEventSourceNamedEventOptions};
/// # use codee::string::FromToStringCodec;
/// #
/// # #[component]
/// # fn Demo() -> impl IntoView {
/// let UseEventSourceReturn {
///     ready_state, data, error, close, ..
/// } = use_event_source_with_options::<String, FromToStringCodec>(
///     "https://event-source-url",
///     UseEventSourceOptions::default()
///         .named_events([
///             UseEventSourceNamedEventOptions::default().name("notice"),
///             UseEventSourceNamedEventOptions::default().name("update"),
///         ])
/// );
/// #
/// # view! { }
/// # }
/// ```
///
/// You can also provide custom handlers for named events:
///
/// ```
/// # use leptos::{prelude::*, web_sys::MessageEvent};
/// # use leptos_use::{use_event_source_with_options, UseEventSourceReturn, UseEventSourceOptions, UseEventSourceNamedEventOptions};
/// # use codee::string::FromToStringCodec;
/// # use std::sync::Arc;
/// #
/// # #[component]
/// # fn Demo() -> impl IntoView {
/// let custom_handler = Arc::new(|event: MessageEvent| {
///     // Handle the event, e.g., log data
///    leptos::logging::log!("Custom event received: {}\ndata: {:?}", event.type_(), event.data().as_string());
/// });
/// let UseEventSourceReturn {
///     ready_state, data, error, close, ..
/// } = use_event_source_with_options::<String, FromToStringCodec>(
///     "https://event-source-url",
///     UseEventSourceOptions::default()
///         .named_events([
///             UseEventSourceNamedEventOptions::default().name("notice").handler(Some(custom_handler)),
///         ])
/// );
/// #
/// # view! { }
/// # }
/// ```
/// ['use_event_source_with_options'] expects custom events to provide data of the same type as the main event source data (T).
/// If your custom event has no data or data of a different type, set `no_or_custom_data` to true:
///
/// ```
/// # use leptos::prelude::*;
/// # use leptos_use::{use_event_source_with_options, UseEventSourceReturn, UseEventSourceOptions, UseEventSourceNamedEventOptions};
/// # use codee::string::FromToStringCodec;
/// #
/// # #[component]
/// # fn Demo() -> impl IntoView {
/// let UseEventSourceReturn {
///     ready_state, data, error, close, ..
/// } = use_event_source_with_options::<String, FromToStringCodec>(
///     "https://event-source-url",
///     UseEventSourceOptions::default()
///         .named_events([
///             UseEventSourceNamedEventOptions::default().name("ping").no_or_custom_data(true),
///         ])
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
    C::Error,
    impl Fn() + Clone + Send + Sync + 'static,
    impl Fn() + Clone + Send + Sync + 'static,
>
where
    T: Clone + PartialEq + Send + Sync + 'static,
    C: Decoder<T, Encoded = str>,
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
    C::Error,
    impl Fn() + Clone + Send + Sync + 'static,
    impl Fn() + Clone + Send + Sync + 'static,
>
where
    T: Clone + PartialEq + Send + Sync + 'static,
    C: Decoder<T, Encoded = str>,
    C::Error: Send + Sync,
{
    let UseEventSourceOptions {
        reconnect_limit,
        reconnect_interval,
        on_failed,
        immediate,
        named_events,
        with_credentials,
        _marker,
    } = options;

    let (event, set_event) = signal(None::<UseEventSourceMessage<T>>);
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

        let (event_source, set_event_source) = signal_local(None::<web_sys::EventSource>);
        let explicitly_closed = Arc::new(AtomicBool::new(false));
        let retried = Arc::new(AtomicU32::new(0));

        let set_event_from_message_event = move |message_event: &web_sys::MessageEvent| {
            match UseEventSourceMessage::<T>::decode::<C>(message_event) {
                Ok(event_msg) => {
                    set_data.set(Some(event_msg.data.clone()));
                    set_event.set(Some(event_msg));
                }
                Err(err) => {
                    set_error.set(Some(UseEventSourceError::Deserialize(err)));
                }
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

        let init = StoredValue::new(None::<Arc<dyn Fn() + Send + Sync>>);

        let set_init = {
            let explicitly_closed = Arc::clone(&explicitly_closed);
            let retried = Arc::clone(&retried);
            move |url: String| {
                init.set_value(Some(Arc::new({
                    let explicitly_closed = Arc::clone(&explicitly_closed);
                    let retried = Arc::clone(&retried);
                    let named_events = named_events.clone();
                    let on_failed = Arc::clone(&on_failed);

                    move || {
                        use wasm_bindgen::prelude::*;

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
                                    // ToDo: should we close and reconnect, if user defined error is received?
                                    let error_msg = UseEventSourceMessage {
                                        data: message_event
                                            .data()
                                            .as_string()
                                            .unwrap_or_default(),
                                        event_type: message_event.type_(),
                                        last_event_id: message_event.last_event_id(),
                                    };
                                    set_error.set(Some(UseEventSourceError::CustomErrorEvent(error_msg)))
                                } else {
                                    set_error.set(Some(UseEventSourceError::GenericErrorEvent));
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

                        for UseEventSourceNamedEventOptions { name, handler, no_or_custom_data } in named_events.clone() {
                            let event_handler = {
                                let handler = handler.clone();
                                let name = name.clone();

                                move |e: web_sys::Event| {
                                let e = if let Ok(message_event) = e.dyn_into::<web_sys::MessageEvent>() {
                                    message_event
                                } else {
                                    set_error.set(Some(UseEventSourceError::CastToMessageEvent(name.clone())));
                                    return;
                                };

                                if let Some(custom_handler) = handler.as_ref() {
                                    custom_handler(e.clone());
                                }

                                if !no_or_custom_data {
                                    set_event_from_message_event(&e);
                                }

                            }};

                            let _ = use_event_listener(
                                es.clone(),
                                leptos::ev::Custom::<leptos::ev::Event>::new(name),
                                event_handler,
                            );
                        }
                    }
                })))
            }
        };

        open = {
            let close = close.clone();
            let explicitly_closed = Arc::clone(&explicitly_closed);
            let retried = Arc::clone(&retried);

            sendwrap_fn!(move || {
                close();
                explicitly_closed.store(false, std::sync::atomic::Ordering::Relaxed);
                retried.store(0, std::sync::atomic::Ordering::Relaxed);
                if let Some(init) = init.get_value() {
                    init();
                }
            })
        };

        let url: Signal<String> = url.into();

        {
            let close = close.clone();
            let open = open.clone();
            Effect::watch(
                move || url.get(),
                move |url, prev_url, _| {
                    if Some(url) != prev_url && !url.is_empty() {
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
pub struct UseEventSourceMessage<T>
where
    T: Clone + Send + Sync + 'static,
{
    pub data: T,
    pub event_type: String,
    pub last_event_id: String,
}

impl Debug for UseEventSourceMessage<String> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("UseEventSourceMessage")
            .field("data", &self.data)
            .field("event_type", &self.event_type)
            .field("last_event_id", &self.last_event_id)
            .finish()
    }
}

impl<T> UseEventSourceMessage<T>
where
    T: Clone + Send + Sync + 'static,
{
    /// decodes a web_sys::MessageEvent into a UseEventSourceMessage
    pub fn decode<C>(message_event: &web_sys::MessageEvent) -> Result<Self, C::Error>
    where
        C: Decoder<T, Encoded = str>,
        C::Error: Send + Sync,
    {
        let data_string = message_event.data().as_string().unwrap_or_default();

        let data = C::decode(&data_string)?;

        Ok(Self {
            data,
            event_type: message_event.type_(),
            last_event_id: message_event.last_event_id(),
        })
    }
}

/// Options to configure Named Events for [`use_event_source_with_options`].
#[cfg_attr(feature = "ssr", allow(dead_code))]
#[derive(DefaultBuilder, Clone, Default)]
pub struct UseEventSourceNamedEventOptions {
    /// Name of the event
    #[builder(into)]
    name: String,

    /// Optional event handler
    /// Defaults to `None`.
    handler: Option<Arc<dyn Fn(web_sys::MessageEvent) + Send + Sync>>,

    /// If true, event has no or custom data. Custom data has to be handled by provided handler, if any.
    /// If false, event data is expected to be of the same type as message event data (T) and will be handled normally.
    /// Defaults to `false`.
    no_or_custom_data: bool,
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
    named_events: Vec<UseEventSourceNamedEventOptions>,

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
            with_credentials: false,
            _marker: PhantomData,
        }
    }
}

/// Return type of [`use_event_source`].
pub struct UseEventSourceReturn<T, Err, OpenFn, CloseFn>
where
    Err: Send + Sync + 'static,
    T: Clone + Send + Sync + 'static,
    OpenFn: Fn() + Clone + Send + Sync + 'static,
    CloseFn: Fn() + Clone + Send + Sync + 'static,
{
    /// Latest data received via the `EventSource`
    pub data: Signal<Option<T>>,

    /// The current state of the connection,
    pub ready_state: Signal<ConnectionReadyState>,

    /// The latest named event
    pub event: Signal<Option<UseEventSourceMessage<T>>>,

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
    #[error("Error event")]
    GenericErrorEvent,

    #[error("Custom error event: {0:?}")]
    CustomErrorEvent(UseEventSourceMessage<String>),

    #[error("Error decoding value")]
    Deserialize(Err),

    #[error("Error casting event '{0}' to MessageEvent")]
    CastToMessageEvent(String),
}
