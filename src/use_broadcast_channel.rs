use crate::utils::StringCodec;
use crate::{
    use_event_listener, use_event_listener_with_options, use_supported, UseEventListenerOptions,
};
use leptos::*;
use thiserror::Error;
use wasm_bindgen::JsValue;

/// Reactive [BroadcastChannel API](https://developer.mozilla.org/en-US/docs/Web/API/BroadcastChannel).
///
/// Closes a broadcast channel automatically when the component is cleaned up.
///
/// ## Demo
///
/// [Link to Demo](https://github.com/Synphonyte/leptos-use/tree/main/examples/use_broadcast_channel)
///
/// ## Usage
///
/// The BroadcastChannel interface represents a named channel that any browsing context of a given origin can subscribe to. It allows communication between different documents (in different windows, tabs, frames, or iframes) of the same origin.
///
/// Messages are broadcasted via a message event fired at all BroadcastChannel objects listening to the channel.
///
/// ```
/// # use leptos::*;
/// # use leptos_use::{use_broadcast_channel, UseBroadcastChannelReturn};
/// # use leptos_use::utils::FromToStringCodec;
/// #
/// # #[component]
/// # fn Demo() -> impl IntoView {
/// let UseBroadcastChannelReturn {
///     is_supported,
///     message,
///     post,
///     error,
///     close,
///     ..
/// } = use_broadcast_channel::<bool, FromToStringCodec>("some-channel-name");
///
/// post(&true);
///
/// close();
/// #
/// # view! { }
/// # }
/// ```
///
/// Just like with [`use_storage`] you can use different codecs for encoding and decoding.
///
/// ```
/// # use leptos::*;
/// # use serde::{Deserialize, Serialize};
/// # use leptos_use::use_broadcast_channel;
/// # use leptos_use::utils::JsonCodec;
/// #
/// // Data sent in JSON must implement Serialize, Deserialize:
/// #[derive(Serialize, Deserialize, Clone, PartialEq)]
/// pub struct MyState {
///     pub playing_lego: bool,
///     pub everything_is_awesome: String,
/// }
///
/// # #[component]
/// # fn Demo() -> impl IntoView {
/// use_broadcast_channel::<MyState, JsonCodec>("everyting-is-awesome");
/// # view! { }
/// # }
/// ```
///
/// ## Create Your Own Custom Codec
///
/// All you need to do is to implement the [`StringCodec`] trait together with `Default` and `Clone`.
pub fn use_broadcast_channel<T, C>(
    name: &str,
) -> UseBroadcastChannelReturn<T, impl Fn(&T) + Clone, impl Fn() + Clone, C::Error>
where
    C: StringCodec<T> + Default + Clone,
{
    let is_supported = use_supported(|| JsValue::from("BroadcastChannel").js_in(&window()));

    let (is_closed, set_closed) = create_signal(false);
    let (channel, set_channel) = create_signal(None::<web_sys::BroadcastChannel>);
    let (message, set_message) = create_signal(None::<T>);
    let (error, set_error) = create_signal(None::<UseBroadcastChannelError<C::Error>>);

    let codec = C::default();

    let post = {
        let codec = codec.clone();

        move |data: &T| {
            if let Some(channel) = channel.get_untracked() {
                match codec.encode(data) {
                    Ok(msg) => {
                        channel
                            .post_message(&msg.into())
                            .map_err(|err| {
                                set_error.set(Some(UseBroadcastChannelError::PostMessage(err)))
                            })
                            .ok();
                    }
                    Err(err) => {
                        set_error.set(Some(UseBroadcastChannelError::Encode(err)));
                    }
                }
            }
        }
    };

    let close = {
        move || {
            if let Some(channel) = channel.get_untracked() {
                channel.close();
            }
            set_closed.set(true);
        }
    };

    if is_supported.get_untracked() {
        let channel_val = web_sys::BroadcastChannel::new(name).ok();
        set_channel.set(channel_val.clone());

        if let Some(channel) = channel_val {
            let _ = use_event_listener_with_options(
                channel.clone(),
                ev::message,
                move |event| {
                    if let Some(data) = event.data().as_string() {
                        match codec.decode(data) {
                            Ok(msg) => {
                                set_message.set(Some(msg));
                            }
                            Err(err) => set_error.set(Some(UseBroadcastChannelError::Decode(err))),
                        }
                    } else {
                        set_error.set(Some(UseBroadcastChannelError::ValueNotString));
                    }
                },
                UseEventListenerOptions::default().passive(true),
            );

            let _ = use_event_listener_with_options(
                channel.clone(),
                ev::messageerror,
                move |event| {
                    set_error.set(Some(UseBroadcastChannelError::MessageEvent(event)));
                },
                UseEventListenerOptions::default().passive(true),
            );

            let _ = use_event_listener(channel, ev::close, move |_| set_closed.set(true));
        }
    }

    on_cleanup(move || {
        close();
    });

    UseBroadcastChannelReturn {
        is_supported,
        channel: channel.into(),
        message: message.into(),
        post,
        close,
        error: error.into(),
        is_closed: is_closed.into(),
    }
}

/// Return type of [`use_broadcast_channel`].
pub struct UseBroadcastChannelReturn<T, PFn, CFn, Err>
where
    T: 'static,
    PFn: Fn(&T) + Clone,
    CFn: Fn() + Clone,
    Err: 'static,
{
    /// `true` if this browser supports `BroadcastChannel`s.
    pub is_supported: Signal<bool>,

    /// The broadcast channel that is wrapped by this function
    pub channel: Signal<Option<web_sys::BroadcastChannel>>,

    /// Latest message received from the channel
    pub message: Signal<Option<T>>,

    /// Sends a message through the channel
    pub post: PFn,

    /// Closes the channel
    pub close: CFn,

    /// Latest error as reported by the `messageerror` event.
    pub error: Signal<Option<UseBroadcastChannelError<Err>>>,

    /// Wether the channel is closed
    pub is_closed: Signal<bool>,
}

#[derive(Debug, Error, Clone)]
pub enum UseBroadcastChannelError<Err> {
    #[error("failed to post message")]
    PostMessage(JsValue),
    #[error("channel message error")]
    MessageEvent(web_sys::MessageEvent),
    #[error("failed to encode value")]
    Encode(Err),
    #[error("failed to decode value")]
    Decode(Err),
    #[error("received value is not a string")]
    ValueNotString,
}
