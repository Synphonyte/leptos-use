use leptos::*;
use leptos_use::docs::{demo_or_body, Note};
use leptos_use::{use_debounce_fn_with_options, DebounceOptions};

#[component]
fn Demo() -> impl IntoView {
    let (click_count, set_click_count) = create_signal(0);
    let (debounced_count, set_debounced_count) = create_signal(0);

    let debounced_fn = use_debounce_fn_with_options(
        move || set_debounced_count.set(debounced_count.get_untracked() + 1),
        1000.0,
        DebounceOptions::default().max_wait(Some(5000.0)),
    );

    view! {
        <button on:click=move |_| {
            set_click_count.set(click_count.get_untracked() + 1);
            debounced_fn();
        }>

            "Smash me!"
        </button>
        <Note>"Delay is set to 1000ms and max_wait is set to 5000ms for this demo."</Note>
        <p>"Button clicked: " {click_count}</p>
        <p>"Event handler called: " {debounced_count}</p>
    }
}

fn main() {
    _ = console_log::init_with_level(log::Level::Debug);
    console_error_panic_hook::set_once();

    mount_to(demo_or_body(), || {
        view! { <Demo/> }
    })
}
