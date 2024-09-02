use leptos::*;
use leptos_use::docs::{demo_or_body, BooleanDisplay};
use leptos_use::{use_toggle, UseToggleReturn};

#[component]
fn Demo() -> impl IntoView {
    let UseToggleReturn { toggle, value, set_value } = use_toggle(true);

    view! {
        <p>Value: <BooleanDisplay value=value /></p>
        <button on:click=move |_| toggle()>Toggle</button>
        <button on:click=move |_| set_value(true)>Set <code>true</code></button>
        <button on:click=move |_| set_value(false)>Set <code>false</code></button>
    }
}

fn main() {
    _ = console_log::init_with_level(log::Level::Debug);
    console_error_panic_hook::set_once();

    mount_to(demo_or_body(), || {
        view! { <Demo /> }
    })
}
