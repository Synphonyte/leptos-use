use crate::OptionLocalSignal;
use codee::{CodecError, Decoder, Encoder};
use leptos::prelude::*;
use thiserror::Error;
use wasm_bindgen::JsValue;

/// Return type of [`use_broadcast_channel`](crate::use_broadcast_channel).
pub struct UseBroadcastChannelReturn<T, PFn, CFn, C>
where
    T: Send + Sync + 'static,
    PFn: Fn(&T) + Clone,
    CFn: Fn() + Clone,
    C: Encoder<T> + Decoder<T> + Send + Sync,
{
    /// `true` if this browser supports `BroadcastChannel`s.
    pub is_supported: Signal<bool>,

    /// The broadcast channel that is wrapped by this function
    pub channel: OptionLocalSignal<web_sys::BroadcastChannel>,

    /// Latest message received from the channel
    pub message: Signal<Option<T>>,

    /// Sends a message through the channel
    pub post: PFn,

    /// Closes the channel
    pub close: CFn,

    /// Latest error as reported by the `messageerror` event.
    pub error: OptionLocalSignal<ErrorType<T, C>>,

    /// Wether the channel is closed
    pub is_closed: Signal<bool>,
}

type ErrorType<T, C> = UseBroadcastChannelError<<C as Encoder<T>>::Error, <C as Decoder<T>>::Error>;

#[derive(Debug, Error)]
pub enum UseBroadcastChannelError<E, D> {
    #[error("failed to post message")]
    PostMessage(JsValue),
    #[error("channel message error")]
    MessageEvent(web_sys::MessageEvent),
    #[error("failed to (de)encode value")]
    Codec(CodecError<E, D>),
    #[error("received value is not a string")]
    ValueNotString,
}

impl<E, D> Clone for UseBroadcastChannelError<E, D>
where
    E: Clone,
    D: Clone,
    CodecError<E, D>: Clone,
{
    fn clone(&self) -> Self {
        match self {
            UseBroadcastChannelError::PostMessage(v) => {
                UseBroadcastChannelError::PostMessage(v.clone())
            }
            UseBroadcastChannelError::MessageEvent(v) => {
                UseBroadcastChannelError::MessageEvent(v.clone())
            }
            UseBroadcastChannelError::Codec(v) => UseBroadcastChannelError::Codec(v.clone()),
            UseBroadcastChannelError::ValueNotString => UseBroadcastChannelError::ValueNotString,
        }
    }
}
