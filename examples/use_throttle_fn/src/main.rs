use leptos::*;
use leptos_use::use_throttle_fn;

#[component]
fn Demo(cx: Scope) -> impl IntoView {
    let (click_count, set_click_count) = create_signal(cx, 0);
    let (throttled_count, set_throttled_count) = create_signal(cx, 0);

    let mut throttled_fn =
        use_throttle_fn(move || set_throttled_count(throttled_count() + 1), 1000.0);

    view! { cx,
        <button
            on:click=move |_| {
                set_click_count(click_count() + 1);
                throttled_fn();
            }
            class="rounded bg-blue-500 hover:bg-blue-400 py-2 px-4 text-white"
        >
            "Smash me!"
        </button>
        <p class="my-2"><small class="block">"Delay is set to 1000ms for this demo."</small></p>
        <p class="my-3">"Button clicked: " { click_count }</p>
        <p>"Event handler called: " { throttled_count }</p>
    }
}

fn main() {
    _ = console_log::init_with_level(log::Level::Debug);
    console_error_panic_hook::set_once();

    mount_to_body(|cx| {
        view! {cx,
            <div class="p-6 bg-gray-700 text-gray-300">
                <Demo />
            </div>
        }
    })
}
