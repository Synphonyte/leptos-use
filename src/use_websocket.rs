#![cfg_attr(feature = "ssr", allow(unused_variables, unused_imports, dead_code))]

use crate::{core::ConnectionReadyState, use_interval_fn, ReconnectLimit};
use cfg_if::cfg_if;
use codee::{CodecError, Decoder, Encoder, HybridCoderError, HybridDecoder, HybridEncoder};
use default_struct_builder::DefaultBuilder;
use js_sys::Array;
use leptos::{leptos_dom::helpers::TimeoutHandle, prelude::*};
use std::marker::PhantomData;
use std::sync::{atomic::AtomicBool, Arc};
use std::time::Duration;
use thiserror::Error;
use wasm_bindgen::prelude::*;
use web_sys::{BinaryType, CloseEvent, Event, MessageEvent, WebSocket};

#[allow(rustdoc::bare_urls)]
/// Creating and managing a [Websocket](https://developer.mozilla.org/en-US/docs/Web/API/WebSocket) connection.
///
/// ## Demo
///
/// [Link to Demo](https://github.com/Synphonyte/leptos-use/tree/main/examples/use_websocket)
///
/// ## Usage
///
/// Values are (en)decoded via the given codec. You can use any of the codecs, string or binary.
///
/// > Please check [the codec chapter](https://leptos-use.rs/codecs.html) to see what codecs are
/// > available and what feature flags they require.
///
/// ```
/// # use leptos::prelude::*;
/// # use codee::string::FromToStringCodec;
/// # use leptos_use::{use_websocket, UseWebSocketReturn};
/// # use leptos_use::core::ConnectionReadyState;
/// #
/// # #[component]
/// # fn Demo() -> impl IntoView {
/// let UseWebSocketReturn {
///     ready_state,
///     message,
///     send,
///     open,
///     close,
///     ..
/// } = use_websocket::<String, String, FromToStringCodec>("wss://echo.websocket.events/");
///
/// let send_message = move |_| {
///     send(&"Hello, world!".to_string());
/// };
///
/// let status = move || ready_state.get().to_string();
///
/// let connected = move || ready_state.get() == ConnectionReadyState::Open;
///
/// let open_connection = move |_| {
///     open();
/// };
///
/// let close_connection = move |_| {
///     close();
/// };
///
/// view! {
///     <div>
///         <p>"status: " {status}</p>
///
///         <button on:click=send_message disabled=move || !connected()>"Send"</button>
///         <button on:click=open_connection disabled=connected>"Open"</button>
///         <button on:click=close_connection disabled=move || !connected()>"Close"</button>
///
///         <p>"Receive message: " {move || format!("{:?}", message.get())}</p>
///     </div>
/// }
/// # }
/// ```
///
/// Here is another example using `msgpack` for encoding and decoding. This means that only binary
/// messages can be sent or received. For this to work you have to enable the **`msgpack_serde` feature** flag.
///
/// ```
/// # use leptos::*;
/// # use codee::binary::MsgpackSerdeCodec;
/// # use leptos_use::{use_websocket, UseWebSocketReturn};
/// # use serde::{Deserialize, Serialize};
/// #
/// # #[component]
/// # fn Demo() -> impl IntoView {
/// #[derive(Serialize, Deserialize)]
/// struct SomeData {
///     name: String,
///     count: i32,
/// }
///
/// let UseWebSocketReturn {
///     message,
///     send,
///     ..
/// } = use_websocket::<SomeData, SomeData, MsgpackSerdeCodec>("wss://some.websocket.server/");
///
/// let send_data = move || {
///     send(&SomeData {
///         name: "John Doe".to_string(),
///         count: 42,
///     });
/// };
/// #
/// # view! {}
/// }
/// ```
///
/// ### Heartbeats
///
/// Heartbeats can be configured by the `heartbeat` option. You have to provide a heartbeat
/// type, that implements the `Default` trait and an `Encoder` for it. This encoder doesn't have
/// to be the same as the one used for the other websocket messages.
///
/// ```
/// # use leptos::*;
/// # use codee::string::FromToStringCodec;
/// # use leptos_use::{use_websocket_with_options, UseWebSocketOptions, UseWebSocketReturn};
/// # use serde::{Deserialize, Serialize};
/// #
/// # #[component]
/// # fn Demo() -> impl IntoView {
/// #[derive(Default)]
/// struct Heartbeat;
///
/// // Simple example for usage with `FromToStringCodec`
/// impl std::fmt::Display for Heartbeat {
///     fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
///         write!(f, "<Heartbeat>")
///     }
/// }
///
/// let UseWebSocketReturn {
///     send,
///     message,
///     ..
/// } = use_websocket_with_options::<String, String, FromToStringCodec, _, _>(
///     "wss://echo.websocket.events/",
///     UseWebSocketOptions::default()
///         // Enable heartbeats every 10 seconds. In this case we use the same codec as for the
///         // other messages. But this is not necessary.
///         .heartbeat::<Heartbeat, FromToStringCodec>(10_000),
/// );
/// #
/// # view! {}
/// }
/// ```
///
/// ## Relative Paths
///
/// If the provided `url` is relative, it will be resolved relative to the current page.
/// Urls will be resolved like this the following. Please note that the protocol (http vs https) will
/// be taken into account as well.
///
/// | Current Page                   | Relative Url             | Resolved Url                        |
/// |--------------------------------|--------------------------|-------------------------------------|
/// | http://example.com/some/where  | /api/ws                  | ws://example.com/api/ws             |
/// | https://example.com/some/where | /api/ws                  | wss://example.com/api/ws            |
/// | https://example.com/some/where | api/ws                   | wss://example.com/some/where/api/ws |
/// | https://example.com/some/where | //otherdomain.com/api/ws | wss://otherdomain.com/api/ws        |
///
///
/// ## Usage with `provide_context`
///
/// The return value of `use_websocket` utilizes several type parameters which can make it
/// cumbersome to use with `provide_context` + `expect_context`.
/// The following example shows how to avoid type parameters with dynamic dispatch.
/// This sacrifices a little bit of performance for the sake of ergonomics. However,
/// compared to network transmission speeds this loss of performance is negligible.
///
/// First we define the `struct` that is going to be passed around as context.
///
/// ```
/// # use leptos::prelude::*;
/// use std::sync::Arc;
///
/// #[derive(Clone)]
/// pub struct WebsocketContext {
///     pub message: Signal<Option<String>>,
///     send: Arc<dyn Fn(&String)>,  // use Arc to make it easily cloneable
/// }
///
/// impl WebsocketContext {
///     pub fn new(message: Signal<Option<String>>, send: Arc<dyn Fn(&String)>) -> Self {
///         Self {
///             message,
///             send,
///         }
///     }
///
///     // create a method to avoid having to use parantheses around the field
///     #[inline(always)]
///     pub fn send(&self, message: &str) {
///         (self.send)(&message.to_string())
///     }
/// }
/// ```
///
/// Now you can provide the context like the following.
///
/// ```
/// # use leptos::prelude::*;
/// # use codee::string::FromToStringCodec;
/// # use leptos_use::{use_websocket, UseWebSocketReturn};
/// # use std::sync::Arc;
/// # #[derive(Clone)]
/// # pub struct WebsocketContext {
/// #     pub message: Signal<Option<String>>,
/// #     send: Arc<dyn Fn(&String) + Send + Sync>,
/// # }
/// #
/// # impl WebsocketContext {
/// #     pub fn new(message: Signal<Option<String>>, send: Arc<dyn Fn(&String) + Send + Sync>) -> Self {
/// #         Self {
/// #             message,
/// #             send,
/// #         }
/// #     }
/// # }
///
/// # #[component]
/// # fn Demo() -> impl IntoView {
/// let UseWebSocketReturn {
///     message,
///     send,
///     ..
/// } = use_websocket::<String, String, FromToStringCodec>("ws:://some.websocket.io");
///
/// provide_context(WebsocketContext::new(message, Arc::new(send.clone())));
/// #
/// # view! {}
/// # }
/// ```
///
/// Finally let's use the context:
///
/// ```
/// # use leptos::prelude::*;
/// # use leptos_use::{use_websocket, UseWebSocketReturn};
/// # use std::sync::Arc;
/// # #[derive(Clone)]
/// # pub struct WebsocketContext {
/// #     pub message: Signal<Option<String>>,
/// #     send: Arc<dyn Fn(&String)>,
/// # }
/// #
/// # impl WebsocketContext {
/// #     #[inline(always)]
/// #     pub fn send(&self, message: &str) {
/// #         (self.send)(&message.to_string())
/// #     }
/// # }
///
/// # #[component]
/// # fn Demo() -> impl IntoView {
/// let websocket = expect_context::<WebsocketContext>();
///
/// websocket.send("Hello World!");
/// #
/// # view! {}
/// # }
/// ```
///
/// ## Server-Side Rendering
///
/// > Make sure you follow the [instructions in Server-Side Rendering](https://leptos-use.rs/server_side_rendering.html).
///
/// On the server the returned functions amount to no-ops.
pub fn use_websocket<Tx, Rx, C>(
    url: &str,
) -> UseWebSocketReturn<
    Tx,
    Rx,
    impl Fn() + Clone + Send + Sync + 'static,
    impl Fn() + Clone + Send + Sync + 'static,
    impl Fn(&Tx) + Clone + Send + Sync + 'static,
>
where
    Tx: Send + Sync + 'static,
    Rx: Send + Sync + 'static,
    C: Encoder<Tx> + Decoder<Rx>,
    C: HybridEncoder<Tx, <C as Encoder<Tx>>::Encoded, Error = <C as Encoder<Tx>>::Error>,
    C: HybridDecoder<Rx, <C as Decoder<Rx>>::Encoded, Error = <C as Decoder<Rx>>::Error>,
{
    use_websocket_with_options::<Tx, Rx, C, (), DummyEncoder>(url, UseWebSocketOptions::default())
}

/// Version of [`use_websocket`] that takes `UseWebSocketOptions`. See [`use_websocket`] for how to use.
#[allow(clippy::type_complexity)]
pub fn use_websocket_with_options<Tx, Rx, C, Hb, HbCodec>(
    url: &str,
    options: UseWebSocketOptions<
        Rx,
        HybridCoderError<<C as Encoder<Tx>>::Error>,
        HybridCoderError<<C as Decoder<Rx>>::Error>,
        Hb,
        HbCodec,
    >,
) -> UseWebSocketReturn<
    Tx,
    Rx,
    impl Fn() + Clone + Send + Sync + 'static,
    impl Fn() + Clone + Send + Sync + 'static,
    impl Fn(&Tx) + Clone + Send + Sync + 'static,
>
where
    Tx: Send + Sync + 'static,
    Rx: Send + Sync + 'static,
    C: Encoder<Tx> + Decoder<Rx>,
    C: HybridEncoder<Tx, <C as Encoder<Tx>>::Encoded, Error = <C as Encoder<Tx>>::Error>,
    C: HybridDecoder<Rx, <C as Decoder<Rx>>::Encoded, Error = <C as Decoder<Rx>>::Error>,
    Hb: Default + Send + Sync + 'static,
    HbCodec: Encoder<Hb> + Send + Sync,
    HbCodec: HybridEncoder<
        Hb,
        <HbCodec as Encoder<Hb>>::Encoded,
        Error = <HbCodec as Encoder<Hb>>::Error,
    >,
    <HbCodec as Encoder<Hb>>::Error: std::fmt::Debug,
{
    let url = normalize_url(url);

    let UseWebSocketOptions {
        on_open,
        on_message,
        on_message_raw,
        on_message_raw_bytes,
        on_error,
        on_close,
        reconnect_limit,
        reconnect_interval,
        immediate,
        protocols,
        heartbeat,
    } = options;

    let (ready_state, set_ready_state) = signal(ConnectionReadyState::Closed);
    let (message, set_message) = signal(None);
    let ws_signal = RwSignal::new_local(None::<WebSocket>);

    let reconnect_timer_ref: StoredValue<Option<TimeoutHandle>> = StoredValue::new(None);

    let reconnect_times_ref: StoredValue<u64> = StoredValue::new(0);
    let manually_closed_ref: StoredValue<bool> = StoredValue::new(false);

    let unmounted = Arc::new(AtomicBool::new(false));

    let connect_ref: StoredValue<Option<Arc<dyn Fn() + Send + Sync>>> = StoredValue::new(None);

    let send_str = move |data: &str| {
        if ready_state.get_untracked() == ConnectionReadyState::Open {
            if let Some(web_socket) = ws_signal.get_untracked() {
                let _ = web_socket.send_with_str(data);
            }
        }
    };

    let send_bytes = move |data: &[u8]| {
        if ready_state.get_untracked() == ConnectionReadyState::Open {
            if let Some(web_socket) = ws_signal.get_untracked() {
                let _ = web_socket.send_with_u8_array(data);
            }
        }
    };

    let send = {
        let on_error = Arc::clone(&on_error);

        move |value: &Tx| {
            let on_error = Arc::clone(&on_error);

            send_with_codec::<Tx, C>(value, send_str, send_bytes, move |err| {
                on_error(UseWebSocketError::Codec(CodecError::Encode(err)));
            });
        }
    };

    let heartbeat_interval_ref = StoredValue::new_local(None::<(Arc<dyn Fn()>, Arc<dyn Fn()>)>);

    let stop_heartbeat = move || {
        if let Some((pause, _)) = heartbeat_interval_ref.get_value() {
            pause();
        }
    };

    #[cfg(not(feature = "ssr"))]
    {
        use crate::utils::Pausable;

        let start_heartbeat = {
            let on_error = Arc::clone(&on_error);

            move || {
                if let Some(heartbeat) = &heartbeat {
                    if let Some((pause, resume)) = heartbeat_interval_ref.get_value() {
                        pause();
                        resume();
                    } else {
                        let on_error = Arc::clone(&on_error);

                        let Pausable { pause, resume, .. } = use_interval_fn(
                            move || {
                                send_with_codec::<Hb, HbCodec>(
                                    &Hb::default(),
                                    send_str,
                                    send_bytes,
                                    {
                                        let on_error = Arc::clone(&on_error);

                                        move |err| {
                                            on_error(UseWebSocketError::HeartbeatCodec(format!(
                                                "Failed to encode heartbeat data: {err:?}"
                                            )))
                                        }
                                    },
                                )
                            },
                            heartbeat.interval,
                        );

                        heartbeat_interval_ref.set_value(Some((Arc::new(pause), Arc::new(resume))));
                    }
                }
            }
        };

        let reconnect_ref: StoredValue<Option<Arc<dyn Fn() + Send + Sync>>> =
            StoredValue::new(None);
        reconnect_ref.set_value({
            let unmounted = Arc::clone(&unmounted);

            Some(Arc::new(move || {
                let unmounted = Arc::clone(&unmounted);

                if !manually_closed_ref.get_value()
                    && !reconnect_limit.is_exceeded_by(reconnect_times_ref.get_value())
                    && ws_signal
                        .get_untracked()
                        .is_some_and(|ws: WebSocket| ws.ready_state() != WebSocket::OPEN)
                    && reconnect_timer_ref.get_value().is_none()
                {
                    reconnect_timer_ref.set_value(
                        set_timeout_with_handle(
                            move || {
                                if unmounted.load(std::sync::atomic::Ordering::Relaxed) {
                                    return;
                                }
                                if let Some(connect) = connect_ref.get_value() {
                                    connect();
                                    reconnect_times_ref.update_value(|current| *current += 1);
                                }
                            },
                            Duration::from_millis(reconnect_interval),
                        )
                        .ok(),
                    );
                }
            }))
        });

        connect_ref.set_value({
            let unmounted = Arc::clone(&unmounted);
            let on_error = Arc::clone(&on_error);

            Some(Arc::new(move || {
                if let Some(reconnect_timer) = reconnect_timer_ref.get_value() {
                    reconnect_timer.clear();
                    reconnect_timer_ref.set_value(None);
                }

                if let Some(web_socket) = ws_signal.get_untracked() {
                    let _ = web_socket.close();
                }

                let web_socket = {
                    protocols.with_untracked(|protocols| {
                        protocols.as_ref().map_or_else(
                            || WebSocket::new(&url).unwrap_throw(),
                            |protocols| {
                                let array = protocols
                                    .iter()
                                    .map(|p| JsValue::from(p.clone()))
                                    .collect::<Array>();
                                WebSocket::new_with_str_sequence(&url, &JsValue::from(&array))
                                    .unwrap_throw()
                            },
                        )
                    })
                };
                web_socket.set_binary_type(BinaryType::Arraybuffer);
                set_ready_state.set(ConnectionReadyState::Connecting);

                // onopen handler
                {
                    let unmounted = Arc::clone(&unmounted);
                    let on_open = Arc::clone(&on_open);

                    let onopen_closure = Closure::wrap(Box::new({
                        let start_heartbeat = start_heartbeat.clone();

                        move |e: Event| {
                            if unmounted.load(std::sync::atomic::Ordering::Relaxed) {
                                return;
                            }

                            #[cfg(debug_assertions)]
                            let zone = leptos::reactive::diagnostics::SpecialNonReactiveZone::enter();

                            on_open(e);

                            #[cfg(debug_assertions)]
                            drop(zone);

                            set_ready_state.set(ConnectionReadyState::Open);

                            start_heartbeat();
                        }
                    })
                        as Box<dyn FnMut(Event)>);
                    web_socket.set_onopen(Some(onopen_closure.as_ref().unchecked_ref()));
                    // Forget the closure to keep it alive
                    onopen_closure.forget();
                }

                // onmessage handler
                {
                    let unmounted = Arc::clone(&unmounted);
                    let on_message = Arc::clone(&on_message);
                    let on_message_raw = Arc::clone(&on_message_raw);
                    let on_message_raw_bytes = Arc::clone(&on_message_raw_bytes);
                    let on_error = Arc::clone(&on_error);

                    let onmessage_closure = Closure::wrap(Box::new(move |e: MessageEvent| {
                        if unmounted.load(std::sync::atomic::Ordering::Relaxed) {
                            return;
                        }

                        e.data().dyn_into::<js_sys::ArrayBuffer>().map_or_else(
                            |_| {
                                e.data().dyn_into::<js_sys::JsString>().map_or_else(
                                    |_| {
                                        unreachable!(
                                            "message event, received Unknown: {:?}",
                                            e.data()
                                        );
                                    },
                                    |txt| {
                                        let txt = String::from(&txt);

                                        #[cfg(debug_assertions)]
                                        let zone = leptos::reactive::diagnostics::SpecialNonReactiveZone::enter();

                                        on_message_raw(&txt);

                                        #[cfg(debug_assertions)]
                                        drop(zone);

                                        match C::decode_str(&txt) {
                                            Ok(val) => {
                                                #[cfg(debug_assertions)]
                                                let prev = leptos::reactive::diagnostics::SpecialNonReactiveZone::enter();

                                                on_message(&val);

                                                #[cfg(debug_assertions)]
                                                drop(prev);

                                                set_message.set(Some(val));
                                            }
                                            Err(err) => {
                                                on_error(CodecError::Decode(err).into());
                                            }
                                        }
                                    },
                                );
                            },
                            |array_buffer| {
                                let array = js_sys::Uint8Array::new(&array_buffer);
                                let array = array.to_vec();

                                #[cfg(debug_assertions)]
                                let zone = leptos::reactive::diagnostics::SpecialNonReactiveZone::enter();

                                on_message_raw_bytes(&array);

                                #[cfg(debug_assertions)]
                                drop(zone);

                                match C::decode_bin(array.as_slice()) {
                                    Ok(val) => {
                                        #[cfg(debug_assertions)]
                                        let prev = leptos::reactive::diagnostics::SpecialNonReactiveZone::enter();

                                        on_message(&val);

                                        #[cfg(debug_assertions)]
                                        drop(prev);

                                        set_message.set(Some(val));
                                    }
                                    Err(err) => {
                                        on_error(CodecError::Decode(err).into());
                                    }
                                }
                            },
                        );
                    })
                        as Box<dyn FnMut(MessageEvent)>);
                    web_socket.set_onmessage(Some(onmessage_closure.as_ref().unchecked_ref()));
                    onmessage_closure.forget();
                }

                // onerror handler
                {
                    let unmounted = Arc::clone(&unmounted);
                    let on_error = Arc::clone(&on_error);

                    let onerror_closure = Closure::wrap(Box::new(move |e: Event| {
                        if unmounted.load(std::sync::atomic::Ordering::Relaxed) {
                            return;
                        }

                        stop_heartbeat();

                        if let Some(reconnect) = &reconnect_ref.get_value() {
                            reconnect();
                        }

                        #[cfg(debug_assertions)]
                        let zone = leptos::reactive::diagnostics::SpecialNonReactiveZone::enter();

                        on_error(UseWebSocketError::Event(e));

                        #[cfg(debug_assertions)]
                        drop(zone);

                        set_ready_state.set(ConnectionReadyState::Closed);
                    })
                        as Box<dyn FnMut(Event)>);
                    web_socket.set_onerror(Some(onerror_closure.as_ref().unchecked_ref()));
                    onerror_closure.forget();
                }

                // onclose handler
                {
                    let unmounted = Arc::clone(&unmounted);
                    let on_close = Arc::clone(&on_close);

                    let onclose_closure = Closure::wrap(Box::new(move |e: CloseEvent| {
                        if unmounted.load(std::sync::atomic::Ordering::Relaxed) {
                            return;
                        }

                        stop_heartbeat();

                        if let Some(reconnect) = &reconnect_ref.get_value() {
                            reconnect();
                        }

                        #[cfg(debug_assertions)]
                        let zone = leptos::reactive::diagnostics::SpecialNonReactiveZone::enter();

                        on_close(e);

                        #[cfg(debug_assertions)]
                        drop(zone);

                        set_ready_state.set(ConnectionReadyState::Closed);
                    })
                        as Box<dyn FnMut(CloseEvent)>);
                    web_socket.set_onclose(Some(onclose_closure.as_ref().unchecked_ref()));
                    onclose_closure.forget();
                }

                ws_signal.set(Some(web_socket));
            }))
        });
    }

    // Open connection
    let open = move || {
        reconnect_times_ref.set_value(0);
        if let Some(connect) = connect_ref.get_value() {
            connect();
        }
    };

    // Close connection
    let close = {
        reconnect_timer_ref.set_value(None);

        move || {
            stop_heartbeat();
            manually_closed_ref.set_value(true);
            if let Some(web_socket) = ws_signal.get_untracked() {
                let _ = web_socket.close();
            }
        }
    };

    // Open connection (not called if option `manual` is true)
    Effect::new(move |_| {
        if immediate {
            open();
        }
    });

    // clean up (unmount)
    on_cleanup(move || {
        unmounted.store(true, std::sync::atomic::Ordering::Relaxed);
        close();
    });

    UseWebSocketReturn {
        ready_state: ready_state.into(),
        message: message.into(),
        ws: ws_signal.into(),
        open,
        close,
        send,
        _marker: PhantomData,
    }
}

fn send_with_codec<T, Codec>(
    value: &T,
    send_str: impl Fn(&str),
    send_bytes: impl Fn(&[u8]),
    on_error: impl Fn(HybridCoderError<<Codec as Encoder<T>>::Error>),
) where
    Codec: Encoder<T>,
    Codec: HybridEncoder<T, <Codec as Encoder<T>>::Encoded, Error = <Codec as Encoder<T>>::Error>,
{
    if Codec::is_binary_encoder() {
        match Codec::encode_bin(value) {
            Ok(val) => send_bytes(&val),
            Err(err) => on_error(err),
        }
    } else {
        match Codec::encode_str(value) {
            Ok(val) => send_str(&val),
            Err(err) => on_error(err),
        }
    }
}

type ArcFnBytes = Arc<dyn Fn(&[u8]) + Send + Sync>;

/// Options for [`use_websocket_with_options`].
#[derive(DefaultBuilder)]
pub struct UseWebSocketOptions<Rx, E, D, Hb, HbCodec>
where
    Rx: ?Sized,
    Hb: Default + Send + Sync + 'static,
    HbCodec: Encoder<Hb>,
    HbCodec: HybridEncoder<
        Hb,
        <HbCodec as Encoder<Hb>>::Encoded,
        Error = <HbCodec as Encoder<Hb>>::Error,
    >,
{
    /// Heartbeat options
    #[builder(skip)]
    heartbeat: Option<HeartbeatOptions<Hb, HbCodec>>,
    /// `WebSocket` connect callback.
    on_open: Arc<dyn Fn(Event) + Send + Sync>,
    /// `WebSocket` message callback for typed message decoded by codec.
    #[builder(skip)]
    on_message: Arc<dyn Fn(&Rx) + Send + Sync>,
    /// `WebSocket` message callback for text.
    on_message_raw: Arc<dyn Fn(&str) + Send + Sync>,
    /// `WebSocket` message callback for binary.
    on_message_raw_bytes: ArcFnBytes,
    /// `WebSocket` error callback.
    #[builder(skip)]
    on_error: Arc<dyn Fn(UseWebSocketError<E, D>) + Send + Sync>,
    /// `WebSocket` close callback.
    on_close: Arc<dyn Fn(CloseEvent) + Send + Sync>,
    /// Retry times. Defaults to `ReconnectLimit::Limited(3)`. Use `ReconnectLimit::Infinite` for
    /// infinite retries.
    reconnect_limit: ReconnectLimit,
    /// Retry interval in ms. Defaults to 3000.
    reconnect_interval: u64,
    /// If `true` the `WebSocket` connection will immediately be opened when calling this function.
    /// If `false` you have to manually call the `open` function.
    /// Defaults to `true`.
    immediate: bool,
    /// Sub protocols. See [MDN Docs](https://developer.mozilla.org/en-US/docs/Web/API/WebSocket/WebSocket#protocols).
    ///
    /// Can be set as a signal to support protocols only available after the initial render.
    ///
    /// Note that protocols are only updated on the next websocket open() call, not whenever the signal is updated.
    /// Therefore "lazy" protocols should use the `immediate(false)` option and manually call `open()`.
    #[builder(into)]
    protocols: Signal<Option<Vec<String>>>,
}

impl<Rx: ?Sized, E, D, Hb, HbCodec> UseWebSocketOptions<Rx, E, D, Hb, HbCodec>
where
    Hb: Default + Send + Sync + 'static,
    HbCodec: Encoder<Hb>,
    HbCodec: HybridEncoder<
        Hb,
        <HbCodec as Encoder<Hb>>::Encoded,
        Error = <HbCodec as Encoder<Hb>>::Error,
    >,
{
    /// `WebSocket` error callback.
    pub fn on_error<F>(self, handler: F) -> Self
    where
        F: Fn(UseWebSocketError<E, D>) + Send + Sync + 'static,
    {
        Self {
            on_error: Arc::new(handler),
            ..self
        }
    }

    /// `WebSocket` message callback for typed message decoded by codec.
    pub fn on_message<F>(self, handler: F) -> Self
    where
        F: Fn(&Rx) + Send + Sync + 'static,
    {
        Self {
            on_message: Arc::new(handler),
            ..self
        }
    }

    /// Set the data, codec and interval at which the heartbeat is sent. The heartbeat
    /// is the default value of the `NewHb` type.
    pub fn heartbeat<NewHb, NewHbCodec>(
        self,
        interval: u64,
    ) -> UseWebSocketOptions<Rx, E, D, NewHb, NewHbCodec>
    where
        NewHb: Default + Send + Sync + 'static,
        NewHbCodec: Encoder<NewHb>,
        NewHbCodec: HybridEncoder<
            NewHb,
            <NewHbCodec as Encoder<NewHb>>::Encoded,
            Error = <NewHbCodec as Encoder<NewHb>>::Error,
        >,
    {
        UseWebSocketOptions {
            heartbeat: Some(HeartbeatOptions {
                data: PhantomData::<NewHb>,
                interval,
                codec: PhantomData::<NewHbCodec>,
            }),
            on_open: self.on_open,
            on_message: self.on_message,
            on_message_raw: self.on_message_raw,
            on_message_raw_bytes: self.on_message_raw_bytes,
            on_close: self.on_close,
            on_error: self.on_error,
            reconnect_limit: self.reconnect_limit,
            reconnect_interval: self.reconnect_interval,
            immediate: self.immediate,
            protocols: self.protocols,
        }
    }
}

impl<Rx: ?Sized, E, D> Default for UseWebSocketOptions<Rx, E, D, (), DummyEncoder> {
    fn default() -> Self {
        Self {
            heartbeat: None,
            on_open: Arc::new(|_| {}),
            on_message: Arc::new(|_| {}),
            on_message_raw: Arc::new(|_| {}),
            on_message_raw_bytes: Arc::new(|_| {}),
            on_error: Arc::new(|_| {}),
            on_close: Arc::new(|_| {}),
            reconnect_limit: ReconnectLimit::default(),
            reconnect_interval: 3000,
            immediate: true,
            protocols: Default::default(),
        }
    }
}

pub struct DummyEncoder;

impl Encoder<()> for DummyEncoder {
    type Encoded = String;
    type Error = ();

    fn encode(_: &()) -> Result<Self::Encoded, Self::Error> {
        Ok("".to_string())
    }
}

/// Options for heartbeats
pub struct HeartbeatOptions<Hb, HbCodec>
where
    Hb: Default + Send + Sync + 'static,
    HbCodec: Encoder<Hb>,
    HbCodec: HybridEncoder<
        Hb,
        <HbCodec as Encoder<Hb>>::Encoded,
        Error = <HbCodec as Encoder<Hb>>::Error,
    >,
{
    /// Heartbeat data that will be sent to the server
    data: PhantomData<Hb>,
    /// Heartbeat interval in ms. A heartbeat will be sent every `interval` ms.
    interval: u64,
    /// Codec used to encode the heartbeat data
    codec: PhantomData<HbCodec>,
}

impl<Hb, HbCodec> Clone for HeartbeatOptions<Hb, HbCodec>
where
    Hb: Default + Send + Sync + 'static,
    HbCodec: Encoder<Hb>,
    HbCodec: HybridEncoder<
        Hb,
        <HbCodec as Encoder<Hb>>::Encoded,
        Error = <HbCodec as Encoder<Hb>>::Error,
    >,
{
    fn clone(&self) -> Self {
        *self
    }
}

impl<Hb, HbCodec> Copy for HeartbeatOptions<Hb, HbCodec>
where
    Hb: Default + Send + Sync + 'static,
    HbCodec: Encoder<Hb>,
    HbCodec: HybridEncoder<
        Hb,
        <HbCodec as Encoder<Hb>>::Encoded,
        Error = <HbCodec as Encoder<Hb>>::Error,
    >,
{
}

/// Return type of [`use_websocket`].
#[derive(Clone)]
pub struct UseWebSocketReturn<Tx, Rx, OpenFn, CloseFn, SendFn>
where
    Tx: Send + Sync + 'static,
    Rx: Send + Sync + 'static,
    OpenFn: Fn() + Clone + Send + Sync + 'static,
    CloseFn: Fn() + Clone + Send + Sync + 'static,
    SendFn: Fn(&Tx) + Clone + Send + Sync + 'static,
{
    /// The current state of the `WebSocket` connection.
    pub ready_state: Signal<ConnectionReadyState>,
    /// Latest message received from `WebSocket`.
    pub message: Signal<Option<Rx>>,
    /// The `WebSocket` instance.
    pub ws: Signal<Option<WebSocket>, LocalStorage>,
    /// Opens the `WebSocket` connection
    pub open: OpenFn,
    /// Closes the `WebSocket` connection
    pub close: CloseFn,
    /// Sends data through the socket
    pub send: SendFn,

    _marker: PhantomData<Tx>,
}

#[derive(Error, Debug)]
pub enum UseWebSocketError<E, D> {
    #[error("WebSocket error event")]
    Event(Event),
    #[error("WebSocket codec error: {0}")]
    Codec(#[from] CodecError<E, D>),
    #[error("WebSocket heartbeat codec error: {0}")]
    HeartbeatCodec(String),
}

fn normalize_url(url: &str) -> String {
    cfg_if! { if #[cfg(feature = "ssr")] {
        url.to_string()
    } else {
        if url.starts_with("ws://") || url.starts_with("wss://") {
            url.to_string()
        } else if url.starts_with("//") {
            format!("{}{}", detect_protocol(), url)
        } else if url.starts_with('/') {
            format!(
                "{}//{}{}",
                detect_protocol(),
                window().location().host().expect("Host not found"),
                url
            )
        } else {
            let mut path = window().location().pathname().expect("Pathname not found");
            if !path.ends_with('/') {
                path.push('/')
            }
            format!(
                "{}//{}{}{}",
                detect_protocol(),
                window().location().host().expect("Host not found"),
                path,
                url
            )
        }
    }}
}

fn detect_protocol() -> String {
    cfg_if! { if #[cfg(feature = "ssr")] {
        "ws".to_string()
    } else {
        window().location().protocol().expect("Protocol not found").replace("http", "ws")
    }}
}
