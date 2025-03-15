use leptos::prelude::*;
use leptos_use::docs::demo_or_body;
use leptos_use::{
    core::ConnectionReadyState, use_websocket, use_websocket_with_options, ReconnectLimit,
    UseWebSocketError, UseWebSocketOptions, UseWebSocketReturn,
};
use serde::{Deserialize, Serialize};
use std::fmt::Display;

use codee::{binary::MsgpackSerdeCodec, string::FromToStringCodec};
use web_sys::{CloseEvent, Event};

#[derive(Serialize, Deserialize, Debug)]
struct Apple {
    name: String,
    worm_count: u32,
}

#[derive(Default)]
struct Heartbeat;

impl Display for Heartbeat {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "<Heartbeat>")
    }
}

#[component]
fn Demo() -> impl IntoView {
    let (history, set_history) = signal(vec![]);

    fn update_history(&history: &WriteSignal<Vec<String>>, message: String) {
        let _ = &history.update(|history: &mut Vec<_>| history.push(message));
    }
    // ----------------------------
    // use_websocket
    // ----------------------------

    let UseWebSocketReturn {
        ready_state,
        message,
        send,
        open,
        close,
        ..
    } = use_websocket::<Apple, Apple, MsgpackSerdeCodec>("wss://echo.websocket.events/");

    let send_message = move |_| {
        let m = Apple {
            name: "More worm than apple".to_string(),
            worm_count: 10,
        };
        send(&m);
        set_history.update(|history: &mut Vec<_>| history.push(format!("[send]: {:?}", m)));
    };

    let status = move || ready_state.get().to_string();

    let connected = move || ready_state.get() == ConnectionReadyState::Open;

    let open_connection = move |_| {
        open();
    };
    let close_connection = move |_| {
        close();
    };

    Effect::new(move |_| {
        message.with(move |message| {
            if let Some(m) = message {
                update_history(&set_history, format!("[message]: {:?}", m));
            }
        })
    });

    // ----------------------------
    // use_websocket_with_options
    // ----------------------------

    let (history2, set_history2) = signal(vec![]);

    let on_open_callback = move |e: Event| {
        set_history2.update(|history: &mut Vec<_>| {
            history.push(format!("[onopen]: event {:?}", e.type_()))
        });
    };

    let on_close_callback = move |e: CloseEvent| {
        set_history2.update(|history: &mut Vec<_>| {
            history.push(format!("[onclose]: event {:?}", e.type_()))
        });
    };

    let on_error_callback = move |e: UseWebSocketError<_, _>| {
        set_history2.update(|history: &mut Vec<_>| {
            history.push(match e {
                UseWebSocketError::Event(e) => format!("[onerror]: event {:?}", e.type_()),
                _ => format!("[onerror]: {:?}", e),
            })
        });
    };

    let on_message_callback = move |m: &String| {
        set_history2.update(|history: &mut Vec<_>| history.push(format!("[onmessage]: {:?}", m)));
    };

    let UseWebSocketReturn {
        ready_state: ready_state2,
        send: send2,
        open: open2,
        close: close2,
        message: message2,
        ..
    } = use_websocket_with_options::<String, String, FromToStringCodec, _, _>(
        "wss://echo.websocket.events/",
        UseWebSocketOptions::default()
            .immediate(false)
            .reconnect_limit(ReconnectLimit::Infinite)
            .on_open(on_open_callback)
            .on_close(on_close_callback)
            .on_error(on_error_callback)
            .on_message(on_message_callback)
            .heartbeat::<Heartbeat, FromToStringCodec>(1000),
    );

    let open_connection2 = move |_| {
        open2();
    };
    let close_connection2 = move |_| {
        close2();
    };

    let send_message2 = move |_| {
        let message = "Hello, use_leptos!".to_string();
        send2(&message);
        update_history(&set_history2, format!("[send]: {:?}", message));
    };

    let status2 = move || ready_state2.get().to_string();

    Effect::new(move |_| {
        if let Some(m) = message2.get() {
            update_history(&set_history2, format!("[message]: {:?}", m));
        };
    });

    let connected2 = move || ready_state2.get() == ConnectionReadyState::Open;

    view! {
        <div class="container">
            <div class="flex flex-col lg:flex-row gap-4">
                <div class="w-full lg:w-1/2">
                    <h1 class="text-xl lg:text-4xl mb-2">"use_websocket"</h1>
                    <p>"status: " {status}</p>
                    <button on:click=send_message disabled=move || !connected()>
                        "Send"
                    </button>

                    <button on:click=open_connection disabled=connected>
                        "Open"
                    </button>
                    <button on:click=close_connection disabled=move || !connected()>
                        "Close"
                    </button>
                    <div class="flex items-center">
                        <h3 class="text-2xl mr-2">"History"</h3>
                        <button
                            on:click=move |_| set_history.set(vec![])
                            disabled=move || history2.with(Vec::is_empty)
                        >
                            "Clear"
                        </button>
                    </div>
                    <For
                        each=move || history.get().into_iter().enumerate()
                        key=|(index, _)| *index
                        let:item
                    >
                        <div>{item.1}</div>
                    </For>

                </div>
                <div class="w-full lg:w-1/2">
                    <h1 class="text-xl lg:text-4xl mb-2">"use_websocket_with_options"</h1>
                    <p>"status: " {status2}</p>
                    <button on:click=open_connection2 disabled=connected2>
                        "Connect"
                    </button>
                    <button on:click=close_connection2 disabled=move || !connected2()>
                        "Close"
                    </button>
                    <button on:click=send_message2 disabled=move || !connected2()>
                        "Send"
                    </button>

                    <div class="flex items-center">
                        <h3 class="text-2xl mr-2">"History"</h3>
                        <button
                            on:click=move |_| set_history2.set(vec![])
                            disabled=move || history2.with(Vec::is_empty)
                        >
                            "Clear"
                        </button>
                    </div>
                    <ul>
                        <For
                            each=move || history2.get().into_iter().enumerate()
                            key=|(index, _)| *index
                            let:item
                        >
                            <li>{item.1}</li>
                        </For>
                    </ul>
                </div>
            </div>

        </div>
    }
}

fn main() {
    _ = console_log::init_with_level(log::Level::Info);
    console_error_panic_hook::set_once();

    let unmount_handle = leptos::mount::mount_to(demo_or_body(), || {
        view! { <Demo/> }
    });

    unmount_handle.forget();
}
