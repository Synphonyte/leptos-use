use leptos::*;
use leptos_use::docs::demo_or_body;
use leptos_use::{use_cycle_list, UseCycleListReturn};

#[component]
fn Demo() -> impl IntoView {
    let UseCycleListReturn {
        state, next, prev, ..
    } = use_cycle_list(vec![
        "Dog", "Cat", "Lizard", "Shark", "Whale", "Dolphin", "Octopus", "Seal",
    ]);

    view! {
        <div>
            <div class="text-primary text-lg font-bold">{state}</div>
            <button on:click=move |_| { prev() }>"Prev"</button>
            <button on:click=move |_| { next() }>"Next"</button>
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
