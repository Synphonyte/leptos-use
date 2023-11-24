use leptos::*;
use leptos_use::docs::{demo_or_body, Note};
use leptos_use::use_display_media;

#[component]
fn Demo() -> impl IntoView {
    let stream = use_display_media(None);
    let video_ref = create_node_ref::<leptos::html::Video>();

    create_effect(move |_| match stream.get() {
        Some(Ok(s)) => {
            video_ref.get().expect("video element ref not created").set_src_object(Some(&s));
            video_ref.get().map(|v| v.play());
        }
        Some(Err(e)) => log::error!("Failed to get media stream: {:?}", e),
        None => log::debug!("No stream yet"),
    });

    view! { <video _ref=video_ref controls=true autoplay=true muted=true></video> }
}

fn main() {
    _ = console_log::init_with_level(log::Level::Debug);
    console_error_panic_hook::set_once();

    mount_to(demo_or_body(), || {
        view! { <Demo/> }
    })
}

