use leptos::{leptos_dom::helpers::TimeoutHandle, *};

use core::fmt;
use std::rc::Rc;
use std::{cell::RefCell, time::Duration};

use js_sys::Array;
use wasm_bindgen::{prelude::*, JsCast, JsValue};
use web_sys::{BinaryType, Event, MessageEvent, WebSocket};

pub use web_sys::CloseEvent;

use crate::utils::CloneableFnMutWithArg;

/// The current state of the `WebSocket` connection.
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

/// Options for `WebSocket`.
// #[derive(DefaultBuilder)]
#[derive(Clone)]
pub struct UseWebSocketOptions {
    /// `WebSocket` connect callback.
    pub onopen: Option<Box<dyn CloneableFnMutWithArg<Event>>>,
    /// `WebSocket` message callback for text.
    pub onmessage: Option<Box<dyn CloneableFnMutWithArg<String>>>,
    /// `WebSocket` message callback for binary.
    pub onmessage_bytes: Option<Box<dyn CloneableFnMutWithArg<Vec<u8>>>>,
    /// `WebSocket` error callback.
    pub onerror: Option<Box<dyn CloneableFnMutWithArg<Event>>>,
    /// `WebSocket` close callback.
    pub onclose: Option<Box<dyn CloneableFnMutWithArg<CloseEvent>>>,

    /// Retry times.
    pub reconnect_limit: Option<u64>,
    /// Retry interval(ms).
    pub reconnect_interval: Option<u64>,
    /// Manually starts connection
    pub manual: bool,
    /// Sub protocols
    pub protocols: Option<Vec<String>>,
}

impl Default for UseWebSocketOptions {
    fn default() -> Self {
        Self {
            onopen: None,
            onmessage: None,
            onmessage_bytes: None,
            onerror: None,
            onclose: None,
            reconnect_limit: Some(3),
            reconnect_interval: Some(3 * 1000),
            manual: false,
            protocols: Default::default(),
        }
    }
}

/// Return type of [`use_websocket`].
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
    pub ws: Rc<RefCell<Option<WebSocket>>>,

    pub open: OpenFn,
    pub close: CloseFn,
    pub send: SendFn,
    pub send_bytes: SendBytesFn,
}

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
    let ws: Rc<RefCell<Option<WebSocket>>> = Rc::new(RefCell::new(None));

    let onopen = Rc::new(RefCell::new(options.onopen));
    let onmessage = Rc::new(RefCell::new(options.onmessage));
    let onmessage_bytes = Rc::new(RefCell::new(options.onmessage_bytes));
    let onerror = Rc::new(RefCell::new(options.onerror));
    let onclose = Rc::new(RefCell::new(options.onclose));

    let reconnect_limit = options.reconnect_limit.unwrap_or(3);
    let reconnect_interval = options.reconnect_interval.unwrap_or(3 * 1000);

    let reconnect_timer: Rc<RefCell<Option<TimeoutHandle>>> = Rc::new(RefCell::new(None));
    let manual = options.manual;
    let protocols = options.protocols;

    let reconnect_times: Rc<RefCell<u64>> = Rc::new(RefCell::new(0));
    let unmounted = Rc::new(RefCell::new(false));

    let connect: Rc<RefCell<Option<Rc<dyn Fn()>>>> = Rc::new(RefCell::new(None));

    let reconnect: Rc<RefCell<Option<Rc<dyn Fn()>>>> = Rc::new(RefCell::new(None));
    *reconnect.borrow_mut() = {
        let ws = Rc::clone(&ws);
        let reconnect_times = Rc::clone(&reconnect_times);
        let connect = connect.clone();
        Some(Rc::new(move || {
            if *reconnect_times.borrow() < reconnect_limit
                && ws
                    .borrow()
                    .as_ref()
                    .map_or(false, |ws: &WebSocket| ws.ready_state() != WebSocket::OPEN)
            {
                let reconnect_times = Rc::clone(&reconnect_times);
                let connect = Rc::clone(&connect);

                *reconnect_timer.borrow_mut() = set_timeout_with_handle(
                    move || {
                        let connect = &mut *connect.borrow_mut();
                        if let Some(connect) = connect {
                            connect();
                            *reconnect_times.borrow_mut() += 1;
                        }
                    },
                    Duration::from_millis(reconnect_interval),
                )
                .ok()
            }
        }))
    };

    *connect.borrow_mut() = {
        let ws = Rc::clone(&ws);
        let url = url.clone();
        let unmounted = Rc::clone(&unmounted);
        let onopen = Rc::clone(&onopen);
        let onmessage = Rc::clone(&onmessage);
        let onerror = Rc::clone(&onerror);
        let onclose = Rc::clone(&onclose);
        let reconnect = Rc::clone(&reconnect);

        Some(Rc::new(move || {
            {
                let web_socket: &mut Option<WebSocket> = &mut ws.borrow_mut();
                if let Some(web_socket) = web_socket {
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
                let unmounted = Rc::clone(&unmounted);
                let onopen = Rc::clone(&onopen);
                let onopen_closure = Closure::wrap(Box::new(move |e: Event| {
                    if *unmounted.borrow() {
                        return;
                    }

                    let onopen = &mut *onopen.borrow_mut();
                    if let Some(onopen) = onopen {
                        onopen(e);
                    }
                    set_ready_state.set(UseWebSocketReadyState::Open);
                }) as Box<dyn FnMut(Event)>);
                web_socket.set_onopen(Some(onopen_closure.as_ref().unchecked_ref()));
                // Forget the closure to keep it alive
                onopen_closure.forget();
            }

            // onmessage handler
            {
                let unmounted = Rc::clone(&unmounted);
                let onmessage = Rc::clone(&onmessage);
                let onmessage_bytes = onmessage_bytes.clone();
                let onmessage_closure = Closure::wrap(Box::new(move |e: MessageEvent| {
                    if *unmounted.borrow() {
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
                                    let onmessage = &mut *onmessage.borrow_mut();
                                    if let Some(onmessage) = onmessage {
                                        let txt = txt.clone();
                                        onmessage(txt);
                                    }
                                    set_message.set(Some(txt.clone()));
                                },
                            );
                        },
                        |array_buffer| {
                            let array = js_sys::Uint8Array::new(&array_buffer);
                            let array = array.to_vec();
                            let onmessage_bytes = &mut *onmessage_bytes.borrow_mut();
                            if let Some(onmessage_bytes) = onmessage_bytes {
                                let array = array.clone();
                                onmessage_bytes(array);
                            }
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
                let unmounted = Rc::clone(&unmounted);
                let onerror = Rc::clone(&onerror);
                let reconnect = Rc::clone(&reconnect);
                let onerror_closure = Closure::wrap(Box::new(move |e: Event| {
                    if *unmounted.borrow() {
                        return;
                    }

                    let reconnect: Rc<dyn Fn()> = { reconnect.borrow().as_ref().unwrap().clone() };
                    reconnect();

                    let onerror = &mut *onerror.borrow_mut();
                    if let Some(onerror) = onerror {
                        onerror(e);
                    }
                    set_ready_state.set(UseWebSocketReadyState::Closed);
                }) as Box<dyn FnMut(Event)>);
                web_socket.set_onerror(Some(onerror_closure.as_ref().unchecked_ref()));
                onerror_closure.forget();
            }
            // onclose handler
            {
                let unmounted = Rc::clone(&unmounted);
                let onclose = Rc::clone(&onclose);

                let reconnect = Rc::clone(&reconnect);
                let onclose_closure = Closure::wrap(Box::new(move |e: CloseEvent| {
                    if *unmounted.borrow() {
                        return;
                    }

                    let reconnect: Rc<dyn Fn()> = { reconnect.borrow().as_ref().unwrap().clone() };
                    reconnect();

                    let onclose = &mut *onclose.borrow_mut();
                    if let Some(onclose) = onclose {
                        onclose(e);
                    }
                    set_ready_state.set(UseWebSocketReadyState::Closed);
                })
                    as Box<dyn FnMut(CloseEvent)>);
                web_socket.set_onclose(Some(onclose_closure.as_ref().unchecked_ref()));
                onclose_closure.forget();
            }

            *ws.borrow_mut() = Some(web_socket);
        }))
    };

    // Send text (String)
    let send = {
        let ws = Rc::clone(&ws);
        Box::new(move |data: String| {
            if ready_state.get() == UseWebSocketReadyState::Open {
                if let Some(web_socket) = ws.borrow_mut().as_ref() {
                    let _ = web_socket.send_with_str(&data);
                }
            }
        })
    };

    // Send bytes
    let send_bytes = {
        let ws = Rc::clone(&ws);
        move |data: Vec<u8>| {
            if ready_state.get() == UseWebSocketReadyState::Open {
                let web_socket: &mut Option<WebSocket> = &mut ws.borrow_mut();
                if let Some(web_socket) = web_socket {
                    let _ = web_socket.send_with_u8_array(&data);
                }
            }
        }
    };

    // Open connection
    let open = {
        let reconnect_times_ref = Rc::clone(&reconnect_times);
        // let connect = Rc::clone(&connect);
        move || {
            let connect = connect.clone();
            *reconnect_times_ref.borrow_mut() = 0;
            let connect: Rc<dyn Fn()> = { connect.borrow().as_ref().unwrap().clone() };
            connect();
        }
    };

    // Close connection
    let close = {
        let ws = Rc::clone(&ws);
        let reconnect_times = Rc::clone(&reconnect_times);
        move || {
            *reconnect_times.as_ref().borrow_mut() = reconnect_limit;
            let web_socket: &mut Option<WebSocket> = &mut ws.borrow_mut();
            if let Some(web_socket) = web_socket {
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
            *unmounted.borrow_mut() = true;
            close();
        });
    }

    UseWebsocketReturn {
        ready_state,
        message,
        message_bytes,
        ws,
        open,
        close,
        send,
        send_bytes,
    }
}
