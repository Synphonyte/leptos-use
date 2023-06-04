use leptos::*;
use leptos_use::docs::{demo_or_body, Note};
use leptos_use::{use_element_size, UseElementSizeReturn};

#[component]
fn Demo(cx: Scope) -> impl IntoView {
    let el = create_node_ref(cx);

    let UseElementSizeReturn { width, height } = use_element_size(cx, el);

    let text = move || format!("width: {}\nheight: {}", width.get(), height.get());

    view! { cx,
        <Note class="mb-2">"Resize the box to see changes"</Note>
        <textarea
            node_ref=el
            readonly
            class="resize rounded-md p-4 w-[200px] h-[100px] text-2xl leading-10"
            prop:value=text
        />
    }
}

fn main() {
    _ = console_log::init_with_level(log::Level::Debug);
    console_error_panic_hook::set_once();

    mount_to(demo_or_body(), |cx| {
        view! { cx, <Demo /> }
    })
}
