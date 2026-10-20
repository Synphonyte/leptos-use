use leptos::prelude::*;
use leptos_use_docs::demo_or_body;
use {{ crate_lib }}::{{ function_name }};

#[component]
fn Demo() -> impl IntoView {

    {{ function_name }}();

    view! {  }
}

fn main() {
    _ = console_log::init_with_level(log::Level::Debug);
    console_error_panic_hook::set_once();

    let unmount = mount_to(demo_or_body(), || {
        view! { <Demo /> }
    });
    unmount.forget();
}
