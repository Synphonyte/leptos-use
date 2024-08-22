use leptos::prelude::*;
use leptos_use::docs::{demo_or_body, Note};
use leptos_use::signal_throttled;

#[component]
fn Demo() -> impl IntoView {
    let (input, set_input) = signal("".to_string());
    let throttled: Signal<String> = signal_throttled(input, 1000.0);

    view! {
        <div>
            <input
                type="text"
                value=input
                on:input=move |event| set_input(event_target_value(&event))
                placeholder="Try to type quickly..."
            />
            <Note>Delay is set to 1000ms for this demo.</Note>
            <p>Input signal: {input}</p>
            <p>Throttled signal: {throttled}</p>
        </div>
    }
}

fn main() {
    _ = console_log::init_with_level(log::Level::Debug);
    console_error_panic_hook::set_once();

    let unmount_handle = leptos::mount::mount_to(demo_or_body(), || {
        view! { <Demo/> }
    });

    unmount_handle.forget();
}
