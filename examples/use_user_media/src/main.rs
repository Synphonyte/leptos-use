use leptos::prelude::*;
use leptos_use::docs::demo_or_body;
use leptos_use::{use_user_media, UseUserMediaReturn};

#[component]
fn Demo() -> impl IntoView {
    let video_ref = NodeRef::<leptos::html::Video>::new();

    let UseUserMediaReturn {
        stream,
        enabled,
        set_enabled,
        ..
    } = use_user_media();

    Effect::new(move |_| {
        match stream.get() {
            Some(Ok(s)) => {
                video_ref.with(|v| {
                    if let Some(v) = v {
                        v.set_src_object(Some(&s));
                    }
                });
                return;
            }
            Some(Err(e)) => leptos::logging::error!("Failed to get media stream: {:?}", e),
            None => leptos::logging::log!("No stream yet"),
        }

        video_ref.with(|v| {
            if let Some(v) = v {
                v.set_src_object(None);
            }
        });
    });

    view! {
        <div class="flex flex-col gap-4 text-center">
            <div>
                <button on:click=move |_| set_enabled(
                    !enabled(),
                )>{move || if enabled() { "Stop" } else { "Start" }} video</button>
            </div>

            <div>
                <video
                    node_ref=video_ref
                    controls=false
                    autoplay=true
                    muted=true
                    class="h-96 w-auto"
                ></video>
            </div>
        </div>
    }
}

fn main() {
    _ = console_log::init_with_level(log::Level::Debug);
    console_error_panic_hook::set_once();

    let _ = leptos::mount::mount_to(demo_or_body(), || {
        view! { <Demo/> }
    });
}
