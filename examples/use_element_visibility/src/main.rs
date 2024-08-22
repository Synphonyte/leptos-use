use leptos::html::Div;
use leptos::prelude::*;
use leptos_use::docs::{demo_or_body, BooleanDisplay, Note};
use leptos_use::use_element_visibility;

#[component]
fn Demo() -> impl IntoView {
    let el = NodeRef::<Div>::new();

    let is_visible = use_element_visibility(el);

    view! {
        <div>
            <Note class="mb-4">"Info on the right bottom corner"</Note>
            <div node_ref=el class="max-w-lg relative area dark:bg-gray-800 shadow-lg z-60">
                "Target Element (scroll down)"
            </div>
        </div>
        <div class="float m-5 area shadow-lg">
            "Element "
            <BooleanDisplay
                value=is_visible
                true_str="inside"
                false_str="outside"
                class="font-bold"
            /> " the viewport"
        </div>
    }
}

fn main() {
    _ = console_log::init_with_level(log::Level::Debug);
    console_error_panic_hook::set_once();

    let _ = leptos::mount::mount_to(demo_or_body(), || {
        view! { <Demo/> }
    });
}
