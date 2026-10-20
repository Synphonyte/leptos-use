#![cfg(target_arch = "wasm32")]

use any_spawner::Executor;
use gloo_timers::future::TimeoutFuture;
use leptos::prelude::*;
use leptos_use_core::Size;
use leptos_use_element_size::{
    UseElementSizeOptions, UseElementSizeReturn, use_element_size, use_element_size_with_options,
};
use wasm_bindgen_test::*;

wasm_bindgen_test_configure!(run_in_browser);

fn append_box(style: &str) -> web_sys::Element {
    let el = document().create_element("div").unwrap();
    el.set_attribute("style", style).unwrap();
    document().body().unwrap().append_child(&el).unwrap();
    el
}

#[wasm_bindgen_test]
async fn measures_and_follows_the_element_size() {
    let _ = Executor::init_wasm_bindgen();
    let owner = Owner::new();
    owner.set();

    let el = append_box("width: 120px; height: 40px;");

    let UseElementSizeReturn { width, height } = use_element_size(el.clone());
    TimeoutFuture::new(100).await;

    assert_eq!(width.get_untracked(), 120.0);
    assert_eq!(height.get_untracked(), 40.0);

    el.set_attribute("style", "width: 200px; height: 75px;")
        .unwrap();
    TimeoutFuture::new(100).await;

    assert_eq!(width.get_untracked(), 200.0);
    assert_eq!(height.get_untracked(), 75.0);
}

#[wasm_bindgen_test]
async fn border_box_includes_padding_and_border() {
    let _ = Executor::init_wasm_bindgen();
    let owner = Owner::new();
    owner.set();

    let el = append_box("width: 100px; height: 50px; padding: 10px; border: 5px solid black;");

    let content = use_element_size(el.clone());
    let border = use_element_size_with_options(
        el.clone(),
        UseElementSizeOptions::default().box_(web_sys::ResizeObserverBoxOptions::BorderBox),
    );
    TimeoutFuture::new(100).await;

    assert_eq!(content.width.get_untracked(), 100.0);
    assert_eq!(border.width.get_untracked(), 130.0);
    assert_eq!(border.height.get_untracked(), 80.0);
}

#[wasm_bindgen_test]
async fn starts_with_the_initial_size_without_an_element() {
    let _ = Executor::init_wasm_bindgen();
    let owner = Owner::new();
    owner.set();

    let UseElementSizeReturn { width, height } = use_element_size_with_options(
        None::<web_sys::Element>,
        UseElementSizeOptions::default().initial_size(Size {
            width: 7.0,
            height: 3.0,
        }),
    );
    TimeoutFuture::new(50).await;

    assert_eq!(width.get_untracked(), 7.0);
    assert_eq!(height.get_untracked(), 3.0);
}
