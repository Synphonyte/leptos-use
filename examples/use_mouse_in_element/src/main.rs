use leptos::html::Div;
use leptos::*;
use leptos_use::docs::demo_or_body;
use leptos_use::{use_mouse_in_element, UseMouseInElementReturn};

#[component]
fn Demo() -> impl IntoView {
    let el = create_node_ref::<Div>();

    let UseMouseInElementReturn {
        x,
        y,
        source_type,
        element_x,
        element_y,
        element_position_x,
        element_position_y,
        element_width,
        element_height,
        is_outside,
        ..
    } = use_mouse_in_element(el);

    view! {
            <div class="flex gap-4">
                <div
                    node_ref=el
                    class="el w-40 h-40 bg-gray-400/20 border-rounded flex place-content-center select-none"
                >
                    <div class="m-auto">Hover me</div>
                </div>
                <pre lang="yaml">    x: {x}
    y: {y}
    source_type: {move || format!("{:?}", source_type())}
    element_x: {element_x}
    element_y: {element_y}
    element_position_x: {element_position_x}
    element_position_y: {element_position_y}
    element_width: {element_width}
    element_height: {element_height}
    is_outside: {is_outside}</pre>
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
