#![cfg(target_arch = "wasm32")]

use any_spawner::Executor;
use gloo_timers::future::TimeoutFuture;
use leptos::prelude::*;
use leptos::wasm_bindgen::JsCast;
use leptos_use_active_element::use_active_element;
use wasm_bindgen_test::*;

wasm_bindgen_test_configure!(run_in_browser);

fn append_input(id: &str) -> web_sys::HtmlElement {
    let el = document().create_element("input").unwrap();
    el.set_id(id);
    document().body().unwrap().append_child(&el).unwrap();
    el.unchecked_into()
}

#[wasm_bindgen_test]
async fn follows_focus_changes() {
    let _ = Executor::init_wasm_bindgen();
    let owner = Owner::new();
    owner.set();

    let first = append_input("active-element-first");
    let second = append_input("active-element-second");

    let active = use_active_element();
    Executor::tick().await;

    first.focus().unwrap();
    TimeoutFuture::new(20).await;
    assert_eq!(
        active.read_untracked().as_ref().map(|el| el.id()),
        Some("active-element-first".to_string())
    );

    second.focus().unwrap();
    TimeoutFuture::new(20).await;
    assert_eq!(
        active.read_untracked().as_ref().map(|el| el.id()),
        Some("active-element-second".to_string())
    );

    second.blur().unwrap();
    TimeoutFuture::new(20).await;
    assert_eq!(
        active.read_untracked().as_ref().map(|el| el.tag_name()),
        Some("BODY".to_string()),
        "blurring moves the focus back to the body"
    );
}

#[wasm_bindgen_test]
async fn starts_with_the_currently_focused_element() {
    let _ = Executor::init_wasm_bindgen();
    let owner = Owner::new();
    owner.set();

    let input = append_input("active-element-initial");
    input.focus().unwrap();

    let active = use_active_element();

    assert_eq!(
        active.read_untracked().as_ref().map(|el| el.id()),
        Some("active-element-initial".to_string())
    );
    input.blur().unwrap();
}
