use leptos::prelude::*;
use leptos_use::docs::{demo_or_body, BooleanDisplay};
use leptos_use::{
    use_web_notification_with_options, NotificationDirection, ShowOptions,
    UseWebNotificationOptions, UseWebNotificationReturn,
};

#[component]
fn Demo() -> impl IntoView {
    let UseWebNotificationReturn {
        is_supported, show, ..
    } = use_web_notification_with_options(
        UseWebNotificationOptions::default()
            .title("Hello World from leptos-use")
            .direction(NotificationDirection::Auto)
            .language("en")
            .renotify(true)
            .tag("test"),
    );

    let show = move || {
        show(ShowOptions::default());
    };

    view! {
        <div>
            <p>Supported: <BooleanDisplay value=is_supported/></p>
        </div>

        <Show
            when=is_supported
            fallback=|| {
                view! { <div>The Notification Web API is not supported in your browser.</div> }
            }
        >

            <button on:click={
                let show = show.clone();
                move |_| show()
            }>Show Notification</button>
        </Show>
    }
}

fn main() {
    _ = console_log::init_with_level(log::Level::Debug);
    console_error_panic_hook::set_once();

    let _ = leptos::mount::mount_to(demo_or_body(), || {
        view! { <Demo/> }
    });
}
