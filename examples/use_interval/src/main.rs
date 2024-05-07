use leptos::prelude::*;
use leptos_use::docs::demo_or_body;
use leptos_use::{use_interval, UseIntervalReturn};

#[component]
fn Demo() -> impl IntoView {
    let UseIntervalReturn { counter, .. } = use_interval(200);

    view! {
        <div>
            <p>"Interval fired: " {counter}</p>
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
