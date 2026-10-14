#![cfg(target_arch = "wasm32")]

use gloo_timers::future::TimeoutFuture;
use leptos_use_now::now;
use wasm_bindgen_test::*;

wasm_bindgen_test_configure!(run_in_browser);

// `now()` is the wall clock in milliseconds since the Unix epoch, like
// `Date.now()` in the browser.
#[wasm_bindgen_test]
fn matches_the_browser_wall_clock() {
    let before = js_sys::Date::now();
    let value = now();
    let after = js_sys::Date::now();

    assert!(
        before <= value && value <= after,
        "{before} <= {value} <= {after}"
    );
}

#[wasm_bindgen_test]
async fn advances_with_time() {
    let first = now();
    TimeoutFuture::new(30).await;
    let second = now();

    assert!(
        second - first >= 25.0,
        "expected at least 25ms, got {}",
        second - first
    );
}
