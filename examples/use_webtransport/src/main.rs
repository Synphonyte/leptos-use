use leptos::*;
use leptos_use::docs::demo_or_body;
use leptos_use::use_webtransport;

#[component]
fn Demo() -> impl IntoView {
    let transport = use_webtransport("https://echo.webtransport.day");

    let (text, set_text) = create_signal("".to_string());

    let on_send = {
        let transport = transport.clone();

        move |e| {
            transport.send_datagrams(text().as_bytes());
        }
    };

    view! {
        <textarea on:change=move |e| set_text(event_target_value(&e))/>
        <button on:click=on_send>"Send"</button>
    }
}

fn main() {
    _ = console_log::init_with_level(log::Level::Debug);
    console_error_panic_hook::set_once();

    mount_to(demo_or_body(), || {
        view! { <Demo/> }
    })
}
