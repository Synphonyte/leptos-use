use crate::{DummyEncoder, ReconnectLimit};
use codee::{CodecError, Encoder, HybridEncoder};
use default_struct_builder::DefaultBuilder;
use leptos::prelude::*;
use std::marker::PhantomData;
use std::sync::Arc;
use thiserror::Error;
use web_sys::{CloseEvent, Event};

type ArcFnBytes = Arc<dyn Fn(&[u8]) + Send + Sync>;

/// Options for [`use_websocket_with_options`](crate::use_websocket_with_options).
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
    pub(crate) heartbeat: Option<HeartbeatOptions<Hb, HbCodec>>,
    /// `WebSocket` connect callback.
    pub(crate) on_open: Arc<dyn Fn(Event) + Send + Sync>,
    /// `WebSocket` message callback for typed message decoded by codec.
    #[builder(skip)]
    pub(crate) on_message: Arc<dyn Fn(&Rx) + Send + Sync>,
    /// `WebSocket` message callback for text.
    pub(crate) on_message_raw: Arc<dyn Fn(&str) + Send + Sync>,
    /// `WebSocket` message callback for binary.
    pub(crate) on_message_raw_bytes: ArcFnBytes,
    /// `WebSocket` error callback.
    #[builder(skip)]
    pub(crate) on_error: Arc<dyn Fn(UseWebSocketError<E, D>) + Send + Sync>,
    /// `WebSocket` close callback.
    pub(crate) on_close: Arc<dyn Fn(CloseEvent) + Send + Sync>,
    /// Retry times. Defaults to `ReconnectLimit::Limited(3)`. Use `ReconnectLimit::Infinite` for
    /// infinite retries.
    pub(crate) reconnect_limit: ReconnectLimit,
    /// Retry interval in ms. Defaults to 3000.
    pub(crate) reconnect_interval: u64,
    /// If `true` the `WebSocket` connection will immediately be opened when calling this function.
    /// If `false` you have to manually call the `open` function.
    /// Defaults to `true`.
    pub(crate) immediate: bool,
    /// Sub protocols. See [MDN Docs](https://developer.mozilla.org/en-US/docs/Web/API/WebSocket/WebSocket#protocols).
    ///
    /// Can be set as a signal to support protocols only available after the initial render.
    ///
    /// Note that protocols are only updated on the next websocket open() call, not whenever the signal is updated.
    /// Therefore "lazy" protocols should use the `immediate(false)` option and manually call `open()`.
    #[builder(into)]
    pub(crate) protocols: Signal<Option<Vec<String>>>,
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

/// Options for heartbeats
#[cfg_attr(feature = "ssr", allow(dead_code))]
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
    pub(crate) interval: u64,
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

#[derive(Error, Debug)]
pub enum UseWebSocketError<E, D> {
    #[error("WebSocket error event")]
    Event(Event),
    #[error("WebSocket codec error: {0}")]
    Codec(#[from] CodecError<E, D>),
    #[error("WebSocket heartbeat codec error: {0}")]
    HeartbeatCodec(String),
}
