#![cfg(target_arch = "wasm32")]

use any_spawner::Executor;
use leptos::prelude::*;
use leptos_use_window_focus::use_window_focus;
use wasm_bindgen_test::*;

wasm_bindgen_test_configure!(run_in_browser);

fn fire(name: &str) {
    window()
        .dispatch_event(&web_sys::Event::new(name).unwrap())
        .unwrap();
}

#[wasm_bindgen_test]
async fn follows_window_focus_and_blur() {
    let _ = Executor::init_wasm_bindgen();
    let owner = Owner::new();
    owner.set();

    let focused = use_window_focus();
    assert_eq!(focused.get_untracked(), document().has_focus().unwrap());
    Executor::tick().await;

    fire("blur");
    assert!(!focused.get_untracked());

    fire("focus");
    assert!(focused.get_untracked());
}
