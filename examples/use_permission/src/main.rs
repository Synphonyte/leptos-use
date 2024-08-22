use leptos::prelude::*;
use leptos_use::docs::demo_or_body;
use leptos_use::use_permission;

#[component]
fn Demo() -> impl IntoView {
    let accelerometer = use_permission("accelerometer");
    let accessibility_events = use_permission("accessibility-events");
    let ambient_light_sensor = use_permission("ambient-light-sensor");
    let background_sync = use_permission("background-sync");
    let camera = use_permission("camera");
    let clipboard_read = use_permission("clipboard-read");
    let clipboard_write = use_permission("clipboard-write");
    let gyroscope = use_permission("gyroscope");
    let magnetometer = use_permission("magnetometer");
    let microphone = use_permission("microphone");
    let notifications = use_permission("notifications");
    let payment_handler = use_permission("payment-handler");
    let persistent_storage = use_permission("persistent-storage");
    let push = use_permission("push");
    let speaker = use_permission("speaker");

    view! {
        <pre>
            <>
            "accelerometer: " {move || accelerometer().to_string()}
            "\naccessibility_events: " {move || accessibility_events().to_string()}
            "\nambient_light_sensor: " {move || ambient_light_sensor().to_string()}
            "\nbackground_sync: " {move || background_sync().to_string()}
            "\ncamera: " {move || camera().to_string()}
            "\nclipboard_read: " {move || clipboard_read().to_string()}
            "\nclipboard_write: " {move || clipboard_write().to_string()}
            "\ngyroscope: " {move || gyroscope().to_string()}
            </><>
            "\nmagnetometer: " {move || magnetometer().to_string()}
            "\nmicrophone: " {move || microphone().to_string()}
            "\nnotifications: " {move || notifications().to_string()}
            "\npayment_handler: " {move || payment_handler().to_string()}
            "\npersistent_storage: " {move || persistent_storage().to_string()}
            "\npush: " {move || push().to_string()}
            "\nspeaker: " {move || speaker().to_string()}
            </>
        </pre>
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
