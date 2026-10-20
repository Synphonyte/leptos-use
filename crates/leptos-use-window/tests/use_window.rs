#![cfg(target_arch = "wasm32")]

use leptos_use_window::use_window;
use wasm_bindgen_test::*;

wasm_bindgen_test_configure!(run_in_browser);

#[wasm_bindgen_test]
fn returns_the_browser_window() {
    let window = use_window();

    assert!(window.is_some());
    assert_eq!(window.as_ref(), web_sys::window().as_ref());
}

#[wasm_bindgen_test]
fn exposes_navigator_and_document() {
    let window = use_window();

    assert!(window.navigator().is_some());

    let document = window.document();
    assert!(document.body().is_some());
}

#[wasm_bindgen_test]
fn matches_media_queries() {
    let window = use_window();

    let all = window
        .match_media("all")
        .unwrap()
        .expect("a media query list");
    assert!(all.matches());

    let none = window
        .match_media("not all")
        .unwrap()
        .expect("a media query list");
    assert!(!none.matches());
}
