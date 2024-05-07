use crate::{LogDisplay, StreamSend};
use leptos::prelude::*;
use leptos_use::core::ConnectionReadyState;
use leptos_use::BidirStream;

#[component]
pub fn StreamBidir(
    #[prop(into)] ready_state: Signal<ConnectionReadyState>,
    stream: BidirStream,
) -> impl IntoView {
    let (log, set_log) = signal(vec![]);

    let on_send = move |msg| {
        set_log.update(|log| log.push(format!("Sent: '{}'", msg)));
    };

    let _ = watch(
        stream.bytes,
        move |bytes, _, _| {
            if let Some(bytes) = bytes {
                set_log.update(|log| {
                    log.push(format!(
                        "Received datagrams: '{}'",
                        String::from_utf8(bytes.clone()).expect("valid utf8")
                    ))
                });
            }
        },
        false,
    );

    view! {
        <StreamSend ready_state=ready_state send_stream=stream.clone() on_send=on_send />
        <LogDisplay log=log />
    }
}
