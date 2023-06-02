use leptos::*;
use leptos_use::docs::{demo_or_body, Note};
use leptos_use::use_resize_observer;

#[component]
fn Demo(cx: Scope) -> impl IntoView {
    let el = create_node_ref(cx);
    let (text, set_text) = create_signal(cx, "".to_string());

    use_resize_observer(cx, el, move |entries, _| {
        let rect = entries[0].content_rect();
        set_text(format!(
            "width: {:.0}\nheight: {:.0}",
            rect.width(),
            rect.height()
        ));
    });

    view! { cx,
        <Note class="mb-2">"Resize the box to see changes"</Note>
        <textarea node_ref=el class="resize rounded-md p-4 w-[200px] h-[100px] text-2xl leading-10" disabled prop:value=text />
    }
}

fn main() {
    _ = console_log::init_with_level(log::Level::Debug);
    console_error_panic_hook::set_once();

    mount_to(demo_or_body(), |cx| {
        view! { cx, <Demo /> }
    })
}
