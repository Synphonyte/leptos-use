use leptos::prelude::*;
use leptos_use::docs::{demo_or_body, BooleanDisplay};
use leptos_use::use_prefers_reduced_motion;

#[component]
fn Demo() -> impl IntoView {
    let is_reduced_motion_preferred = use_prefers_reduced_motion();

    view! {
        <div>
            <p>Prefers reduced motions: <BooleanDisplay value=is_reduced_motion_preferred/></p>
            <p>
                Update reduce motion preference
                <a href="https://developer.mozilla.org/en-US/docs/Web/CSS/@media/prefers-reduced-motion#user_preferences">
                    documentation.
                </a>
            </p>
        </div>
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
