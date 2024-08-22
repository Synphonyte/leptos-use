use leptos::prelude::*;
use leptos_use::docs::demo_or_body;
use leptos_use::use_locales;

#[component]
fn Demo() -> impl IntoView {
    let locales = use_locales();

    view! {
        <pre>{move || format!("Locales:\n    {}", locales().join("\n    "))}</pre>
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
