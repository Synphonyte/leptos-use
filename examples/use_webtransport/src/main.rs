use leptos::*;
use leptos_use::core::ConnectionReadyState;
use leptos_use::docs::demo_or_body;
use leptos_use::{use_webtransport_with_options, UseWebTransportOptions};

#[component]
fn Demo() -> impl IntoView {
    let (log, set_log) = create_signal(vec![]);

    let transport = use_webtransport_with_options(
        "https://echo.webtransport.day",
        UseWebTransportOptions::default()
            .on_error(|e| set_log.update(|log| log.push(format!("Error: {:?}", e)))),
    );

    let (text, set_text) = create_signal("".to_string());

    let on_send = {
        let transport = transport.clone();

        move |e| {
            set_log.update(|log| log.push(format!("Sent datagram: '{}'", text())));

            transport.send_datagrams(text().as_bytes());
        }
    };

    let ready_state = transport.ready_state;

    let _ = watch(
        ready_state,
        move |ready, prev_ready, _| {
            if ready == &ConnectionReadyState::Open
                && prev_ready.unwrap_or(&ConnectionReadyState::Closed)
                    != &ConnectionReadyState::Open
            {
                set_log.update(|log| log.push("Connection opened".to_string()));
            } else if ready == &ConnectionReadyState::Closed
                && prev_ready.unwrap_or(&ConnectionReadyState::Open)
                    != &ConnectionReadyState::Closed
            {
                set_log.update(|log| log.push("Connection closed".to_string()));
            }
        },
        false,
    );

    let _ = watch(
        transport.datagrams,
        move |grams, _, _| {
            if let Some(grams) = grams {
                set_log.update(|log| {
                    log.push(format!(
                        "Received datagrams: '{}'",
                        String::from_utf8(grams.clone()).expect("valid utf8")
                    ))
                });
            }
        },
        false,
    );

    view! {
        <h2>Datagrams</h2>
        <textarea on:change=move |e| set_text(event_target_value(&e))/>
        <button on:click=on_send disabled=move || ready_state() != ConnectionReadyState::Open>"Send"</button>

        <div>
            <ul>
                {move || log().iter().map(|l| view! { <li>{l}</li> }).collect::<Vec<_>>()}
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
