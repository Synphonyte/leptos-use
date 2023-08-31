use leptos::*;
use leptos_use::core::ConnectionReadyState;
use leptos_use::docs::demo_or_body;
use leptos_use::{use_webtransport_with_options, UseWebTransportOptions};

#[component]
fn Demo() -> impl IntoView {
    let (datagrams_log, set_datagrams_log) = create_signal(vec![]);

    let transport = use_webtransport_with_options(
        "https://echo.webtransport.day",
        UseWebTransportOptions::default()
            .on_open(move || {
                set_datagrams_log.update(|log| log.push("Connection opened".to_string()))
            })
            .on_close(move || {
                set_datagrams_log.update(|log| log.push("Connection closed".to_string()))
            })
            .on_error(move |e| set_datagrams_log.update(|log| log.push(format!("Error: {:?}", e)))),
    );

    let (text, set_text) = create_signal("".to_string());

    let on_send = {
        let transport = transport.clone();

        move |e| {
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

    let ready_state = transport.ready_state;

    view! {
        <h2>Datagrams</h2>
        <textarea on:change=move |e| set_text(event_target_value(&e)) prop:value=text />
        <button on:click=on_send disabled=move || ready_state() != ConnectionReadyState::Open>"Send"</button>

        <div>
            <ul>
                {move || datagrams_log().iter().map(|l| view! { <li>{l}</li> }).collect::<Vec<_>>()}
            </ul>
        </div>
    }
}

fn main() {
    _ = console_log::init_with_level(log::Level::Debug);
    console_error_panic_hook::set_once();

    mount_to(demo_or_body(), || {
        view! { <Demo/> }
    })
}
