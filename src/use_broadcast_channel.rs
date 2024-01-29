use crate::storage::Codec;
use crate::use_supported;
use default_struct_builder::DefaultBuilder;
use leptos::*;

///
///
/// ## Demo
///
/// [Link to Demo](https://github.com/Synphonyte/leptos-use/tree/main/examples/use_broadcast_channel)
///
/// ## Usage
///
/// ```
/// # use leptos::*;
/// # use leptos_use::use_broadcast_channel;
/// #
/// # #[component]
/// # fn Demo() -> impl IntoView {
/// use_broadcast_channel();
/// #
/// # view! { }
/// # }
/// ```
pub fn use_broadcast_channel<T, C>(name: &str) -> UseBroadcastChannelReturn
where
    C: Codec<T> + Default + Clone,
{
    let is_supported = use_supported(|| JsValue::from("BroadcastChannel").js_in(&window()));

    let (is_closed, set_closed) = create_signal(false);
    let (channel, set_channel) = create_signal(None::<web_sys::BroadcastChannel>);
    let (message, set_message) = create_signal(None::<T>);
    let (error, set_error) = create_signal(None::<web_sys::MessageEvent>);

    let post = move |data: T| {
        if let Some(channel) = channel.get_untracked() {
            channel.post_message().ok();
        }
    };
}

/// Return type of [`use_broadcast_channel`].
pub struct UseBroadcastChannelReturn<T, PFn, CFn>
where
    PFn: Fn(T),
    CFn: Fn(),
{
    /// `true` if this browser supports `BroadcastChannel`s.
    is_supported: Signal<bool>,

    /// The broadcast channel that is wrapped by this function
    channel: Signal<Option<web_sys::BroadcastChannel>>,

    /// Latest message received from the channel
    message: Signal<Option<T>>,

    /// Sends a message through the channel
    post: PFn,

    /// Closes the channel
    close: CFn,

    /// Latest error as reported by the `messageerror` event.
    error: Signal<Option<web_sys::MessageEvent>>,

    /// Wether the channel is closed
    is_closed: Signal<bool>,
}
