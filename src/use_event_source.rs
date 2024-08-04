use crate::core::ConnectionReadyState;
use crate::{js, use_event_listener, ReconnectLimit};
use codee::Decoder;
use default_struct_builder::DefaultBuilder;
use leptos::prelude::diagnostics::SpecialNonReactiveZone;
use leptos::prelude::*;
use std::marker::PhantomData;
use std::sync::atomic::{AtomicBool, AtomicU32};
use std::sync::Arc;
use std::time::Duration;
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
/// binary codec wrapped in [`Base64`].
///
/// > Please check [the codec chapter](https://leptos-use.rs/codecs.html) to see what codecs are
///   available and what feature flags they require.
///
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
/// ### Create Your Own Custom Codec
///
/// All you need to do is to implement the [`StringCodec`] trait together with `Default` and `Clone`.
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
/// To disable auto-reconnection, set `reconnect_limit` to `0`.
///
/// ## Server-Side Rendering
///
/// On the server-side, `use_event_source` will always return `ready_state` as `ConnectionReadyState::Closed`,
/// `data`, `event` and `error` will always be `None`, and `open` and `close` will do nothing.
pub fn use_event_source<T, C>(
    url: &str,
) -> UseEventSourceReturn<T, C::Error, impl Fn() + Clone + 'static, impl Fn() + Clone + 'static>
where
    T: Clone + PartialEq + Send + Sync + 'static,
    C: Decoder<T, Encoded = str>,
    C::Error: Send + Sync,
{
    use_event_source_with_options::<T, C>(url, UseEventSourceOptions::<T>::default())
}

/// Version of [`use_event_source`] that takes a `UseEventSourceOptions`. See [`use_event_source`] for how to use.
pub fn use_event_source_with_options<T, C>(
    url: &str,
    options: UseEventSourceOptions<T>,
) -> UseEventSourceReturn<T, C::Error, impl Fn() + Clone + 'static, impl Fn() + Clone + 'static>
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

    let url = url.to_owned();

    let (event, set_event) = signal_local(None::<web_sys::Event>);
    let (data, set_data) = signal(None::<T>);
    let (ready_state, set_ready_state) = signal(ConnectionReadyState::Closed);
    let (event_source, set_event_source) = signal_local(None::<web_sys::EventSource>);
    let (error, set_error) = signal_local(None::<UseEventSourceError<C::Error>>);

    let explicitly_closed = Arc::new(AtomicBool::new(false));
    let retried = Arc::new(AtomicU32::new(0));

    let set_data_from_string = move |data_string: Option<String>| {
        if let Some(data_string) = data_string {
            match C::decode(&data_string) {
                Ok(data) => set_data.set(Some(data)),
                Err(err) => set_error.set(Some(UseEventSourceError::Deserialize(err))),
            }
        }
    };

    let close = {
        let explicitly_closed = Arc::clone(&explicitly_closed);

        move || {
            if let Some(event_source) = event_source.get_untracked() {
                event_source.close();
                set_event_source.set(None);
                set_ready_state.set(ConnectionReadyState::Closed);
                explicitly_closed.store(true, std::sync::atomic::Ordering::Relaxed);
            }
        }
    };

    let init = StoredValue::new(None::<Arc<dyn Fn() + Send + Sync>>);

    init.set_value(Some(Arc::new({
        let explicitly_closed = Arc::clone(&explicitly_closed);
        let retried = Arc::clone(&retried);

        move || {
            use wasm_bindgen::prelude::*;

            if explicitly_closed.load(std::sync::atomic::Ordering::Relaxed) {
                return;
            }

            let mut event_src_opts = web_sys::EventSourceInit::new();
            event_src_opts.with_credentials(with_credentials);

            let es = web_sys::EventSource::new_with_event_source_init_dict(&url, &event_src_opts)
                .unwrap_throw();

            set_ready_state.set(ConnectionReadyState::Connecting);

            set_event_source.set(Some(es.clone()));

            let on_open = Closure::wrap(Box::new(move |_: web_sys::Event| {
                set_ready_state.set(ConnectionReadyState::Open);
                set_error.set(None);
            }) as Box<dyn FnMut(web_sys::Event)>);
            es.set_onopen(Some(on_open.as_ref().unchecked_ref()));
            on_open.forget();

            let on_error = Closure::wrap(Box::new({
                let explicitly_closed = Arc::clone(&explicitly_closed);
                let retried = Arc::clone(&retried);
                let on_failed = Arc::clone(&on_failed);
                let es = es.clone();

                move |e: web_sys::Event| {
                    set_ready_state.set(ConnectionReadyState::Closed);
                    set_error.set(Some(UseEventSourceError::Event(e)));

                    // only reconnect if EventSource isn't reconnecting by itself
                    // this is the case when the connection is closed (readyState is 2)
                    if es.ready_state() == 2
                        && !explicitly_closed.load(std::sync::atomic::Ordering::Relaxed)
                        && matches!(reconnect_limit, ReconnectLimit::Limited(_))
                    {
                        es.close();

                        let retried_value =
                            retried.fetch_add(1, std::sync::atomic::Ordering::Relaxed) + 1;

                        if reconnect_limit.is_exceeded_by(retried_value as u64) {
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
                            let _z = SpecialNonReactiveZone::enter();

                            on_failed();
                        }
                    }
                }
            }) as Box<dyn FnMut(web_sys::Event)>);
            es.set_onerror(Some(on_error.as_ref().unchecked_ref()));
            on_error.forget();

            let on_message = Closure::wrap(Box::new(move |e: web_sys::MessageEvent| {
                set_data_from_string(e.data().as_string());
            }) as Box<dyn FnMut(web_sys::MessageEvent)>);
            es.set_onmessage(Some(on_message.as_ref().unchecked_ref()));
            on_message.forget();

            for event_name in named_events.clone() {
                let _ = use_event_listener(
                    es.clone(),
                    leptos::ev::Custom::<leptos::ev::Event>::new(event_name),
                    move |e| {
                        set_event.set(Some(e.clone()));
                        let data_string = js!(e["data"]).ok().and_then(|d| d.as_string());
                        set_data_from_string(data_string);
                    },
                );
            }
        }
    })));

    let open;

    #[cfg(not(feature = "ssr"))]
    {
        open = {
            let close = close.clone();
            let explicitly_closed = Arc::clone(&explicitly_closed);
            let retried = Arc::clone(&retried);

            move || {
                close();
                explicitly_closed.store(false, std::sync::atomic::Ordering::Relaxed);
                retried.store(0, std::sync::atomic::Ordering::Relaxed);
                if let Some(init) = init.get_value() {
                    init();
                }
            }
        };
    }

    #[cfg(feature = "ssr")]
    {
        open = move || {};
    }

    if immediate {
        open();
    }

    on_cleanup(close.clone());

    UseEventSourceReturn {
        event_source: event_source.into(),
        event: event.into(),
        data: data.into(),
        ready_state: ready_state.into(),
        error: error.into(),
        open,
        close,
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
    OpenFn: Fn() + Clone + 'static,
    CloseFn: Fn() + Clone + 'static,
{
    /// Latest data received via the `EventSource`
    pub data: Signal<Option<T>>,

    /// The current state of the connection,
    pub ready_state: Signal<ConnectionReadyState>,

    /// The latest named event
    pub event: Signal<Option<web_sys::Event>, LocalStorage>,

    /// The current error
    pub error: Signal<Option<UseEventSourceError<Err>>, LocalStorage>,

    /// (Re-)Opens the `EventSource` connection
    /// If the current one is active, will close it before opening a new one.
    pub open: OpenFn,

    /// Closes the `EventSource` connection
    pub close: CloseFn,

    /// The `EventSource` instance
    pub event_source: Signal<Option<web_sys::EventSource>, LocalStorage>,
}

#[derive(Error, Debug)]
pub enum UseEventSourceError<Err> {
    #[error("Error event: {0:?}")]
    Event(web_sys::Event),

    #[error("Error decoding value")]
    Deserialize(Err),
}
