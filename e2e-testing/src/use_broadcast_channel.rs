use codee::string::FromToStringCodec;
use leptos::{prelude::*, web_sys};
use leptos_use::{use_broadcast_channel, UseBroadcastChannelReturn};

/// E2E test component for use_broadcast_channel
#[component]
pub fn BroadcastChannelDemo() -> impl IntoView {
    let UseBroadcastChannelReturn {
        is_supported,
        message,
        post,
        error,
        channel,
        ..
    } = use_broadcast_channel::<String, FromToStringCodec>("leptos-use-e2e-testing-channel");

    let (input_value, set_input_value) = signal(String::new());
    
    view! {
        <h2>Broadcast Channel E2E Test</h2>
        <p>Please open this page in at least two tabs</p>
        <Show
            when=move || is_supported.get()
            fallback=move || view! { <p>"BroadcastChannel not supported"</p> }
        >
            <form on:submit={
                let post = post.clone();
                move |ev: web_sys::SubmitEvent| {
                    ev.prevent_default();
                    let value = input_value.get();
                    post(&value);
                }
            }>
                <input
                    value=input_value
                    on:input=move |event| {
                        set_input_value.set(event_target_value(&event));
                    }
                    type="text"
                />
                <button type="submit">Send Message</button>
            </form>
            <Show when=move || message.get().is_some()>
                <p>"Received message: " {move || message.get().as_ref().unwrap().to_string()}</p>
            </Show>
            <Show when=move || error.with(|e| e.is_some())>
                <p>"Error: " {move || error.with(|e| format!("{:?}", e.as_ref().unwrap()))}</p>
            </Show>
        </Show>
    }
}
