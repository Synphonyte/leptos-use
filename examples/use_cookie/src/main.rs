use leptos::*;
use leptos_use::docs::demo_or_body;
use leptos_use::use_cookie;

#[component]
fn Demo() -> impl IntoView {
    if let Some(cookie) = use_cookie("auth") {
        view! { <div>"'auth' cookie set to " <code>"`" {cookie.value().to_string()} "`"</code></div> }
        .into_view()
    } else {
        view! { <div>"No 'auth' cookie set"</div> }
        .into_view()
    }
}

fn main() {
    _ = console_log::init_with_level(log::Level::Debug);
    console_error_panic_hook::set_once();

    mount_to(demo_or_body(), || {
        view! { <Demo/> }
    })
}
