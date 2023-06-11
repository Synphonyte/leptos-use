use leptos::*;
use leptos_use::docs::{demo_or_body, BooleanDisplay, Note};
use leptos_use::use_element_visibility;

#[component]
fn Demo(cx: Scope) -> impl IntoView {
    let el = create_node_ref(cx);

    let is_visible = use_element_visibility(cx, el);

    view! { cx,
        <div>
            <Note class="mb-4">"Info on the right bottom corner"</Note>
            <div node_ref=el class="max-w-lg relative area dark:bg-gray-800 shadow-lg z-60">
                "Target Element (scroll down)"
            </div>
        </div>
        <div class="float m-5 area shadow-lg">
            "Element "
            <BooleanDisplay value=is_visible true_str="inside" false_str="outside" class="font-bold"/>
            " the viewport"
        </div>
    }
}

fn main() {
    _ = console_log::init_with_level(log::Level::Debug);
    console_error_panic_hook::set_once();

    mount_to(demo_or_body(), |cx| {
        view! { cx, <Demo /> }
    })
}
