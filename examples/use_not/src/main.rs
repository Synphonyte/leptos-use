use leptos::prelude::*;
use leptos_use::docs::{demo_or_body, BooleanDisplay};
use leptos_use::math::use_not;

#[component]
fn Demo() -> impl IntoView {
    let (a, set_a) = signal(false);
    let not_a = use_not(a);

    view! {
        <div class="px-6 py-4 rounded grid grid-cols-[100px_auto] gap-4">
            <label for_="smooth-scrolling-option" class="text-right opacity-75">
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
            <span class="text-right opacity-75">Output <code>"!A"</code></span>
            <BooleanDisplay value=not_a/>
        </div>
    }
}

fn main() {
    _ = console_log::init_with_level(log::Level::Debug);
    console_error_panic_hook::set_once();

    mount_to(demo_or_body(), || {
        view! { <Demo/> }
    })
}
