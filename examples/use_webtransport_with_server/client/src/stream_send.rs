use leptos::prelude::*;
use leptos_use::core::ConnectionReadyState;
use leptos_use::{SendableStream, StreamState};

#[component]
pub fn StreamSend<S, F>(
    #[prop(into)] ready_state: Signal<ConnectionReadyState>,
    stream_state: Signal<StreamState>,
    send_stream: S,
    on_send: F,
) -> impl IntoView
where
    S: SendableStream + 'static,
    F: Fn(String) + 'static,
{
    let (text, set_text) = signal("".to_string());

    let on_send = {
        move |_| {
            send_stream.send_bytes(text().as_bytes());
            on_send(text());
            set_text("".to_string());
        }
    };

    view! {
        <textarea on:change=move |e| set_text(event_target_value(&e)) prop:value=text />
        <button on:click=on_send disabled=move || ready_state() != ConnectionReadyState::Open || stream_state() != StreamState::Open>"Send"</button>
    }
}
