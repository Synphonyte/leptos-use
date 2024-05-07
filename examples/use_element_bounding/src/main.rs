use leptos::html::Textarea;
use leptos::prelude::*;
use leptos_use::docs::{demo_or_body, Note};
use leptos_use::{use_element_bounding, UseElementBoundingReturn};

#[component]
fn Demo() -> impl IntoView {
    let el = create_node_ref::<Textarea>();

    let UseElementBoundingReturn {
        width,
        height,
        left,
        right,
        top,
        bottom,
        x,
        y,
        ..
    } = use_element_bounding(el);

    let text = move || {
        format!(
            "width: {}\nheight: {}\nleft: {}\nright: {}\ntop: {}\nbottom: {}\nx: {}\ny: {}",
            width.get(),
            height.get(),
            left.get(),
            right.get(),
            top.get(),
            bottom.get(),
            x.get(),
            y.get()
        )
    };

    view! {
        <Note class="mb-2">Resize the box to see changes</Note>
        <textarea
            node_ref=el
            readonly
            class="resize rounded-md p-4 w-[335px] h-[175px] text-2xl leading-10"
            prop:value=text
        ></textarea>
    }
}

fn main() {
    _ = console_log::init_with_level(log::Level::Debug);
    console_error_panic_hook::set_once();

    mount_to(demo_or_body(), || {
        view! { <Demo/> }
    })
}
