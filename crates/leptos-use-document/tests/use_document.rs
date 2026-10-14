#![cfg(target_arch = "wasm32")]

use leptos_use_document::use_document;
use wasm_bindgen_test::*;

wasm_bindgen_test_configure!(run_in_browser);

#[wasm_bindgen_test]
fn returns_the_browser_document() {
    let document = use_document();

    let expected = web_sys::window().unwrap().document();
    assert_eq!(document.as_ref(), expected.as_ref());
    assert!(document.body().is_some());
}

#[wasm_bindgen_test]
fn queries_elements() {
    let document = use_document();

    let el = document
        .as_ref()
        .unwrap()
        .create_element("section")
        .unwrap();
    el.set_id("use-document-query-target");
    document.body().unwrap().append_child(&el).unwrap();

    let found = document
        .query_selector("#use-document-query-target")
        .unwrap()
        .expect("element is found");
    assert_eq!(found, el);

    assert!(
        document
            .query_selector("#use-document-missing")
            .unwrap()
            .is_none()
    );
}

#[wasm_bindgen_test]
fn reports_the_active_element() {
    let document = use_document();

    // nothing is focused, so the body is the active element
    let active = document.active_element().expect("an active element");
    assert_eq!(active.tag_name(), "BODY");
}
