#![cfg(target_arch = "wasm32")]

use any_spawner::Executor;
use leptos::prelude::*;
use leptos_use_supported::use_supported;
use wasm_bindgen_test::*;
use wasm_js_macros::js;

wasm_bindgen_test_configure!(run_in_browser);

#[wasm_bindgen_test]
async fn is_false_until_checked_in_the_browser() {
    let _ = Executor::init_wasm_bindgen();
    let owner = Owner::new();
    owner.set();

    let is_supported = use_supported(|| js!("document" in &window()));
    assert!(!is_supported.get_untracked(), "the check runs in an effect");

    Executor::tick().await;
    assert!(is_supported.get_untracked());
}

#[wasm_bindgen_test]
async fn reports_missing_apis() {
    let _ = Executor::init_wasm_bindgen();
    let owner = Owner::new();
    owner.set();

    let is_supported = use_supported(|| js!("definitelyNotABrowserApi" in &window()));
    Executor::tick().await;

    assert!(!is_supported.get_untracked());
}

#[wasm_bindgen_test]
async fn rechecks_when_tracked_signals_change() {
    let _ = Executor::init_wasm_bindgen();
    let owner = Owner::new();
    owner.set();

    let (api, set_api) = signal("definitelyNotABrowserApi");
    let is_supported = use_supported(move || js!([api.get()] in &window()));
    Executor::tick().await;
    assert!(!is_supported.get_untracked());

    set_api.set("navigator");
    Executor::tick().await;
    assert!(is_supported.get_untracked());
}
