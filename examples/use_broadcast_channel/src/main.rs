use codee::string::FromToStringCodec;
use leptos::prelude::*;
use leptos_use::docs::demo_or_body;
use leptos_use::{UseBroadcastChannelReturn, use_broadcast_channel};

#[component]
fn Demo() -> impl IntoView {
    let UseBroadcastChannelReturn {
        is_supported,
        message,
        post,
        error,
        ..
    } = use_broadcast_channel::<String, FromToStringCodec>("leptos-use-demo-channel");

    let (input_value, set_input_value) = signal(String::new());

    view! {
        <p>Please open this page in at least two tabs</p>

        <Show
            when=move || is_supported()
            fallback=move || view! { <p>"BroadcastChannel not supported"</p> }
        >
            <form on:submit={
                let post = post.clone();

                move |ev: web_sys::SubmitEvent| {
                    ev.prevent_default();
                    let value = input_value();
                    post(&value);
                }
            }>
                <input
                    value=input_value
                    on:input=move |event| {
                        set_input_value(event_target_value(&event));
                    }

                    type="text"
                />
                <button type="submit">Send Message</button>
            </form>

            <ShowLet some=move || message.get() let:message>
                <p>"Received message: " {message}</p>
            </ShowLet>

            <ShowLet some=move || error.get() let:error>
                <p>"Error: " {format!("{error:?}")}</p>
            </ShowLet>
        </Show>
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
