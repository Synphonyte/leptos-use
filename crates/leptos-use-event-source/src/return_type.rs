use crate::ConnectionReadyState;
use codee::Decoder;
use leptos::prelude::*;
use std::fmt::Debug;
use std::marker::PhantomData;
use thiserror::Error;
use wasm_bindgen::JsCast;

/// Message received from the `EventSource` with transcoded data.
#[derive(PartialEq)]
pub struct UseEventSourceMessage<T, C>
where
    T: Clone + Send + Sync + 'static,
    C: Decoder<T, Encoded = str> + Send + Sync,
    C::Error: Send + Sync,
{
    pub event_type: String,
    pub data: T,
    pub last_event_id: String,
    _marker: PhantomData<C>,
}

impl<T, C> Clone for UseEventSourceMessage<T, C>
where
    T: Clone + Send + Sync + 'static,
    C: Decoder<T, Encoded = str> + Send + Sync,
    C::Error: Send + Sync,
{
    fn clone(&self) -> Self {
        Self {
            event_type: self.event_type.clone(),
            data: self.data.clone(),
            last_event_id: self.last_event_id.clone(),
            _marker: PhantomData,
        }
    }
}

impl<T, C> Debug for UseEventSourceMessage<T, C>
where
    T: Debug + Clone + Send + Sync + 'static,
    C: Decoder<T, Encoded = str> + Send + Sync,
    C::Error: Send + Sync,
{
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("UseEventSourceMessage")
            .field("data", &self.data)
            .field("event_type", &self.event_type)
            .field("last_event_id", &self.last_event_id)
            .finish()
    }
}

impl<T, C> TryFrom<&web_sys::MessageEvent> for UseEventSourceMessage<T, C>
where
    T: Clone + Send + Sync + 'static,
    C: Decoder<T, Encoded = str> + Send + Sync,
    C::Error: Send + Sync,
{
    type Error = UseEventSourceError<C::Error>;

    fn try_from(message_event: &web_sys::MessageEvent) -> Result<Self, Self::Error> {
        let data_string = message_event.data().as_string().unwrap_or_default();

        let data = C::decode(&data_string).map_err(UseEventSourceError::Deserialize)?;

        Ok(Self {
            event_type: message_event.type_(),
            data,
            last_event_id: message_event.last_event_id(),
            _marker: PhantomData,
        })
    }
}

impl<T, C> TryFrom<web_sys::Event> for UseEventSourceMessage<T, C>
where
    T: Clone + Send + Sync + 'static,
    C: Decoder<T, Encoded = str> + Send + Sync,
    C::Error: Send + Sync,
{
    type Error = UseEventSourceError<C::Error>;

    fn try_from(event: web_sys::Event) -> Result<Self, Self::Error> {
        let message_event = event
            .dyn_into::<web_sys::MessageEvent>()
            .map_err(|e| UseEventSourceError::CastToMessageEvent(e.type_()))?;

        UseEventSourceMessage::try_from(&message_event)
    }
}

/// Return type of the `on_event` handler in [`UseEventSourceOptions`](crate::UseEventSourceOptions).
pub enum UseEventSourceOnEventReturn {
    /// Ignore further processing of the message event in [`use_event_source`](crate::use_event_source).
    IgnoreProcessingMessage,
    /// Use the default processing of the message event in [`use_event_source`](crate::use_event_source).
    ProcessMessage,
}

/// Return type of [`use_event_source`](crate::use_event_source).
pub struct UseEventSourceReturn<T, C, Err, OpenFn, CloseFn>
where
    T: Clone + Send + Sync + 'static,
    C: Decoder<T, Encoded = str> + Send + Sync,
    C::Error: Send + Sync,
    Err: Send + Sync + 'static,
    OpenFn: Fn() + Clone + Send + Sync + 'static,
    CloseFn: Fn() + Clone + Send + Sync + 'static,
{
    /// The latest message
    pub message: Signal<Option<UseEventSourceMessage<T, C>>>,

    /// The current state of the connection,
    pub ready_state: Signal<ConnectionReadyState>,

    /// The current error
    pub error: Signal<Option<UseEventSourceError<Err>>>,

    /// (Re-)Opens the `EventSource` connection
    /// If the current one is active, will close it before opening a new one.
    pub open: OpenFn,

    /// Closes the `EventSource` connection
    pub close: CloseFn,
}

#[derive(Error, Debug)]
pub enum UseEventSourceError<Err> {
    #[error("Error event received")]
    ErrorEvent,

    #[error("Error decoding value")]
    Deserialize(Err),

    #[error("Error casting event '{0}' to MessageEvent")]
    CastToMessageEvent(String),
}
