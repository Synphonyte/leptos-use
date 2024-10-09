use leptos::prelude::*;
use leptos::wasm_bindgen::JsCast;
use leptos_use::docs::{demo_or_body, Note};
use leptos_use::use_active_element;

#[component]
fn Demo() -> impl IntoView {
    let active_element = use_active_element();
    let key = move || {
        format!(
            "{:?}",
            active_element
                .get()
                .map(|el| el
                    .unchecked_ref::<web_sys::HtmlElement>()
                    .dataset()
                    .get("id"))
                .unwrap_or_default()
        )
    };

    view! {
        <Note class="mb-3">"Select the inputs below to see the changes"</Note>

        <div class="grid grid-cols-1 md:grid-cols-3 gap-2">
            <For each=move || (1..7) key=|i| *i let:i>
                <input type="text" data-id=i class="!my-0 !min-w-0" placeholder=i/>
            </For>

        </div>

        <div class="mt-2">"Current Active Element: " <span class="text-primary">{key}</span></div>
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
