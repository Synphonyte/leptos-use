use leptos::{leptos_dom::helpers::TimeoutHandle, *};

use core::fmt;
use std::rc::Rc;
use std::time::Duration;

use default_struct_builder::DefaultBuilder;
use js_sys::Array;
use wasm_bindgen::{prelude::*, JsCast, JsValue};
use web_sys::{BinaryType, CloseEvent, Event, MessageEvent, WebSocket};

use crate::utils::CloneableFnMutWithArg;

/// Creating and managing a [Websocket](https://developer.mozilla.org/en-US/docs/Web/API/WebSocket) connection.
///
/// ## Demo
///
/// [Link to Demo](https://github.com/Synphonyte/leptos-use/tree/main/examples/use_websocket)
///
/// ## Usage
///
/// ```
/// # use leptos::*;
/// # use leptos_use::websocket::*;
/// #
/// # #[component]
/// # fn Demo(cx: Scope) -> impl IntoView {
/// let UseWebsocketReturn {
///   ready_state,
///   message,
///   message_bytes,
///   send,
///   send_bytes,
///   open,
///   close,
///   ..
///   } = use_websocket(cx, "wss://echo.websocket.events/".to_string());
///
/// let send_message = move |_| {
///   let m = "Hello, world!".to_string();
///   send(m.clone());
/// };
///
/// let send_byte_message = move |_| {
///   let m = b"Hello, world!\r\n".to_vec();
///   send_bytes(m.clone());
/// };
///
/// let status = move || ready_state().to_string();
///
/// let connected = move || ready_state.get() == UseWebSocketReadyState::Open;
///
/// let open_connection = move |_| {
///   open();
/// };
///
/// let close_connection = move |_| {
///   close();
/// };
///
/// view! { cx,
/// <div>
///   <p>"status: " {status}</p>
///   button on:click=send_message disabled=move || !connected()>"Send"</button>
///   <button on:click=send_byte_message disabled=move || !connected()>"Send bytes"</button>
///   <button on:click=open_connection disabled=connected>"Open"</button>
///   <button on:click=close_connection disabled=move || !connected()>"Close"</button>
///   <p>"Receive message: " {format! {"{:?}", message}}</p>
///   <p>"Receive byte message: " {format! {"{:?}", message_bytes}}</p>
///   </div>
/// }
/// # }
/// ```
// #[doc(cfg(feature = "websocket"))]
pub fn use_websocket(
    cx: Scope,
    url: String,
) -> UseWebsocketReturn<
    impl Fn() + Clone + 'static,
    impl Fn() + Clone + 'static,
    impl Fn(String) + Clone + 'static,
    impl Fn(Vec<u8>) + Clone + 'static,
> {
    use_websocket_with_options(cx, url, UseWebSocketOptions::default())
}

/// Version of [`use_websocket`] that takes `UseWebSocketOptions`. See [`use_websocket`] for how to use.
// #[doc(cfg(feature = "websocket"))]
pub fn use_websocket_with_options(
    cx: Scope,
    url: String,
    options: UseWebSocketOptions,
) -> UseWebsocketReturn<
    impl Fn() + Clone + 'static,
    impl Fn() + Clone + 'static,
    impl Fn(String) + Clone + 'static,
    impl Fn(Vec<u8>) + Clone,
> {
    let (ready_state, set_ready_state) = create_signal(cx, UseWebSocketReadyState::Closed);
    let (message, set_message) = create_signal(cx, None);
    let (message_bytes, set_message_bytes) = create_signal(cx, None);
    let ws_ref: StoredValue<Option<WebSocket>> = store_value(cx, None);

    let onopen_ref = store_value(cx, options.onopen);
    let onmessage_ref = store_value(cx, options.onmessage);
    let onmessage_bytes_ref = store_value(cx, options.onmessage_bytes);
    let onerror_ref = store_value(cx, options.onerror);
    let onclose_ref = store_value(cx, options.onclose);

    let reconnect_limit = options.reconnect_limit.unwrap_or(3);
    let reconnect_interval = options.reconnect_interval.unwrap_or(3 * 1000);

    let reconnect_timer_ref: StoredValue<Option<TimeoutHandle>> = store_value(cx, None);
    let manual = options.manual;
    let protocols = options.protocols;

    let reconnect_times_ref: StoredValue<u64> = store_value(cx, 0);
    let unmounted_ref = store_value(cx, false);

    let connect_ref: StoredValue<Option<Rc<dyn Fn()>>> = store_value(cx, None);

    let reconnect_ref: StoredValue<Option<Rc<dyn Fn()>>> = store_value(cx, None);
    reconnect_ref.set_value({
        let ws = ws_ref.get_value();
        Some(Rc::new(move || {
            if reconnect_times_ref.get_value() < reconnect_limit
                && ws
                    .clone()
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
        let ws = ws_ref.get_value();
        let url = url.clone();

        Some(Rc::new(move || {
            reconnect_timer_ref.set_value(None);
            {
                if let Some(web_socket) = &ws {
                    let _ = web_socket.close();
                }
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
            set_ready_state.set(UseWebSocketReadyState::Connecting);

            // onopen handler
            {
                let onopen_closure = Closure::wrap(Box::new(move |e: Event| {
                    if unmounted_ref.get_value() {
                        return;
                    }

                    let mut onopen = onopen_ref.get_value();
                    onopen(e);

                    set_ready_state.set(UseWebSocketReadyState::Open);
                }) as Box<dyn FnMut(Event)>);
                web_socket.set_onopen(Some(onopen_closure.as_ref().unchecked_ref()));
                // Forget the closure to keep it alive
                onopen_closure.forget();
            }

            // onmessage handler
            {
                let onmessage_closure = Closure::wrap(Box::new(move |e: MessageEvent| {
                    if unmounted_ref.get_value() {
                        return;
                    }

                    e.data().dyn_into::<js_sys::ArrayBuffer>().map_or_else(
                        |_| {
                            e.data().dyn_into::<js_sys::JsString>().map_or_else(
                                |_| {
                                    unreachable!("message event, received Unknown: {:?}", e.data());
                                },
                                |txt| {
                                    let txt = String::from(&txt);
                                    let mut onmessage = onmessage_ref.get_value();
                                    onmessage(txt.clone());

                                    set_message.set(Some(txt.clone()));
                                },
                            );
                        },
                        |array_buffer| {
                            let array = js_sys::Uint8Array::new(&array_buffer);
                            let array = array.to_vec();
                            let mut onmessage_bytes = onmessage_bytes_ref.get_value();
                            onmessage_bytes(array.clone());

                            set_message_bytes.set(Some(array));
                        },
                    );
                })
                    as Box<dyn FnMut(MessageEvent)>);
                web_socket.set_onmessage(Some(onmessage_closure.as_ref().unchecked_ref()));
                onmessage_closure.forget();
            }
            // onerror handler
            {
                let onerror_closure = Closure::wrap(Box::new(move |e: Event| {
                    if unmounted_ref.get_value() {
                        return;
                    }

                    if let Some(reconnect) = &reconnect_ref.get_value() {
                        reconnect();
                    }

                    let mut onerror = onerror_ref.get_value();
                    onerror(e);

                    set_ready_state.set(UseWebSocketReadyState::Closed);
                }) as Box<dyn FnMut(Event)>);
                web_socket.set_onerror(Some(onerror_closure.as_ref().unchecked_ref()));
                onerror_closure.forget();
            }
            // onclose handler
            {
                let onclose_closure = Closure::wrap(Box::new(move |e: CloseEvent| {
                    if unmounted_ref.get_value() {
                        return;
                    }

                    if let Some(reconnect) = &reconnect_ref.get_value() {
                        reconnect();
                    }

                    let mut onclose = onclose_ref.get_value();
                    onclose(e);

                    set_ready_state.set(UseWebSocketReadyState::Closed);
                })
                    as Box<dyn FnMut(CloseEvent)>);
                web_socket.set_onclose(Some(onclose_closure.as_ref().unchecked_ref()));
                onclose_closure.forget();
            }

            ws_ref.set_value(Some(web_socket));
        }))
    });

    // Send text (String)
    let send = {
        Box::new(move |data: String| {
            if ready_state.get() == UseWebSocketReadyState::Open {
                if let Some(web_socket) = ws_ref.get_value() {
                    let _ = web_socket.send_with_str(&data);
                }
            }
        })
    };

    // Send bytes
    let send_bytes = {
        move |data: Vec<u8>| {
            if ready_state.get() == UseWebSocketReadyState::Open {
                if let Some(web_socket) = ws_ref.get_value() {
                    let _ = web_socket.send_with_u8_array(&data);
                }
            }
        }
    };

    // Open connection
    let open = {
        move || {
            reconnect_times_ref.set_value(0);
            if let Some(connect) = connect_ref.get_value() {
                connect();
            }
        }
    };

    // Close connection
    let close = {
        reconnect_timer_ref.set_value(None);
        move || {
            reconnect_times_ref.set_value(reconnect_limit);
            if let Some(web_socket) = ws_ref.get_value() {
                let _ = web_socket.close();
            }
        }
    };

    // Open connection (not called if option `manual` is true)
    {
        let open = open.clone();
        create_effect(cx, move |_| {
            if !manual {
                open();
            }

            || ()
        });
    }

    // clean up (unmount)
    {
        let close = close.clone();
        on_cleanup(cx, move || {
            unmounted_ref.set_value(true);
            close();
        });
    }

    UseWebsocketReturn {
        ready_state,
        message,
        message_bytes,
        ws: ws_ref.get_value(),
        open,
        close,
        send,
        send_bytes,
    }
}

/// The current state of the `WebSocket` connection.
// #[doc(cfg(feature = "websocket"))]
#[derive(Debug, PartialEq, Eq, Clone)]
pub enum UseWebSocketReadyState {
    Connecting,
    Open,
    Closing,
    Closed,
}

impl fmt::Display for UseWebSocketReadyState {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match *self {
            UseWebSocketReadyState::Connecting => write!(f, "Connecting"),
            UseWebSocketReadyState::Open => write!(f, "Open"),
            UseWebSocketReadyState::Closing => write!(f, "Closing"),
            UseWebSocketReadyState::Closed => write!(f, "Closed"),
        }
    }
}

/// Options for [`use_websocket_with_options`].
// #[doc(cfg(feature = "websocket"))]
#[derive(DefaultBuilder)]
pub struct UseWebSocketOptions {
    /// `WebSocket` connect callback.
    #[builder(into)]
    onopen: Box<dyn CloneableFnMutWithArg<Event>>,
    /// `WebSocket` message callback for text.
    #[builder(into)]
    onmessage: Box<dyn CloneableFnMutWithArg<String>>,
    /// `WebSocket` message callback for binary.
    #[builder(into)]
    onmessage_bytes: Box<dyn CloneableFnMutWithArg<Vec<u8>>>,
    /// `WebSocket` error callback.
    #[builder(into)]
    onerror: Box<dyn CloneableFnMutWithArg<Event>>,
    /// `WebSocket` close callback.
    #[builder(into)]
    onclose: Box<dyn CloneableFnMutWithArg<CloseEvent>>,
    /// Retry times.
    reconnect_limit: Option<u64>,
    /// Retry interval(ms).
    reconnect_interval: Option<u64>,
    /// Manually starts connection
    manual: bool,
    /// Sub protocols
    protocols: Option<Vec<String>>,
}

impl Default for UseWebSocketOptions {
    fn default() -> Self {
        Self {
            onopen: Box::new(|_| {}),
            onmessage: Box::new(|_| {}),
            onmessage_bytes: Box::new(|_| {}),
            onerror: Box::new(|_| {}),
            onclose: Box::new(|_| {}),
            reconnect_limit: Some(3),
            reconnect_interval: Some(3 * 1000),
            manual: false,
            protocols: Default::default(),
        }
    }
}

/// Return type of [`use_websocket`].
// #[doc(cfg(feature = "websocket"))]
#[derive(Clone)]
pub struct UseWebsocketReturn<OpenFn, CloseFn, SendFn, SendBytesFn>
where
    OpenFn: Fn() + Clone + 'static,
    CloseFn: Fn() + Clone + 'static,
    SendFn: Fn(String) + Clone + 'static,
    SendBytesFn: Fn(Vec<u8>) + Clone + 'static,
{
    /// The current state of the `WebSocket` connection.
    pub ready_state: ReadSignal<UseWebSocketReadyState>,
    /// Latest text message received from `WebSocket`.
    pub message: ReadSignal<Option<String>>,
    /// Latest binary message received from `WebSocket`.
    pub message_bytes: ReadSignal<Option<Vec<u8>>>,
    /// The `WebSocket` instance.
    pub ws: Option<WebSocket>,
    /// Opens the `WebSocket` connection
    pub open: OpenFn,
    /// Closes the `WebSocket` connection
    pub close: CloseFn,
    /// Sends `text` (string) based data
    pub send: SendFn,
    /// Sends binary data
    pub send_bytes: SendBytesFn,
}
