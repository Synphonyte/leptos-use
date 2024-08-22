use leptos::html::Div;
use leptos::prelude::*;
use leptos_use::docs::{demo_or_body, Note};
use leptos_use::{
    use_mouse, use_mouse_with_options, UseMouseCoordType, UseMouseEventExtractor, UseMouseOptions,
};
use web_sys::{MouseEvent, Touch};

#[derive(Clone)]
struct Extractor;

impl UseMouseEventExtractor for Extractor {
    fn extract_mouse_coords(&self, event: &MouseEvent) -> Option<(f64, f64)> {
        Some((event.offset_x() as f64, event.offset_y() as f64))
    }

    // this is not necessary as it's the same as the default implementation of the trait.
    fn extract_touch_coords(&self, _touch: &Touch) -> Option<(f64, f64)> {
        // ignore touch events
        None
    }
}

#[component]
fn Demo() -> impl IntoView {
    let el = NodeRef::<Div>::new();

    let mouse_default = use_mouse();

    let mouse_with_extractor = use_mouse_with_options(
        UseMouseOptions::default()
            .target(el)
            .coord_type(UseMouseCoordType::Custom(Extractor)),
    );

    view! {
        <div node_ref=el>
            <p class="font-semibold">"Basic Usage"</p>
            <pre lang="yaml">
                {move || {
                    format!(
                        r#"    x: {}
    y: {}
    source_type: {:?}
"#,
                        mouse_default.x.get(),
                        mouse_default.y.get(),
                        mouse_default.source_type.get(),
                    )
                }}

            </pre>
            <p class="font-semibold">"Extractor Usage"</p>
            <Note>"Only works when the mouse is over the demo element"</Note>
            <pre lang="yaml">
                {move || {
                    format!(
                        r#"    x: {}
    y: {}
    source_type: {:?}
"#,
                        mouse_with_extractor.x.get(),
                        mouse_with_extractor.y.get(),
                        mouse_with_extractor.source_type.get(),
                    )
                }}

            </pre>
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
