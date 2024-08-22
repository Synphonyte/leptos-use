#![cfg_attr(feature = "ssr", allow(unused_variables, unused_imports, dead_code))]

use cfg_if::cfg_if;
use leptos::{leptos_dom::helpers::TimeoutHandle, *};
use std::cell::Cell;
use std::rc::Rc;
use std::time::Duration;
use thiserror::Error;

use crate::{core::ConnectionReadyState, ReconnectLimit};
use codee::{
    CodecError, Decoder, Encoder, HybridCoderError, HybridDecoder, HybridEncoder, IsBinary,
};
use default_struct_builder::DefaultBuilder;
use js_sys::Array;
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
/// # use leptos::*;
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
/// } = use_websocket::<String, FromToStringCodec>("wss://echo.websocket.events/");
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
/// } = use_websocket::<SomeData, MsgpackSerdeCodec>("wss://some.websocket.server/");
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
/// # use leptos::*;
/// use std::rc::Rc;
///
/// #[derive(Clone)]
/// pub struct WebsocketContext {
///     pub message: Signal<Option<String>>,
///     send: Rc<dyn Fn(&String)>,  // use Rc to make it easily cloneable
/// }
///
/// impl WebsocketContext {
///     pub fn new(message: Signal<Option<String>>, send: Rc<dyn Fn(&String)>) -> Self {
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
/// # use leptos::*;
/// # use codee::string::FromToStringCodec;
/// # use leptos_use::{use_websocket, UseWebSocketReturn};
/// # use std::rc::Rc;
/// # #[derive(Clone)]
/// # pub struct WebsocketContext {
/// #     pub message: Signal<Option<String>>,
/// #     send: Rc<dyn Fn(&String)>,
/// # }
/// #
/// # impl WebsocketContext {
/// #     pub fn new(message: Signal<Option<String>>, send: Rc<dyn Fn(&String)>) -> Self {
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
/// } = use_websocket::<String, FromToStringCodec>("ws:://some.websocket.io");
///
/// provide_context(WebsocketContext::new(message, Rc::new(send.clone())));
/// #
/// # view! {}
/// # }
/// ```
///
/// Finally let's use the context:
///
/// ```
/// # use leptos::*;
/// # use leptos_use::{use_websocket, UseWebSocketReturn};
/// # use std::rc::Rc;
/// # #[derive(Clone)]
/// # pub struct WebsocketContext {
/// #     pub message: Signal<Option<String>>,
/// #     send: Rc<dyn Fn(&String)>,
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
/// On the server the returned functions amount to no-ops.
pub fn use_websocket<T, C>(
    url: &str,
) -> UseWebSocketReturn<
    T,
    impl Fn() + Clone + 'static,
    impl Fn() + Clone + 'static,
    impl Fn(&T) + Clone + 'static,
>
where
    T: 'static,
    C: Encoder<T> + Decoder<T>,
    C: IsBinary<T, <C as Decoder<T>>::Encoded>,
    C: HybridDecoder<T, <C as Decoder<T>>::Encoded, Error = <C as Decoder<T>>::Error>,
    C: HybridEncoder<T, <C as Encoder<T>>::Encoded, Error = <C as Encoder<T>>::Error>,
{
    use_websocket_with_options::<T, C>(url, UseWebSocketOptions::default())
}

/// Version of [`use_websocket`] that takes `UseWebSocketOptions`. See [`use_websocket`] for how to use.
#[allow(clippy::type_complexity)]
pub fn use_websocket_with_options<T, C>(
    url: &str,
    options: UseWebSocketOptions<
        T,
        HybridCoderError<<C as Encoder<T>>::Error>,
        HybridCoderError<<C as Decoder<T>>::Error>,
    >,
) -> UseWebSocketReturn<
    T,
    impl Fn() + Clone + 'static,
    impl Fn() + Clone + 'static,
    impl Fn(&T) + Clone + 'static,
>
where
    T: 'static,
    C: Encoder<T> + Decoder<T>,
    C: IsBinary<T, <C as Decoder<T>>::Encoded>,
    C: HybridDecoder<T, <C as Decoder<T>>::Encoded, Error = <C as Decoder<T>>::Error>,
    C: HybridEncoder<T, <C as Encoder<T>>::Encoded, Error = <C as Encoder<T>>::Error>,
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
    } = options;

    let (ready_state, set_ready_state) = create_signal(ConnectionReadyState::Closed);
    let (message, set_message) = create_signal(None);
    let ws_ref: StoredValue<Option<WebSocket>> = store_value(None);

    let reconnect_timer_ref: StoredValue<Option<TimeoutHandle>> = store_value(None);

    let reconnect_times_ref: StoredValue<u64> = store_value(0);
    let manually_closed_ref: StoredValue<bool> = store_value(false);

    let unmounted = Rc::new(Cell::new(false));

    let connect_ref: StoredValue<Option<Rc<dyn Fn()>>> = store_value(None);

    #[cfg(not(feature = "ssr"))]
    {
        let reconnect_ref: StoredValue<Option<Rc<dyn Fn()>>> = store_value(None);
        reconnect_ref.set_value({
            Some(Rc::new(move || {
                if !manually_closed_ref.get_value()
                    && !reconnect_limit.is_exceeded_by(reconnect_times_ref.get_value())
                    && ws_ref
                        .get_value()
                        .map_or(false, |ws: WebSocket| ws.ready_state() != WebSocket::OPEN)
                {
                    reconnect_timer_ref.set_value(
                        set_timeout_with_handle(
                            move || {
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
            let unmounted = Rc::clone(&unmounted);
            let on_error = Rc::clone(&on_error);

            Some(Rc::new(move || {
                reconnect_timer_ref.set_value(None);

                if let Some(web_socket) = ws_ref.get_value() {
                    let _ = web_socket.close();
                }

                let web_socket = {
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
                };
                web_socket.set_binary_type(BinaryType::Arraybuffer);
                set_ready_state.set(ConnectionReadyState::Connecting);

                // onopen handler
                {
                    let unmounted = Rc::clone(&unmounted);
                    let on_open = Rc::clone(&on_open);

                    let onopen_closure = Closure::wrap(Box::new(move |e: Event| {
                        if unmounted.get() {
                            return;
                        }

                        #[cfg(debug_assertions)]
                        let prev = SpecialNonReactiveZone::enter();

                        on_open(e);

                        #[cfg(debug_assertions)]
                        SpecialNonReactiveZone::exit(prev);

                        set_ready_state.set(ConnectionReadyState::Open);
                    })
                        as Box<dyn FnMut(Event)>);
                    web_socket.set_onopen(Some(onopen_closure.as_ref().unchecked_ref()));
                    // Forget the closure to keep it alive
                    onopen_closure.forget();
                }

                // onmessage handler
                {
                    let unmounted = Rc::clone(&unmounted);
                    let on_message = Rc::clone(&on_message);
                    let on_message_raw = Rc::clone(&on_message_raw);
                    let on_message_raw_bytes = Rc::clone(&on_message_raw_bytes);
                    let on_error = Rc::clone(&on_error);

                    let onmessage_closure = Closure::wrap(Box::new(move |e: MessageEvent| {
                        if unmounted.get() {
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
                                        let prev = SpecialNonReactiveZone::enter();

                                        on_message_raw(&txt);

                                        #[cfg(debug_assertions)]
                                        SpecialNonReactiveZone::exit(prev);

                                        match C::decode_str(&txt) {
                                            Ok(val) => {
                                                #[cfg(debug_assertions)]
                                                let prev = SpecialNonReactiveZone::enter();

                                                on_message(&val);

                                                #[cfg(debug_assertions)]
                                                SpecialNonReactiveZone::exit(prev);

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
                                let prev = SpecialNonReactiveZone::enter();

                                on_message_raw_bytes(&array);

                                #[cfg(debug_assertions)]
                                SpecialNonReactiveZone::exit(prev);

                                match C::decode_bin(array.as_slice()) {
                                    Ok(val) => {
                                        #[cfg(debug_assertions)]
                                        let prev = SpecialNonReactiveZone::enter();

                                        on_message(&val);

                                        #[cfg(debug_assertions)]
                                        SpecialNonReactiveZone::exit(prev);

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
                    let unmounted = Rc::clone(&unmounted);
                    let on_error = Rc::clone(&on_error);

                    let onerror_closure = Closure::wrap(Box::new(move |e: Event| {
                        if unmounted.get() {
                            return;
                        }

                        if let Some(reconnect) = &reconnect_ref.get_value() {
                            reconnect();
                        }

                        #[cfg(debug_assertions)]
                        let prev = SpecialNonReactiveZone::enter();

                        on_error(UseWebSocketError::Event(e));

                        #[cfg(debug_assertions)]
                        SpecialNonReactiveZone::exit(prev);

                        set_ready_state.set(ConnectionReadyState::Closed);
                    })
                        as Box<dyn FnMut(Event)>);
                    web_socket.set_onerror(Some(onerror_closure.as_ref().unchecked_ref()));
                    onerror_closure.forget();
                }

                // onclose handler
                {
                    let unmounted = Rc::clone(&unmounted);
                    let on_close = Rc::clone(&on_close);

                    let onclose_closure = Closure::wrap(Box::new(move |e: CloseEvent| {
                        if unmounted.get() {
                            return;
                        }

                        if let Some(reconnect) = &reconnect_ref.get_value() {
                            reconnect();
                        }

                        #[cfg(debug_assertions)]
                        let prev = SpecialNonReactiveZone::enter();

                        on_close(e);

                        #[cfg(debug_assertions)]
                        SpecialNonReactiveZone::exit(prev);

                        set_ready_state.set(ConnectionReadyState::Closed);
                    })
                        as Box<dyn FnMut(CloseEvent)>);
                    web_socket.set_onclose(Some(onclose_closure.as_ref().unchecked_ref()));
                    onclose_closure.forget();
                }

                ws_ref.set_value(Some(web_socket));
            }))
        });
    }

    // Send text (String)
    let send_str = {
        Box::new(move |data: &str| {
            if ready_state.get_untracked() == ConnectionReadyState::Open {
                if let Some(web_socket) = ws_ref.get_value() {
                    let _ = web_socket.send_with_str(data);
                }
            }
        })
    };

    // Send bytes
    let send_bytes = move |data: &[u8]| {
        if ready_state.get_untracked() == ConnectionReadyState::Open {
            if let Some(web_socket) = ws_ref.get_value() {
                let _ = web_socket.send_with_u8_array(data);
            }
        }
    };

    let send = {
        let on_error = Rc::clone(&on_error);

        move |value: &T| {
            if C::is_binary() {
                match C::encode_bin(value) {
                    Ok(val) => send_bytes(&val),
                    Err(err) => on_error(CodecError::Encode(err).into()),
                }
            } else {
                match C::encode_str(value) {
                    Ok(val) => send_str(&val),
                    Err(err) => on_error(CodecError::Encode(err).into()),
                }
            }
        }
    };

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
            manually_closed_ref.set_value(true);
            if let Some(web_socket) = ws_ref.get_value() {
                let _ = web_socket.close();
            }
        }
    };

    // Open connection (not called if option `manual` is true)
    create_effect(move |_| {
        if immediate {
            open();
        }
    });

    // clean up (unmount)
    on_cleanup(move || {
        unmounted.set(true);
        close();
    });

    UseWebSocketReturn {
        ready_state: ready_state.into(),
        message: message.into(),
        ws: ws_ref.get_value(),
        open,
        close,
        send,
    }
}

type RcFnBytes = Rc<dyn Fn(&[u8])>;

/// Options for [`use_websocket_with_options`].
#[derive(DefaultBuilder)]
pub struct UseWebSocketOptions<T, E, D>
where
    T: ?Sized,
{
    /// `WebSocket` connect callback.
    on_open: Rc<dyn Fn(Event)>,
    /// `WebSocket` message callback for typed message decoded by codec.
    #[builder(skip)]
    on_message: Rc<dyn Fn(&T)>,
    /// `WebSocket` message callback for text.
    on_message_raw: Rc<dyn Fn(&str)>,
    /// `WebSocket` message callback for binary.
    on_message_raw_bytes: RcFnBytes,
    /// `WebSocket` error callback.
    #[builder(skip)]
    on_error: Rc<dyn Fn(UseWebSocketError<E, D>)>,
    /// `WebSocket` close callback.
    on_close: Rc<dyn Fn(CloseEvent)>,
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
    protocols: Option<Vec<String>>,
}

impl<T: ?Sized, E, D> UseWebSocketOptions<T, E, D> {
    /// `WebSocket` error callback.
    pub fn on_error<F>(self, handler: F) -> Self
    where
        F: Fn(UseWebSocketError<E, D>) + 'static,
    {
        Self {
            on_error: Rc::new(handler),
            ..self
        }
    }

    /// `WebSocket` message callback for typed message decoded by codec.
    pub fn on_message<F>(self, handler: F) -> Self
    where
        F: Fn(&T) + 'static,
    {
        Self {
            on_message: Rc::new(handler),
            ..self
        }
    }
}

impl<T: ?Sized, E, D> Default for UseWebSocketOptions<T, E, D> {
    fn default() -> Self {
        Self {
            on_open: Rc::new(|_| {}),
            on_message: Rc::new(|_| {}),
            on_message_raw: Rc::new(|_| {}),
            on_message_raw_bytes: Rc::new(|_| {}),
            on_error: Rc::new(|_| {}),
            on_close: Rc::new(|_| {}),
            reconnect_limit: ReconnectLimit::default(),
            reconnect_interval: 3000,
            immediate: true,
            protocols: Default::default(),
        }
    }
}

/// Return type of [`use_websocket`].
#[derive(Clone)]
pub struct UseWebSocketReturn<T, OpenFn, CloseFn, SendFn>
where
    T: 'static,
    OpenFn: Fn() + Clone + 'static,
    CloseFn: Fn() + Clone + 'static,
    SendFn: Fn(&T) + Clone + 'static,
{
    /// The current state of the `WebSocket` connection.
    pub ready_state: Signal<ConnectionReadyState>,
    /// Latest message received from `WebSocket`.
    pub message: Signal<Option<T>>,
    /// The `WebSocket` instance.
    pub ws: Option<WebSocket>,
    /// Opens the `WebSocket` connection
    pub open: OpenFn,
    /// Closes the `WebSocket` connection
    pub close: CloseFn,
    /// Sends data through the socket
    pub send: SendFn,
}

#[derive(Error, Debug)]
pub enum UseWebSocketError<E, D> {
    #[error("WebSocket error event")]
    Event(Event),
    #[error("WebSocket codec error: {0}")]
    Codec(#[from] CodecError<E, D>),
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
