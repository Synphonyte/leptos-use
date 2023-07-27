use leptos::*;
use leptos_use::docs::{demo_or_body, Note};
use leptos_use::use_throttle_fn;

#[component]
fn Demo() -> impl IntoView {
    let (click_count, set_click_count) = create_signal(0);
    let (throttled_count, set_throttled_count) = create_signal(0);

    let throttled_fn = use_throttle_fn(
        move || set_throttled_count.set(throttled_count.get_untracked() + 1),
        1000.0,
    );

    view! {
        <button on:click=move |_| {
            set_click_count.set(click_count.get_untracked() + 1);
            throttled_fn();
        }>

            "Smash me!"
        </button>
        <Note>"Delay is set to 1000ms for this demo."</Note>
        <p>"Button clicked: " {click_count}</p>
        <p>"Event handler called: " {throttled_count}</p>
    }
}

fn main() {
    _ = console_log::init_with_level(log::Level::Debug);
    console_error_panic_hook::set_once();

    mount_to(demo_or_body(), || {
        view! { <Demo/> }
    })
}
