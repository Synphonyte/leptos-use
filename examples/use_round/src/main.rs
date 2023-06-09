use leptos::*;
use leptos_use::docs::demo_or_body;
use leptos_use::math::use_round;

#[component]
fn Demo(cx: Scope) -> impl IntoView {
    let (value, set_value) = create_signal(cx, 5.95);

    let result: Signal<f64> = use_round(cx, value);

    view! { cx,
        <input
            class="block"
            prop:value=value
            on:input=move |e| set_value(event_target_value(&e).parse().unwrap())
            type="range"
            min="0"
            max="10"
            step="0.01"
        />
        <p>"Value: " {value}</p>
        <p>"Rounded: " {result}</p>
    }
}

fn main() {
    _ = console_log::init_with_level(log::Level::Debug);
    console_error_panic_hook::set_once();

    mount_to(demo_or_body(), |cx| {
        view! { cx, <Demo /> }
    })
}
