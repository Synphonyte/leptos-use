#![cfg(target_arch = "wasm32")]

use any_spawner::Executor;
use gloo_timers::future::TimeoutFuture;
use leptos::prelude::*;
use leptos_use_element_bounding::{
    UseElementBoundingOptions, UseElementBoundingReturn, use_element_bounding,
    use_element_bounding_with_options,
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
async fn reports_the_bounding_rect() {
    let _ = Executor::init_wasm_bindgen();
    let owner = Owner::new();
    owner.set();

    let el = append_box("position: fixed; left: 30px; top: 20px; width: 100px; height: 50px;");

    let UseElementBoundingReturn {
        x,
        y,
        top,
        left,
        right,
        bottom,
        width,
        height,
        ..
    } = use_element_bounding(el.clone());
    TimeoutFuture::new(100).await;

    assert_eq!(x.get_untracked(), 30.0);
    assert_eq!(y.get_untracked(), 20.0);
    assert_eq!(left.get_untracked(), 30.0);
    assert_eq!(top.get_untracked(), 20.0);
    assert_eq!(right.get_untracked(), 130.0);
    assert_eq!(bottom.get_untracked(), 70.0);
    assert_eq!(width.get_untracked(), 100.0);
    assert_eq!(height.get_untracked(), 50.0);
}

#[wasm_bindgen_test]
async fn follows_resizes() {
    let _ = Executor::init_wasm_bindgen();
    let owner = Owner::new();
    owner.set();

    let el = append_box("position: fixed; left: 0; top: 0; width: 10px; height: 10px;");

    let UseElementBoundingReturn { width, right, .. } = use_element_bounding(el.clone());
    TimeoutFuture::new(100).await;
    assert_eq!(width.get_untracked(), 10.0);

    el.set_attribute(
        "style",
        "position: fixed; left: 0; top: 0; width: 60px; height: 10px;",
    )
    .unwrap();
    TimeoutFuture::new(100).await;

    assert_eq!(width.get_untracked(), 60.0);
    assert_eq!(right.get_untracked(), 60.0);
}

// Moving an element doesn't resize it, so the values only change on `update()`.
#[wasm_bindgen_test]
async fn update_remeasures_on_demand() {
    let _ = Executor::init_wasm_bindgen();
    let owner = Owner::new();
    owner.set();

    let el = append_box("position: fixed; left: 0; top: 0; width: 10px; height: 10px;");

    let UseElementBoundingReturn { left, update, .. } = use_element_bounding_with_options(
        el.clone(),
        UseElementBoundingOptions::default()
            .window_scroll(false)
            .window_resize(false),
    );
    TimeoutFuture::new(100).await;
    assert_eq!(left.get_untracked(), 0.0);

    el.set_attribute(
        "style",
        "position: fixed; left: 45px; top: 0; width: 10px; height: 10px;",
    )
    .unwrap();
    TimeoutFuture::new(50).await;
    assert_eq!(left.get_untracked(), 0.0);

    update();
    assert_eq!(left.get_untracked(), 45.0);
}

#[wasm_bindgen_test]
async fn is_zero_without_an_element() {
    let _ = Executor::init_wasm_bindgen();
    let owner = Owner::new();
    owner.set();

    let UseElementBoundingReturn { width, top, .. } =
        use_element_bounding(None::<web_sys::Element>);
    TimeoutFuture::new(50).await;

    assert_eq!(width.get_untracked(), 0.0);
    assert_eq!(top.get_untracked(), 0.0);
}
