#![cfg(target_arch = "wasm32")]

use any_spawner::Executor;
use gloo_timers::future::TimeoutFuture;
use leptos::prelude::*;
use leptos_use_window_size::{
    MeasureType, UseWindowSizeOptions, UseWindowSizeReturn, use_window_size,
    use_window_size_with_options,
};
use wasm_bindgen_test::*;

wasm_bindgen_test_configure!(run_in_browser);

fn inner_size() -> (f64, f64) {
    (
        window().inner_width().unwrap().as_f64().unwrap(),
        window().inner_height().unwrap().as_f64().unwrap(),
    )
}

#[wasm_bindgen_test]
async fn reports_the_inner_window_size() {
    let _ = Executor::init_wasm_bindgen();
    let owner = Owner::new();
    owner.set();

    let UseWindowSizeReturn { width, height } = use_window_size();
    TimeoutFuture::new(50).await;

    assert!(width.get_untracked() > 0.0);
    assert_eq!(
        (width.get_untracked(), height.get_untracked()),
        inner_size()
    );
}

#[wasm_bindgen_test]
async fn excludes_the_scrollbar_when_asked() {
    let _ = Executor::init_wasm_bindgen();
    let owner = Owner::new();
    owner.set();

    let UseWindowSizeReturn { width, height } =
        use_window_size_with_options(UseWindowSizeOptions::default().include_scrollbar(false));
    TimeoutFuture::new(50).await;

    let root = document().document_element().unwrap();
    assert_eq!(width.get_untracked(), root.client_width() as f64);
    assert_eq!(height.get_untracked(), root.client_height() as f64);
}

#[wasm_bindgen_test]
async fn reports_the_outer_window_size() {
    let _ = Executor::init_wasm_bindgen();
    let owner = Owner::new();
    owner.set();

    let UseWindowSizeReturn { width, height } = use_window_size_with_options(
        UseWindowSizeOptions::default().measure_type(MeasureType::Outer),
    );
    TimeoutFuture::new(50).await;

    assert_eq!(
        width.get_untracked(),
        window().outer_width().unwrap().as_f64().unwrap()
    );
    assert_eq!(
        height.get_untracked(),
        window().outer_height().unwrap().as_f64().unwrap()
    );
}
