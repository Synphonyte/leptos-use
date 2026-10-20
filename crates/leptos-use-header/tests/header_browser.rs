#![cfg(target_arch = "wasm32")]

use leptos_use_header::get_header;
use wasm_bindgen_test::*;

wasm_bindgen_test_configure!(run_in_browser);

fn color_scheme_header() -> Option<String> {
    get_header!(
        "sec-ch-prefers-color-scheme",
        color_scheme_header,
        header_getter
    )
}

// There are no request headers in the browser.
#[wasm_bindgen_test]
fn get_header_is_none_in_the_browser() {
    assert_eq!(color_scheme_header(), None);
}
