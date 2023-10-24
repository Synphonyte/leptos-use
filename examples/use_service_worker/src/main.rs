use leptos::*;
use leptos_use::docs::{demo_or_body, BooleanDisplay};
use leptos_use::{use_document, use_service_worker, UseServiceWorkerReturn};
use web_sys::HtmlMetaElement;

#[component]
fn Demo() -> impl IntoView {
    let build = load_meta_element("version")
        .map(|meta| meta.content())
        .expect("'version' meta element");

    let UseServiceWorkerReturn {
        registration,
        installing,
        waiting,
        active,
        skip_waiting,
        ..
    } = use_service_worker();

    view! {
        <p>"Current build: " {build}</p>

        <br/>

        <p>"registration: " {move || format!("{:#?}", registration())}</p>
        <p>"installing: " <BooleanDisplay value=installing/></p>
        <p>"waiting: " <BooleanDisplay value=waiting/></p>
        <p>"active: " <BooleanDisplay value=active/></p>

        <br/>

        <button on:click=move |_| { skip_waiting() }>"Send skip_waiting event"</button>
    }
}

fn main() {
    _ = console_log::init_with_level(log::Level::Debug);
    console_error_panic_hook::set_once();

    mount_to(demo_or_body(), || {
        view! { <Demo/> }
    })
}

fn load_meta_element(name: &str) -> Result<web_sys::HtmlMetaElement, String> {
    use wasm_bindgen::JsCast;
    if let Some(document) = &*use_document() {
        document
            .query_selector(format!("meta[name=\"{name}\"]").as_str())
            .ok()
            .flatten()
            .ok_or_else(|| format!("Unable to find meta element with name '{name}'."))?
            .dyn_into::<HtmlMetaElement>()
            .map_err(|err| format!("Unable to cast element to HtmlMetaElement. Err: '{err:?}'."))
    } else {
        Err("Unable to find document.".into())
    }
}
