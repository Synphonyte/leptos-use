use leptos::prelude::*;
use leptos_use::docs::demo_or_body;
use leptos_use::use_device_pixel_ratio;

#[component]
fn Demo() -> impl IntoView {
    let pixel_ratio = use_device_pixel_ratio();

    view! {
        <pre>{move || format!("pixelRatio: {}", pixel_ratio())}</pre>
        <p>
            "Zoom in and out (or move the window to a screen with a different scaling factor) to see the value changes."
        </p>
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
