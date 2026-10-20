#![cfg(target_arch = "wasm32")]

use any_spawner::Executor;
use gloo_timers::future::TimeoutFuture;
use leptos::prelude::*;
use leptos_use_element_visibility::use_element_visibility;
use wasm_bindgen_test::*;

wasm_bindgen_test_configure!(run_in_browser);

fn append_box(style: &str) -> web_sys::Element {
    let el = document().create_element("div").unwrap();
    el.set_attribute("style", style).unwrap();
    document().body().unwrap().append_child(&el).unwrap();
    el
}

#[wasm_bindgen_test]
async fn follows_whether_the_element_is_in_the_viewport() {
    let _ = Executor::init_wasm_bindgen();
    let owner = Owner::new();
    owner.set();

    let el = append_box("position: fixed; left: 0; top: 0; width: 20px; height: 20px;");

    let is_visible = use_element_visibility(el.clone());
    TimeoutFuture::new(150).await;
    assert!(is_visible.get_untracked());

    el.set_attribute(
        "style",
        "position: fixed; left: -500px; top: 0; width: 20px; height: 20px;",
    )
    .unwrap();
    TimeoutFuture::new(150).await;
    assert!(!is_visible.get_untracked());

    el.set_attribute(
        "style",
        "position: fixed; left: 10px; top: 10px; width: 20px; height: 20px;",
    )
    .unwrap();
    TimeoutFuture::new(150).await;
    assert!(is_visible.get_untracked());
}

#[wasm_bindgen_test]
async fn an_element_outside_the_viewport_is_not_visible() {
    let _ = Executor::init_wasm_bindgen();
    let owner = Owner::new();
    owner.set();

    let el = append_box("position: fixed; left: 0; top: 10000px; width: 20px; height: 20px;");

    let is_visible = use_element_visibility(el);
    TimeoutFuture::new(150).await;

    assert!(!is_visible.get_untracked());
}
