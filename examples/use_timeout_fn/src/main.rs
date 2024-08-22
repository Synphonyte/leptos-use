use leptos::prelude::*;
use leptos_use::docs::demo_or_body;
use leptos_use::{use_timeout_fn, UseTimeoutFnReturn};

#[component]
fn Demo() -> impl IntoView {
    const DEFAULT_TEXT: &str = "Please wait for 3 seconds";

    let (text, set_text) = signal(DEFAULT_TEXT.to_string());
    let UseTimeoutFnReturn {
        start, is_pending, ..
    } = use_timeout_fn(
        move |_| {
            set_text("Fired!".to_string());
        },
        3000.0,
    );

    let restart = move |_| {
        set_text(DEFAULT_TEXT.to_string());
        start(());
    };

    view! {
        <p>{text}</p>
        <button on:click=restart disabled=is_pending>"Restart"</button>
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
