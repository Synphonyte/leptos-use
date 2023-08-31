use leptos::*;
use leptos_use::core::ConnectionReadyState;
use leptos_use::docs::demo_or_body;
use leptos_use::{use_webtransport_with_options, UseWebTransportOptions};

mod log_display;
mod stream_bidir;
mod stream_send;

use log_display::*;
use stream_bidir::*;
use stream_send::*;

#[component]
fn Demo() -> impl IntoView {
    let (datagrams_log, set_datagrams_log) = create_signal(vec![]);

    let id = store_value(0);

    let transport = use_webtransport_with_options(
        "https://echo.webtransport.day",
        UseWebTransportOptions::default()
            .on_open(move || {
                set_datagrams_log.update(|log| log.push("Connection opened".to_string()))
            })
            .on_close(move || {
                set_datagrams_log.update(|log| log.push("Connection closed".to_string()))
            }),
    );

    let (text, set_text) = create_signal("".to_string());

    let on_send_datagrams = {
        let transport = transport.clone();

        move |_| {
            set_datagrams_log.update(|log| log.push(format!("Sent datagram: '{}'", text())));

            transport.send_datagrams(text().as_bytes());
            set_text("".to_string());
        }
    };

    let _ = watch(
        transport.datagrams,
        move |grams, _, _| {
            if let Some(grams) = grams {
                set_datagrams_log.update(|log| {
                    log.push(format!(
                        "Received datagrams: '{}'",
                        String::from_utf8(grams.clone()).expect("valid utf8")
                    ))
                });
            }
        },
        false,
    );

    let (bidir_streams, set_bidir_streams) = create_signal(vec![]);

    let on_open_bidir_stream = {
        let transport = transport.clone();

        move |_| {
            let transport = transport.clone();

            spawn_local(async move {
                match transport.create_bidir_stream().await {
                    Ok(bidir_stream) => {
                        let i = id.get_value();
                        id.set_value(i + 1);

                        set_bidir_streams.update(|s| s.push((i, bidir_stream)));
                    }
                    Err(e) => {
                        set_datagrams_log.update(|log| {
                            log.push(format!("Failed to open bidir stream: {:?}", e))
                        });
                    }
                }
            });
        }
    };

    let ready_state = transport.ready_state;

    view! {
        <button on:click=on_open_bidir_stream>"Open Bidir Stream"</button>
        <h2>Datagrams</h2>
        <textarea on:change=move |e| set_text(event_target_value(&e)) prop:value=text />
        <button on:click=on_send_datagrams disabled=move || ready_state() != ConnectionReadyState::Open>"Send"</button>

        <LogDisplay log=datagrams_log />

        <h2>Bidir Streams</h2>
        <For
            each=bidir_streams
            key=|(i, _)| *i
            view=move |(_, bidir_stream)| view! { <StreamBidir ready_state=ready_state stream=bidir_stream.clone() /> }
        />
    }
}

fn main() {
    _ = console_log::init_with_level(log::Level::Debug);
    console_error_panic_hook::set_once();

    mount_to(demo_or_body(), || {
        view! { <Demo/> }
    })
}
