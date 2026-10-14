#![cfg(target_arch = "wasm32")]

use any_spawner::Executor;
use gloo_timers::future::TimeoutFuture;
use leptos::prelude::*;
use leptos_use_device_pixel_ratio::use_device_pixel_ratio;
use wasm_bindgen_test::*;

wasm_bindgen_test_configure!(run_in_browser);

#[wasm_bindgen_test]
async fn reports_the_device_pixel_ratio() {
    let _ = Executor::init_wasm_bindgen();
    let owner = Owner::new();
    owner.set();

    let pixel_ratio = use_device_pixel_ratio();
    TimeoutFuture::new(20).await;

    assert!(pixel_ratio.get_untracked() > 0.0);
    assert_eq!(pixel_ratio.get_untracked(), window().device_pixel_ratio());
}
