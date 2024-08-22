use leptos::html::Button;
use leptos::prelude::*;
use leptos_use::docs::demo_or_body;
use leptos_use::{use_element_hover_with_options, UseElementHoverOptions};

#[component]
fn Demo() -> impl IntoView {
    let el = NodeRef::<Button>::new();

    let is_hovered = use_element_hover_with_options(
        el,
        UseElementHoverOptions::default()
            .delay_enter(200)
            .delay_leave(600),
    );

    view! {
        <button node_ref=el>
            {move || if is_hovered.get() { "Thank you!" } else { "Hover me" }}
        </button>
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
