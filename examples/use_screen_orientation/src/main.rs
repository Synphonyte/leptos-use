use leptos::prelude::*;
use leptos_use::docs::{demo_or_body, Note};
use leptos_use::{use_screen_orientation, ScreenOrientation, UseScreenOrientationReturn};

#[component]
fn Demo() -> impl IntoView {
    let UseScreenOrientationReturn {
        orientation, angle, ..
    } = use_screen_orientation();

    view! {
        <Note class="mb-2">
            "For best results, please use a mobile or tablet device (or use your browser's native inspector to simulate an
            orientation change)"
        </Note>

        <div>
            "Orientation Type: "
            <b>
                {move || match orientation.get() {
                    ScreenOrientation::PortraitPrimary => "PortraitPrimary",
                    ScreenOrientation::LandscapePrimary => "LandscapePrimary",
                    ScreenOrientation::PortraitSecondary => "PortraitSecondary",
                    ScreenOrientation::LandscapeSecondary => "LandscapeSecondary",
                }}
            </b>
        </div>
        <div>"Orientation Angle: "<b>{move || angle.get().to_string()}</b></div>
    }
}

fn main() {
    _ = console_log::init_with_level(log::Level::Debug);
    console_error_panic_hook::set_once();

    let unmount = mount_to(demo_or_body(), || {
        view! { <Demo /> }
    });
    unmount.forget();
}
