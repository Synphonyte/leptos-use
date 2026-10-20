#![cfg(target_arch = "wasm32")]

use any_spawner::Executor;
use gloo_timers::future::TimeoutFuture;
use leptos::prelude::*;
use leptos_use_element_hover::{
    UseElementHoverOptions, use_element_hover, use_element_hover_with_options,
};
use wasm_bindgen_test::*;

wasm_bindgen_test_configure!(run_in_browser);

fn append_div() -> web_sys::Element {
    let el = document().create_element("div").unwrap();
    document().body().unwrap().append_child(&el).unwrap();
    el
}

fn fire(el: &web_sys::Element, name: &str) {
    el.dispatch_event(&web_sys::Event::new(name).unwrap())
        .unwrap();
}

#[wasm_bindgen_test]
async fn tracks_mouse_enter_and_leave() {
    let _ = Executor::init_wasm_bindgen();
    let owner = Owner::new();
    owner.set();

    let el = append_div();
    let is_hovered = use_element_hover(el.clone());
    Executor::tick().await;
    assert!(!is_hovered.get_untracked());

    fire(&el, "mouseenter");
    assert!(is_hovered.get_untracked());

    fire(&el, "mouseleave");
    assert!(!is_hovered.get_untracked());
}

#[wasm_bindgen_test]
async fn delays_the_state_change() {
    let _ = Executor::init_wasm_bindgen();
    let owner = Owner::new();
    owner.set();

    let el = append_div();
    let is_hovered = use_element_hover_with_options(
        el.clone(),
        UseElementHoverOptions::default()
            .delay_enter(60)
            .delay_leave(60),
    );
    Executor::tick().await;

    fire(&el, "mouseenter");
    assert!(!is_hovered.get_untracked(), "enter is delayed");
    TimeoutFuture::new(120).await;
    assert!(is_hovered.get_untracked());

    fire(&el, "mouseleave");
    assert!(is_hovered.get_untracked(), "leave is delayed");
    TimeoutFuture::new(120).await;
    assert!(!is_hovered.get_untracked());
}

// Leaving before the enter delay has elapsed cancels the pending enter.
#[wasm_bindgen_test]
async fn leaving_early_cancels_the_delayed_enter() {
    let _ = Executor::init_wasm_bindgen();
    let owner = Owner::new();
    owner.set();

    let el = append_div();
    let is_hovered = use_element_hover_with_options(
        el.clone(),
        UseElementHoverOptions::default().delay_enter(80),
    );
    Executor::tick().await;

    fire(&el, "mouseenter");
    TimeoutFuture::new(20).await;
    fire(&el, "mouseleave");
    TimeoutFuture::new(150).await;

    assert!(!is_hovered.get_untracked());
}
