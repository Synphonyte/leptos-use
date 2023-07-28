use crate::core::ConnectionReadyState;
use crate::utils::{CloneableFn, CloneableFnWithArg};
use default_struct_builder::DefaultBuilder;
use leptos::leptos_dom::helpers::TimeoutHandle;
use leptos::*;
use std::rc::Rc;
use wasm_bindgen::prelude::*;
use wasm_bindgen_futures::JsFuture;

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
pub fn use_webtransport(url: &str) -> UseWebtransportReturn {
    use_webtransport_with_options(url, UseWebTransportOptions::default())
}

/// Version of [`use_webtransport`] that takes a `UseWebtransportOptions`. See [`use_webtransport`] for how to use.
pub fn use_webtransport_with_options(
    url: &str,
    options: UseWebTransportOptions,
) -> UseWebtransportReturn {
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

    let reconnect_timer_ref = store_value(None::<TimeoutHandle>);
    let reconnect_count_ref = store_value(0_u64);

    let connect = move || {
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
    };

    let open = {
        move || {
            reconnect_count_ref.set_value(0);
        }
    };

    UseWebtransportReturn {}
}

/// Options for [`use_webtransport_with_options`].
#[derive(DefaultBuilder)]
pub struct UseWebTransportOptions {
    /// Callback when `WebTransport` is ready.
    on_open: Box<dyn CloneableFn>,
    /// Error callback.
    on_error: Box<dyn CloneableFnWithArg<WebTransportError>>,
    /// Callback when `WebTransport` is closed.
    on_close: Box<dyn CloneableFn>,
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
            on_open: Box::new(|| {}),
            on_error: Box::new(|_| {}),
            on_close: Box::new(|| {}),
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
pub struct UseWebtransportReturn {
    /// The current state of the `WebTransport` connection.
    pub ready_state: Signal<ConnectionReadyState>,
}

/// Error enum for [`UseWebTransportOptions::on_error`]
pub enum WebTransportError {}
