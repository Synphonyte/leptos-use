use crate::{LogDisplay, StreamSend};
use leptos::*;
use leptos_use::core::ConnectionReadyState;
use leptos_use::{BidirStream, CloseableStream, StreamState};

#[component]
pub fn StreamBidir(
    #[prop(into)] ready_state: Signal<ConnectionReadyState>,
    stream: BidirStream,
    opened_by: &'static str,
) -> impl IntoView {
    let (log, set_log) = create_signal(vec![]);

    let on_send = move |msg| {
        set_log.update(|log| log.push(format!("Sent: '{}'", msg)));
    };

    let _ = watch(
        stream.bytes,
        move |bytes, _, _| {
            if let Some(bytes) = bytes {
                set_log.update(|log| {
                    log.push(format!(
                        "Received bidir: '{}'",
                        String::from_utf8(bytes.clone()).expect("valid utf8")
                    ))
                });
            }
        },
        false,
    );

    let on_close = {
        let stream = stream.clone();

        move |_| {
            stream.close();
        }
    };

    view! {
        <p>Opened by {opened_by}</p>
        <StreamSend ready_state=ready_state stream_state=stream.state() send_stream=stream.clone() on_send=on_send />
        <LogDisplay log=log />
        // TODO : make component out of this:
        <p>Stream state: { let stream = stream.clone(); move || format!("{:?}", stream.state().get()) }</p>
        <button on:click=on_close disabled=move || ready_state() != ConnectionReadyState::Open || stream.state().get() != StreamState::Open>Close</button>
    }
}
