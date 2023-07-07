use leptos::*;
use leptos_use::docs::demo_or_body;
use leptos_use::{use_document_visibility, watch};
use std::time::Duration;

#[component]
fn Demo(cx: Scope) -> impl IntoView {
    let start_message = "💡 Minimize the page or switch tab then return";
    let (message, set_message) = create_signal(cx, start_message);
    let visibility = use_document_visibility(cx);

    let _ = watch(cx, visibility, move |cur, prev, _| {
        if let Some(prev) = prev {
            if *cur == web_sys::VisibilityState::Visible
                && *prev == web_sys::VisibilityState::Hidden
            {
                set_message("🎉 Welcome back!");

                set_timeout(
                    move || {
                        set_message(start_message);
                    },
                    Duration::from_millis(3000),
                )
            }
        }
    });

    view! { cx,
        <div>{ message }</div>
    }
}

fn main() {
    _ = console_log::init_with_level(log::Level::Debug);
    console_error_panic_hook::set_once();

    mount_to(demo_or_body(), |cx| {
        view! { cx, <Demo /> }
    })
}
