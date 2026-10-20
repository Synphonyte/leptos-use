use crate::{
    OptionLocalRwSignal, UseBroadcastChannelError, UseBroadcastChannelReturn,
    UseEventListenerOptions, js, sendwrap_fn, use_event_listener_with_options, use_supported,
};
use codee::{CodecError, Decoder, Encoder};
use leptos::ev::messageerror;
use leptos::prelude::*;

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
/// # use leptos::prelude::*;
/// # use leptos_use_broadcast_channel::{use_broadcast_channel, UseBroadcastChannelReturn};
/// # use codee::string::FromToStringCodec;
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
/// Values are (en)decoded via the given codec. You can use any of the string codecs or a
/// binary codec wrapped in `Base64`.
///
/// > Please check [the codec chapter](https://leptos-use.rs/codecs.html) to see what codecs are
/// > available and what feature flags they require.
///
/// ```
/// # use leptos::prelude::*;
/// # use serde::{Deserialize, Serialize};
/// # use leptos_use_broadcast_channel::use_broadcast_channel;
/// # use codee::string::JsonSerdeCodec;
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
/// use_broadcast_channel::<MyState, JsonSerdeCodec>("everyting-is-awesome");
/// # view! { }
/// # }
/// ```
///
/// ## SendWrapped Return
///
/// The returned closures `post` and `close` are sendwrapped functions. They can
/// only be called from the same thread that called `use_broadcast_channel`.
pub fn use_broadcast_channel<T, C>(
    name: &str,
) -> UseBroadcastChannelReturn<
    T,
    impl Fn(&T) + Clone + Send + Sync + use<T, C>,
    impl Fn() + Clone + Send + Sync,
    C,
>
where
    T: Send + Sync,
    C: Encoder<T, Encoded = String> + Decoder<T, Encoded = str> + Send + Sync,
    <C as Encoder<T>>::Error: Send + Sync,
    <C as Decoder<T>>::Error: Send + Sync,
{
    let (is_closed, set_closed) = signal(false);
    let (message, set_message) = signal(None::<T>);
    let channel = OptionLocalRwSignal::<web_sys::BroadcastChannel>::new();
    let error = OptionLocalRwSignal::<
        UseBroadcastChannelError<<C as Encoder<T>>::Error, <C as Decoder<T>>::Error>,
    >::new();

    let is_supported = use_supported(|| js!("BroadcastChannel" in &window()));

    let post = {
        sendwrap_fn!(move |data: &T| {
            if let Some(channel) = channel.get_untracked() {
                match C::encode(data) {
                    Ok(msg) => {
                        if let Err(err) = channel.post_message(&msg.into()) {
                            error.set(Some(UseBroadcastChannelError::PostMessage(err)))
                        }
                    }
                    Err(err) => {
                        error.set(Some(UseBroadcastChannelError::Codec(CodecError::Encode(
                            err,
                        ))));
                    }
                }
            }
        })
    };

    let close = {
        sendwrap_fn!(move || {
            if let Some(channel) = channel.get_untracked() {
                channel.close();
            }
            set_closed.set(true);
        })
    };

    // initialize the BroadcastChannel and event listeners, if supported by the browser
    // we do this in an Effect to make sure it runs after mount
    Effect::new({
        let name = name.to_string();
        move |_| {
            if is_supported.get() {
                let channel_val = web_sys::BroadcastChannel::new(&name).ok();
                channel.set(channel_val.clone());

                if let Some(channel) = channel_val {
                    let _ = use_event_listener_with_options(
                        channel.clone(),
                        leptos::ev::message,
                        move |event| {
                            if let Some(data) = event.data().as_string() {
                                match C::decode(&data) {
                                    Ok(msg) => {
                                        set_message.set(Some(msg));
                                    }
                                    Err(err) => error.set(Some(UseBroadcastChannelError::Codec(
                                        CodecError::Decode(err),
                                    ))),
                                }
                            } else {
                                error.set(Some(UseBroadcastChannelError::ValueNotString));
                            }
                        },
                        UseEventListenerOptions::default().passive(true),
                    );

                    let _ = use_event_listener_with_options(
                        channel,
                        messageerror,
                        move |event| {
                            error.set(Some(UseBroadcastChannelError::MessageEvent(event)));
                        },
                        UseEventListenerOptions::default().passive(true),
                    );
                }
            }
        }
    });

    on_cleanup({
        let close = close.clone();

        move || {
            close();
        }
    });

    UseBroadcastChannelReturn {
        is_supported,
        channel: channel.read_only(),
        message: message.into(),
        post,
        close,
        error: error.read_only(),
        is_closed: is_closed.into(),
    }
}
