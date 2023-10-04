use leptos::*;
use leptos_use::docs::demo_or_body;
use leptos_use::{use_service_worker, use_window};
use web_sys::HtmlMetaElement;

#[component]
fn Demo() -> impl IntoView {
    let build = load_meta_element("version")
        .map(|meta| meta.content())
        .expect("'version' meta element");

    let sw = use_service_worker();

    view! {
        <p>"Current build: "{build}</p>

        <br/>

        <p>"registration: "{move || format!("{:#?}", sw.registration.get())}</p>
        <p>"installing: "{move || sw.installing.get()}</p>
        <p>"waiting: "{move || sw.waiting.get()}</p>
        <p>"active: "{move || sw.active.get()}</p>

        <br/>

        <button on:click=move |_| {sw.skip_waiting.call(())}>"Send skipWaiting event"</button>
    }
}

fn main() {
    _ = console_log::init_with_level(log::Level::Debug);
    console_error_panic_hook::set_once();

    mount_to(demo_or_body(), || {
        view! { <Demo/> }
    })
}

fn load_meta_element<S: AsRef<str>>(name: S) -> Result<web_sys::HtmlMetaElement, String> {
    use wasm_bindgen::JsCast;
    use_window()
        .as_ref()
        .ok_or_else(|| "No window instance!".to_owned())
        .and_then(|window| {
            window
                .document()
                .ok_or_else(|| "No document instance!".to_owned())
        })
        .and_then(|document| {
            document
                .query_selector(format!("meta[name=\"{}\"]", name.as_ref()).as_str())
                .ok()
                .flatten()
                .ok_or_else(|| format!("Unable to find meta element with name 'version'."))
        })
        .and_then(|element| {
            element.dyn_into::<HtmlMetaElement>().map_err(|err| {
                format!("Unable to cast element to HtmlMetaElement. Err: '{err:?}'.")
            })
        })
}
