use leptos::*;
use leptos_use::docs::demo_or_body;
use leptos_use::{use_element_hover_with_options, UseElementHoverOptions};

#[component]
fn Demo(cx: Scope) -> impl IntoView {
    let el = create_node_ref(cx);

    let is_hovered = use_element_hover_with_options(
        cx,
        el,
        UseElementHoverOptions::default()
            .delay_enter(200)
            .delay_leave(600),
    );

    view! { cx,
        <button node_ref=el>{ move || if is_hovered() { "Thank you!" } else { "Hover me" } }</button>
    }
}

fn main() {
    _ = console_log::init_with_level(log::Level::Debug);
    console_error_panic_hook::set_once();

    mount_to(demo_or_body(), |cx| {
        view! { cx, <Demo /> }
    })
}
