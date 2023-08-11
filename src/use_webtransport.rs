use crate::core::ConnectionReadyState;
use default_struct_builder::DefaultBuilder;
use js_sys::{Reflect, Uint8Array};
use leptos::leptos_dom::helpers::TimeoutHandle;
use leptos::*;
use std::rc::Rc;
use std::time::Duration;
use wasm_bindgen::prelude::*;
use wasm_bindgen_futures::JsFuture;
use web_sys::{
    ReadableStream, ReadableStreamReaderMode, WebTransport, WritableStreamDefaultWriter,
};

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
pub fn use_webtransport(url: &str) -> UseWebTransportReturn<impl Fn(Vec<u8>) + Clone + 'static> {
    use_webtransport_with_options(url, UseWebTransportOptions::default())
}

/// Version of [`use_webtransport`] that takes a `UseWebtransportOptions`. See [`use_webtransport`] for how to use.
pub fn use_webtransport_with_options(
    url: &str,
    options: UseWebTransportOptions,
) -> UseWebTransportReturn<impl Fn(Vec<u8>) + Clone + 'static> {
    let UseWebTransportOptions {
        on_open,
        on_error,
        on_close,
        reconnect_limit,
        reconnect_interval,
        immediate,
    } = options;

    let (ready_state, set_ready_state) = create_signal(ConnectionReadyState::Closed);

    let transport_ref = store_value(None::<web_sys::WebTransport>);
    let datagrams_reader_initialized_ref = store_value(false);
    let datagrams_writer_ref = store_value(None::<web_sys::WritableStreamDefaultWriter>);

    let reconnect_timer_ref = store_value(None::<TimeoutHandle>);
    let reconnect_count_ref = store_value(0_u64);

    let connect_ref = store_value(None::<Rc<dyn Fn()>>);

    let reconnect = Rc::new(move || {
        if reconnect_count_ref.get_value() < reconnect_limit
            && ready_state.get_untracked() == ConnectionReadyState::Open
        {
            reconnect_timer_ref.set_value(
                set_timeout_with_handle(
                    move || {
                        if let Some(connect) = connect_ref.get_value() {
                            connect();
                            reconnect_count_ref.update_value(|current| *current += 1);
                        }
                    },
                    Duration::from_millis(reconnect_interval),
                )
                .ok(),
            )
        }
    });

    connect_ref.set_value(Some(Rc::new(move || {
        let transport = transport_ref.get_value();

        reconnect_timer_ref.set_value(None);

        if let Some(transport) = transport.as_ref() {
            transport.close();
        }

        let transport =
            web_sys::WebTransport::new_with_options(url, &options.into()).unwrap_throw();

        set_ready_state.set(ConnectionReadyState::Connecting);

        spawn_local(async move {
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
        });
    })));

    let open = {
        move || {
            reconnect_count_ref.set_value(0);
            if let Some(connect) = connect_ref.get_value() {
                connect();
            }
        }
    };

    let close = move || {
        reconnect_count_ref.set_value(reconnect_limit);
        if let Some(transport) = transport_ref.get_value() {
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
    };

    let (datagrams_signal, set_datagrams) = create_signal(None::<Vec<u8>>);

    let datagrams = Signal::derive(move || {
        // lazily create datagrams reader listener
        if ready_state.get() == ConnectionReadyState::Open {
            initialize_datagrams_reader(
                transport_ref,
                datagrams_reader_initialized_ref,
                set_datagrams,
            );
        }

        datagrams_signal.get()
    });

    let send_datagrams = move |data| {
        if let Some(transport) = transport_ref.get_value() {
            let writer = get_or_create_datagrams_writer(datagrams_writer_ref, transport);

            let _ = writer.write_with_chunk(&data.into());
        }
    };

    // TODO : reliable streams

    UseWebTransportReturn {
        ready_state: ready_state.into(),
        datagrams,
        send_datagrams,
    }
}

fn get_or_create_datagrams_writer(
    datagrams_writer_ref: StoredValue<Option<WritableStreamDefaultWriter>>,
    transport: WebTransport,
) -> WritableStreamDefaultWriter {
    if let Some(writer) = datagrams_writer_ref.get_value() {
        writer
    } else {
        let writer = transport
            .datagrams()
            .writable()
            .get_writer()
            .expect("should be able to get the writer");
        datagrams_writer_ref.set_value(Some(writer.clone()));
        writer
    }
}

fn initialize_datagrams_reader(
    transport_ref: StoredValue<Option<web_sys::WebTransport>>,
    datagrams_reader_initialized_ref: StoredValue<bool>,
    set_datagrams: WriteSignal<Option<Vec<u8>>>,
) {
    if !datagrams_reader_initialized_ref.get_value() {
        if let Some(transport) = transport_ref.get_value() {
            datagrams_reader_initialized_ref.set_value(true);

            listen_to_stream(
                transport.datagrams().readable(),
                move || datagrams_reader_initialized_ref.set_value(false),
                set_datagrams,
            );
        }
    }
}

fn listen_to_stream(
    readable_stream: ReadableStream,
    on_done: fn(),
    set_signal: WriteSignal<Option<Vec<u8>>>,
) {
    let mut reader_options = web_sys::ReadableStreamGetReaderOptions::new();
    reader_options.mode(ReadableStreamReaderMode::Byob);

    let reader: web_sys::ReadableStreamByobReader = readable_stream
        .get_reader_with_options(&reader_options)
        .into();

    spawn_local(async move {
        // the length value 4000 is taken from the MDN example
        // https://developer.mozilla.org/en-US/docs/Web/API/ReadableStreamBYOBReader/read#examples
        let mut buffer = [0_u8; 4000];

        loop {
            let result = JsFuture::from(reader.read_with_u8_array(&mut buffer)).await;
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

                    let value: Uint8Array = Reflect::get(&result, &"value".into())
                        .expect("if not done there should be a value")
                        .into();

                    set_signal.set(Some(value.to_vec()))
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
    on_error: Rc<dyn Fn(WebTransportError)>,
    /// Callback when `WebTransport` is closed.
    on_close: Rc<dyn Fn()>,
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
            on_error: Rc::new(|_| {}),
            on_close: Rc::new(|| {}),
            reconnect_limit: 3,
            reconnect_interval: 3000,
            immediate: true,
        }
    }
}

impl From<UseWebTransportOptions> for web_sys::WebTransportOptions {
    fn from(options: UseWebTransportOptions) -> Self {
        web_sys::WebTransportOptions::new()
    }
}

/// Return type of [`use_webtransport`].
pub struct UseWebTransportReturn<SendGramsFn>
where
    SendGramsFn: Fn(Vec<u8>) + Clone + 'static,
{
    /// The current state of the `WebTransport` connection.
    pub ready_state: Signal<ConnectionReadyState>,
    /// Latest datagrams message received
    pub datagrams: Signal<Option<Vec<u8>>>,
    /// Sends binary data through the datagrams stream
    pub send_datagrams: SendGramsFn,
}

/// Error enum for [`UseWebTransportOptions::on_error`]
pub enum WebTransportError {}
