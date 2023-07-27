use leptos::html::Textarea;
use leptos::*;
use leptos_use::docs::{demo_or_body, Note};
use leptos_use::use_resize_observer;

#[component]
fn Demo() -> impl IntoView {
    let el = create_node_ref::<Textarea>();
    let (text, set_text) = create_signal("".to_string());

    use_resize_observer(el, move |entries, _| {
        let rect = entries[0].content_rect();
        set_text.set(format!(
            "width: {:.0}\nheight: {:.0}",
            rect.width(),
            rect.height()
        ));
    });

    view! {
        <Note class="mb-2">"Resize the box to see changes"</Note>
        <textarea
            node_ref=el
            readonly
            class="resize rounded-md p-4 w-[200px] h-[100px] text-2xl leading-10"
            prop:value=move || text.get()
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
