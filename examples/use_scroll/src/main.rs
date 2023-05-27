use leptos::*;
use leptos_use::utils::demo_or_body;

#[component]
fn Demo(cx: Scope) -> impl IntoView {
    view! { cx,

    }
}

fn main() {
    _ = console_log::init_with_level(log::Level::Debug);
    console_error_panic_hook::set_once();

    mount_to(demo_or_body(), |cx| {
        view! { cx, <Demo /> }
    })
}
