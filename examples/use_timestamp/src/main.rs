use leptos::prelude::*;
use leptos_use_docs::demo_or_body;
use leptos_use_timestamp::use_timestamp;

#[component]
fn Demo() -> impl IntoView {
    let timestamp = use_timestamp();

    view! { <div>Timestamp: {timestamp}</div> }
}

fn main() {
    _ = console_log::init_with_level(log::Level::Debug);
    console_error_panic_hook::set_once();

    let unmount_handle = leptos::mount::mount_to(demo_or_body(), || {
        view! { <Demo/> }
    });

    unmount_handle.forget();
}
