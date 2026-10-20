use crate::ConnectionReadyState;
use leptos::prelude::*;
use std::marker::PhantomData;

/// Return type of [`use_websocket`](crate::use_websocket).
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
    /// Opens the `WebSocket` connection
    pub open: OpenFn,
    /// Closes the `WebSocket` connection
    pub close: CloseFn,
    /// Sends data through the socket
    pub send: SendFn,

    pub(crate) _marker: PhantomData<Tx>,
}
