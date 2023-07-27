use leptos::*;
use leptos_use::docs::{demo_or_body, BooleanDisplay};
use leptos_use::use_media_query;

#[component]
fn Demo() -> impl IntoView {
    let is_large_screen = use_media_query("(min-width: 1024px)");
    let is_dark_preferred = use_media_query("(prefers-color-scheme: dark)");

    view! {
        <p>"Is large screen: " <BooleanDisplay value=is_large_screen/></p>
        <p>"Is dark preferred: " <BooleanDisplay value=is_dark_preferred/></p>
    }
}

fn main() {
    _ = console_log::init_with_level(log::Level::Debug);
    console_error_panic_hook::set_once();

    mount_to(demo_or_body(), || {
        view! { <Demo/> }
    })
}
