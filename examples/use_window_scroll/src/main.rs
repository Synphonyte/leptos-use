use leptos::prelude::*;
use leptos_use::docs::{demo_or_body, Note};
use leptos_use::use_window_scroll;

#[component]
fn Demo() -> impl IntoView {
    let (x, y) = use_window_scroll();

    let div = document().create_element("div").unwrap();
    div.set_attribute(
        "style",
        "position: absolute; top: 100%; left: 100%; width: 10000px; height: 10000px;",
    )
    .unwrap();

    document().body().unwrap().append_child(&div).unwrap();

    view! {
        <div>See scroll values in the lower right corner of the screen.</div>
        <div class="float m-5 area shadow-lg">
            <Note class="mb-2">Scroll value</Note>
            <div>x: {move || format!("{:.1}", x())} <br/> y: {move || format!("{:.1}", y())}</div>
        </div>
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
