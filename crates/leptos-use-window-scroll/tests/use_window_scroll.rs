#![cfg(target_arch = "wasm32")]

use any_spawner::Executor;
use gloo_timers::future::TimeoutFuture;
use leptos::prelude::*;
use leptos_use_window_scroll::use_window_scroll;
use wasm_bindgen_test::*;

wasm_bindgen_test_configure!(run_in_browser);

#[wasm_bindgen_test]
async fn follows_the_window_scroll_position() {
    let _ = Executor::init_wasm_bindgen();
    let owner = Owner::new();
    owner.set();

    let spacer = document().create_element("div").unwrap();
    spacer
        .set_attribute("style", "width: 5000px; height: 5000px;")
        .unwrap();
    document().body().unwrap().append_child(&spacer).unwrap();
    window().scroll_to_with_x_and_y(0.0, 0.0);

    let (x, y) = use_window_scroll();
    TimeoutFuture::new(50).await;
    assert_eq!((x.get_untracked(), y.get_untracked()), (0.0, 0.0));

    window().scroll_to_with_x_and_y(120.0, 300.0);
    TimeoutFuture::new(100).await;
    assert_eq!((x.get_untracked(), y.get_untracked()), (120.0, 300.0));

    window().scroll_to_with_x_and_y(0.0, 40.0);
    TimeoutFuture::new(100).await;
    assert_eq!((x.get_untracked(), y.get_untracked()), (0.0, 40.0));

    window().scroll_to_with_x_and_y(0.0, 0.0);
    spacer.remove();
}
