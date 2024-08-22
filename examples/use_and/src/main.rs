use leptos::prelude::*;
use leptos_use::docs::{demo_or_body, BooleanDisplay};
use leptos_use::math::use_and;

#[component]
fn Demo() -> impl IntoView {
    let (a, set_a) = signal(false);
    let (b, set_b) = signal(false);

    let a_and_b = use_and(a, b);

    view! {
        <div class="px-6 py-4 rounded grid grid-cols-[100px_auto] gap-4">
            <label for="smooth-scrolling-option" class="text-right opacity-75">
                Input <code>A</code>
            </label>
            <span>
                <input
                    id="smooth-scrolling-option"
                    prop:checked=a
                    on:input=move |e| set_a.set(event_target_checked(&e))
                    type="checkbox"
                />
            </span>
            <label for="smooth-scrolling-option" class="text-right opacity-75">
                Input <code>B</code>
            </label>
            <span>
                <input
                    id="smooth-scrolling-option"
                    prop:checked=b
                    on:input=move |e| set_b.set(event_target_checked(&e))
                    type="checkbox"
                />
            </span>
            <span class="text-right opacity-75">Output <code>"A && B"</code></span>
            <BooleanDisplay value=a_and_b/>
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
