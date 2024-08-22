use leptos::prelude::*;
use leptos_use::docs::demo_or_body;
use leptos_use::math::use_floor;

#[component]
fn Demo() -> impl IntoView {
    let (value, set_value) = signal(5.95);

    let result: Signal<f64> = use_floor(value);

    view! {
        <input
            class="block"
            prop:value=move || value.get()
            on:input=move |e| set_value.set(event_target_value(&e).parse().unwrap())
            type="range"
            min="0"
            max="10"
            step="0.01"
        />
        <p>"Value: " {move || value.get()}</p>
        <p>"Floored: " {move || result.get()}</p>
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
