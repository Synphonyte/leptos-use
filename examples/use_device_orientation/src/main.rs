use leptos::prelude::*;
use leptos_use::docs::demo_or_body;
use leptos_use::use_device_orientation;

#[component]
fn Demo() -> impl IntoView {
    let orientation = use_device_orientation();

    view! {
    <pre>
        {move || format!(
            concat!(
                "is_supported: {}\n",
                "absolute: {}\n",
                "alpha: {:?}\n",
                "beta: {:?}\n",
                "gamma: {:?}\n",
            ),
            orientation.is_supported.get(),
            orientation.absolute.get(),
            orientation.alpha.get(),
            orientation.beta.get(),
            orientation.gamma.get(),
        )}
    </pre> }
}

fn main() {
    _ = console_log::init_with_level(log::Level::Debug);
    console_error_panic_hook::set_once();

    let _ = leptos::mount::mount_to(demo_or_body(), || {
        view! { <Demo/> }
    });
}
