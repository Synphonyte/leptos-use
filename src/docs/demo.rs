use leptos::prelude::*;
use wasm_bindgen::JsCast;

pub fn demo_or_body() -> web_sys::HtmlElement {
    document()
        .get_element_by_id("demo-anchor")
        .map(|e| e.unchecked_into::<web_sys::HtmlElement>())
        .unwrap_or(document().body().expect("body to exist"))
}
