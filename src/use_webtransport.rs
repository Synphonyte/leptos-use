use crate::core::ConnectionReadyState;
use async_trait::async_trait;
use cfg_if::cfg_if;
use default_struct_builder::DefaultBuilder;
use js_sys::Reflect;
use leptos::leptos_dom::helpers::TimeoutHandle;
use leptos::*;
use std::cell::{Cell, RefCell};
use std::rc::Rc;
use std::time::Duration;
use wasm_bindgen::prelude::*;
use wasm_bindgen_futures::JsFuture;

#[cfg(any(feature = "bincode", feature = "msgpack"))]
use serde::{Deserialize, Serialize};

#[cfg(feature = "msgpack")]
use rmp_serde::{from_slice, to_vec};
use thiserror::Error;

#[cfg(feature = "bincode")]
use bincode::serde::{decode_from_slice as from_slice, encode_to_vec as to_vec};

///
///
/// ## Demo
///
/// [Link to Demo](https://github.com/Synphonyte/leptos-use/tree/main/examples/use_webtransport)
///
/// ## Usage
///
/// ```
/// # use leptos::*;
/// # use leptos_use::use_webtransport;
/// #
/// # #[component]
/// # fn Demo() -> impl IntoView {
/// use_webtransport();
/// #
/// # view! { }
/// # }
/// ```
pub fn use_webtransport(url: &str) -> UseWebTransportReturn {
    use_webtransport_with_options(url, UseWebTransportOptions::default())
}

/// Version of [`use_webtransport`] that takes a `UseWebtransportOptions`. See [`use_webtransport`] for how to use.
pub fn use_webtransport_with_options(
    url: &str,
    options: UseWebTransportOptions,
) -> UseWebTransportReturn {
    let UseWebTransportOptions {
        on_open,
        // on_error,
        on_close,
        on_receive_stream,
        on_bidir_stream,
        reconnect_limit,
        reconnect_interval,
        immediate,
    } = options;
    let url = url.to_string();

    let (ready_state, set_ready_state) = create_signal(ConnectionReadyState::Closed);
    let ready_state: Signal<_> = ready_state.into();

    let transport = Rc::new(RefCell::new(None::<web_sys::WebTransport>));
    let datagrams_reader_initialized = Rc::new(Cell::new(false));
    let datagrams_writer = Rc::new(RefCell::new(None::<web_sys::WritableStreamDefaultWriter>));

    let reconnect_timer = Rc::new(Cell::new(None::<TimeoutHandle>));
    let reconnect_count = Rc::new(Cell::new(0_u64));

    let unmounted = Rc::new(Cell::new(false));

    let connect_ref = store_value(None::<Rc<dyn Fn()>>);

    let reconnect = Rc::new({
        let reconnect_timer = Rc::clone(&reconnect_timer);
        let reconnect_count = Rc::clone(&reconnect_count);

        move || {
            if reconnect_count.get() < reconnect_limit
                && ready_state.get_untracked() == ConnectionReadyState::Open
            {
                reconnect_timer.set(
                    set_timeout_with_handle(
                        move || {
                            if let Some(connect) = connect_ref.get_value() {
                                connect();
                                reconnect_count.set(reconnect_count.get() + 1);
                            }
                        },
                        Duration::from_millis(reconnect_interval),
                    )
                    .ok(),
                )
            }
        }
    });

    connect_ref.set_value(Some(Rc::new({
        let transport = Rc::clone(&transport);
        let reconnect_timer = Rc::clone(&reconnect_timer);
        let on_open = Rc::clone(&on_open);

        move || {
            reconnect_timer.set(None);

            if let Some(transport) = transport.borrow().as_ref() {
                transport.close();
            }

            let options = web_sys::WebTransportOptions::new();
            transport.replace(Some(
                web_sys::WebTransport::new_with_options(&url, &options).unwrap_throw(),
            ));

            set_ready_state.set(ConnectionReadyState::Connecting);

            spawn_local({
                let on_open = Rc::clone(&on_open);
                let transport = Rc::clone(&transport);

                async move {
                    let transport = transport.borrow();
                    let transport = transport.as_ref().expect("Transport should be set");

                    match JsFuture::from(transport.ready()).await {
                        Ok(_) => {
                            set_ready_state.set(ConnectionReadyState::Open);
                            on_open();
                        }
                        Err(e) => {
                            // TODO : handle error?
                            set_ready_state.set(ConnectionReadyState::Closed);
                        }
                    }
                }
            });
        }
    })));

    let open = {
        let reconnect_count = Rc::clone(&reconnect_count);

        move || {
            reconnect_count.set(0);
            if let Some(connect) = connect_ref.get_value() {
                connect();
            }
        }
    };

    let on_closed = {
        let reconnect = Rc::clone(&reconnect);
        let unmounted = Rc::clone(&unmounted);

        move || {
            if unmounted.get() {
                return;
            }

            // TODO
            // reconnect();
        }
    };

    let close = {
        let transport = Rc::clone(&transport);
        let reconnect_count = Rc::clone(&reconnect_count);

        move || {
            reconnect_count.set(reconnect_limit);

            if let Some(transport) = transport.take() {
                transport.close();
                set_ready_state.set(ConnectionReadyState::Closing);

                spawn_local(async move {
                    let result = JsFuture::from(transport.closed()).await;
                    set_ready_state.set(ConnectionReadyState::Closed);

                    match result {
                        Ok(_) => {
                            on_close();
                        }
                        Err(e) => {
                            // TODO : handle error?
                        }
                    }
                });
            }
        }
    };

    let (datagrams_signal, set_datagrams) = create_signal(None::<Vec<u8>>);

    let datagrams = Signal::derive({
        let transport = Rc::clone(&transport);
        let datagrams_reader_initialized = Rc::clone(&datagrams_reader_initialized);

        move || {
            let transport = Rc::clone(&transport);

            lazy_initialize_u8_reader(
                ready_state,
                Rc::clone(&datagrams_reader_initialized),
                move || {
                    transport
                        .borrow()
                        .as_ref()
                        .expect("transport should be set a this point")
                        .datagrams()
                        .readable()
                },
                set_datagrams,
            );

            datagrams_signal.get()
        }
    });

    // TODO : reliable streams

    {
        let unmounted = Rc::clone(&unmounted);

        on_cleanup(move || {
            unmounted.set(true);
            close();
        });
    }

    if immediate {
        open();
    }

    UseWebTransportReturn {
        transport,
        ready_state,
        datagrams,
        datagrams_writer,
    }
}

fn get_or_create_datagrams_writer(
    datagrams_writer: Rc<RefCell<Option<web_sys::WritableStreamDefaultWriter>>>,
    transport: &web_sys::WebTransport,
) -> web_sys::WritableStreamDefaultWriter {
    let writer = datagrams_writer.borrow().clone();

    if let Some(writer) = writer {
        writer
    } else {
        let writer = transport
            .datagrams()
            .writable()
            .get_writer()
            .expect("should be able to get the writer");
        datagrams_writer.replace(Some(writer.clone()));
        writer
    }
}

fn lazy_initialize_u8_reader(
    ready_state: Signal<ConnectionReadyState>,
    initialized: Rc<Cell<bool>>,
    get_readable_stream: impl Fn() -> web_sys::ReadableStream,
    set_signal: WriteSignal<Option<Vec<u8>>>,
) {
    lazy_initialize_reader(
        ready_state,
        initialized,
        get_readable_stream,
        move |value| {
            let value: js_sys::Uint8Array = value.into();
            set_signal.set(Some(value.to_vec()));
        },
    );
}

fn lazy_initialize_reader(
    ready_state: Signal<ConnectionReadyState>,
    initialized: Rc<Cell<bool>>,
    get_readable_stream: impl Fn() -> web_sys::ReadableStream,
    on_value: impl Fn(JsValue) + 'static,
) {
    if ready_state.get() == ConnectionReadyState::Open {
        if !initialized.get() {
            initialized.set(true);

            listen_to_stream(get_readable_stream(), on_value, move || {
                initialized.set(false)
            });
        }
    }
}

fn listen_to_stream(
    readable_stream: web_sys::ReadableStream,
    on_value: impl Fn(JsValue) + 'static,
    on_done: impl Fn() + 'static,
) {
    let reader: web_sys::ReadableStreamDefaultReader =
        readable_stream.get_reader().unchecked_into();

    spawn_local(async move {
        loop {
            let result = JsFuture::from(reader.read()).await;
            match result {
                Ok(result) => {
                    let done = Reflect::get(&result, &"done".into())
                        .expect("done should always be there")
                        .as_bool()
                        .unwrap_or(true);

                    if done {
                        // TODO : close connection?
                        break;
                    }

                    let value = Reflect::get(&result, &"value".into())
                        .expect("if not done there should be a value");

                    on_value(value);
                }
                Err(..) => {
                    // TODO : error handling?
                    break;
                }
            }
        }

        on_done();
    });
}

/// Options for [`use_webtransport_with_options`].
#[derive(DefaultBuilder)]
pub struct UseWebTransportOptions {
    /// Callback when `WebTransport` is ready.
    on_open: Rc<dyn Fn()>,

    /// Error callback.
    // TODO : ? on_error: Rc<dyn Fn(WebTransportError)>,

    /// Callback when `WebTransport` is closed.
    on_close: Rc<dyn Fn()>,

    /// Callback when the server opens a one-way stream.
    on_receive_stream: Rc<dyn Fn(ReceiveStream)>,

    /// Callback when the server opens a bidirectional stream.
    on_bidir_stream: Rc<dyn Fn(BidirStream)>,

    /// Retry times. Defaults to 3.
    reconnect_limit: u64,

    /// Retry interval in ms. Defaults to 3000.
    reconnect_interval: u64,

    /// If `true` the `WebSocket` connection will immediately be opened when calling this function.
    /// If `false` you have to manually call the `open` function.
    /// Defaults to `true`.
    immediate: bool,
}

impl Default for UseWebTransportOptions {
    fn default() -> Self {
        Self {
            on_open: Rc::new(|| {}),
            // on_error: Rc::new(|_| {}),
            on_close: Rc::new(|| {}),
            on_receive_stream: Rc::new(|_| {}),
            on_bidir_stream: Rc::new(|_| {}),
            reconnect_limit: 3,
            reconnect_interval: 3000,
            immediate: true,
        }
    }
}

#[async_trait(?Send)]
pub trait CloseableStream {
    fn close(&self);

    async fn close_async(&self) -> Result<(), WebTransportError>;
}

#[async_trait(?Send)]
pub trait SendableStream {
    fn writer(&self) -> &web_sys::WritableStreamDefaultWriter;

    fn send_bytes(&self, data: &[u8]) {
        let arr = js_sys::Uint8Array::from(data);
        let _ = self.writer().write_with_chunk(&arr);
    }

    async fn send_bytes_async(&self, data: &[u8]) -> Result<(), SendError> {
        let arr = js_sys::Uint8Array::from(data);
        let _ = JsFuture::from(self.writer().write_with_chunk(&arr))
            .await
            .map_err(|e| SendError::FailedToWrite(e));

        Ok(())
    }

    #[cfg(any(feature = "msgpack", feature = "bincode"))]
    fn send<T: Serialize>(&self, data: &T) {
        self.send_bytes(
            to_vec(data)
                .expect("Serialization should not fail")
                .as_slice(),
        );
    }

    #[cfg(any(feature = "msgpack", feature = "bincode"))]
    async fn send_async<T: Serialize>(&self, data: &T) -> Result<(), SendError> {
        let serialized = to_vec(data)?;
        self.send_bytes_async(&serialized).await
    }
}

#[derive(Clone, Debug)]
pub struct SendStream {
    pub writer: web_sys::WritableStreamDefaultWriter,
}

#[async_trait(?Send)]
impl SendableStream for SendStream {
    #[inline(always)]
    fn writer(&self) -> &web_sys::WritableStreamDefaultWriter {
        &self.writer
    }
}

#[async_trait(?Send)]
impl CloseableStream for SendStream {
    fn close(&self) {
        let _ = self.writer.close();
    }

    async fn close_async(&self) -> Result<(), WebTransportError> {
        let _ = JsFuture::from(self.writer.close())
            .await
            .map_err(|e| WebTransportError::OnCloseWriter(e))?;

        Ok(())
    }
}

#[derive(Clone, Debug)]
pub struct ReceiveStream {
    pub reader: web_sys::ReadableStreamDefaultReader,
    pub message: Signal<Option<Vec<u8>>>,
    // pub close: Rc<dyn Fn()>,
}

// TODO : implement ReceiveStream

#[derive(Clone, Debug)]
pub struct BidirStream {
    pub writer: web_sys::WritableStreamDefaultWriter,
    pub bytes: Signal<Option<Vec<u8>>>,
}

cfg_if! { if #[cfg(any(feature = "msgpack", feature = "bincode"))] {
    impl BidirStream {
        pub fn receive<T: for <'a> Deserialize<'a>>(&self) -> Signal<Option<T>> {
            let bytes = self.bytes;

            Signal::derive(move || {
                 bytes.get().map(|bytes| from_slice(bytes.as_slice()).expect("Deserialization should not fail"))
            })
        }

        pub fn try_receive<T: for <'a> Deserialize<'a>>(&self) -> Signal<Option<Result<T, ReceiveError>>> {
            let bytes = self.bytes;

            Signal::derive(move || {
                 bytes.get().map(|bytes| Ok(from_slice(bytes.as_slice())?))
            })
        }
    }
}}

#[async_trait(?Send)]
impl CloseableStream for BidirStream {
    fn close(&self) {
        let _ = self.writer.close();
    }

    async fn close_async(&self) -> Result<(), WebTransportError> {
        let _ = JsFuture::from(self.writer.close())
            .await
            .map_err(|e| WebTransportError::OnCloseWriter(e))?;

        Ok(())
    }
}

#[async_trait(?Send)]
impl SendableStream for BidirStream {
    #[inline(always)]
    fn writer(&self) -> &web_sys::WritableStreamDefaultWriter {
        &self.writer
    }
}

/// Return type of [`use_webtransport`].
#[derive(Clone, Debug)]
pub struct UseWebTransportReturn {
    transport: Rc<RefCell<Option<web_sys::WebTransport>>>,
    datagrams_writer: Rc<RefCell<Option<web_sys::WritableStreamDefaultWriter>>>,

    /// The current state of the `WebTransport` connection.
    pub ready_state: Signal<ConnectionReadyState>,
    /// Latest datagrams message received
    pub datagrams: Signal<Option<Vec<u8>>>,
}

impl UseWebTransportReturn {
    /// Access to the underlying `WebTransport`
    pub async fn transport(&self) -> Option<web_sys::WebTransport> {
        self.transport.borrow().clone()
    }

    /// Sends binary data through the datagrams stream
    pub fn send_datagrams(&self, data: &[u8]) {
        if let Some(transport) = self.transport.borrow().as_ref() {
            let writer =
                get_or_create_datagrams_writer(Rc::clone(&self.datagrams_writer), transport);

            let arr = js_sys::Uint8Array::from(data);
            let _ = writer.write_with_chunk(&arr);
        }
    }

    // TODO : send_datagrams_async

    /// Create a unidirectional send stream
    pub async fn create_send_stream(&self) -> Result<SendStream, WebTransportError> {
        if let Some(transport) = self.transport.borrow().as_ref() {
            let result = JsFuture::from(transport.create_unidirectional_stream())
                .await
                .map_err(|e| WebTransportError::FailedToOpenStream(e))?;
            let stream: web_sys::WritableStream = result.unchecked_into();
            let writer = stream
                .get_writer()
                .map_err(|e| WebTransportError::FailedToOpenWriter(e))?;

            Ok(SendStream { writer })
        } else {
            Err(WebTransportError::NotConnected)
        }
    }

    /// Create a bidirectional stream
    pub async fn create_bidir_stream(&self) -> Result<BidirStream, WebTransportError> {
        if let Some(transport) = self.transport.borrow().as_ref() {
            let result = JsFuture::from(transport.create_bidirectional_stream())
                .await
                .map_err(|e| WebTransportError::FailedToOpenStream(e))?;
            let stream: web_sys::WebTransportBidirectionalStream = result.unchecked_into();
            let writer = stream
                .writable()
                .get_writer()
                .map_err(|e| WebTransportError::FailedToOpenWriter(e))?;

            let bytes = Signal::derive({
                let reader_initialized = Rc::new(Cell::new(false));
                let ready_state = self.ready_state;
                let (message_signal, set_message) = create_signal(None::<Vec<u8>>);
                let stream = stream.clone();

                move || {
                    let stream = stream.clone();

                    lazy_initialize_u8_reader(
                        ready_state,
                        Rc::clone(&reader_initialized),
                        move || stream.readable().unchecked_into(),
                        set_message,
                    );

                    message_signal.get()
                }
            });

            Ok(BidirStream { writer, bytes })
        } else {
            Err(WebTransportError::NotConnected)
        }
    }
}

/// Error enum for [`UseWebTransportOptions::on_error`]
#[derive(Debug)]
pub enum WebTransportError {
    /// The `WebTransport` is not connected yet. Call `open` first.
    NotConnected,
    FailedToOpenStream(JsValue),
    FailedToOpenWriter(JsValue),
    FailedToOpenReader(JsValue),
    FailedToRead(JsValue),
    OnCloseWriter(JsValue),
    OnCloseReader(JsValue),
}

/// Error enum for [`SendStream::send`]
#[derive(Error, Debug)]
pub enum SendError {
    #[error("Failed to write to stream")]
    FailedToWrite(JsValue),

    #[cfg(feature = "bincode")]
    #[error("Serialization failed: {0}")]
    SerializationFailed(#[from] bincode::Error),

    #[cfg(feature = "msgpack")]
    #[error("Serialization failed: {0}")]
    SerializationFailed(#[from] rmp_serde::encode::Error),
}

#[derive(Error, Debug)]
pub enum ReceiveError {
    #[cfg(feature = "bincode")]
    #[error("Serialization failed: {0}")]
    SerializationFailed(#[from] bincode::Error),

    #[cfg(feature = "msgpack")]
    #[error("Serialization failed: {0}")]
    DeserializationFailed(#[from] rmp_serde::decode::Error),
}
