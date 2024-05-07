use leptos::prelude::*;
use leptos_use::docs::demo_or_body;
use leptos_use::sync_signal;

#[component]
fn Demo() -> impl IntoView {
    let (a, set_a) = signal(String::new());
    let (b, set_b) = signal(String::new());

    let _ = sync_signal((a, set_a), (b, set_b));

    view! {
        <input prop:value=a on:input=move |e| set_a(event_target_value(&e)) placeholder="A" type="text" />
        <input prop:value=b on:input=move |e| set_b(event_target_value(&e)) placeholder="B" type="text" />
    }
}

fn main() {
    _ = console_log::init_with_level(log::Level::Debug);
    console_error_panic_hook::set_once();

    mount_to(demo_or_body(), || {
        view! { <Demo/> }
    })
}
