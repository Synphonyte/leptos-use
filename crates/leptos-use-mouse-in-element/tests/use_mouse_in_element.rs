#![cfg(target_arch = "wasm32")]

use any_spawner::Executor;
use gloo_timers::future::TimeoutFuture;
use leptos::prelude::*;
use leptos_use_mouse_in_element::{
    UseMouseInElementOptions, UseMouseInElementReturn, use_mouse_in_element,
    use_mouse_in_element_with_options,
};
use wasm_bindgen_test::*;

wasm_bindgen_test_configure!(run_in_browser);

fn append_box() -> web_sys::Element {
    let el = document().create_element("div").unwrap();
    el.set_attribute(
        "style",
        "position: fixed; left: 100px; top: 50px; width: 200px; height: 100px;",
    )
    .unwrap();
    document().body().unwrap().append_child(&el).unwrap();
    el
}

fn move_mouse(x: i32, y: i32) {
    let init = web_sys::MouseEventInit::new();
    init.set_client_x(x);
    init.set_client_y(y);
    init.set_bubbles(true);
    let event = web_sys::MouseEvent::new_with_mouse_event_init_dict("mousemove", &init).unwrap();
    window().dispatch_event(&event).unwrap();
}

#[wasm_bindgen_test]
async fn reports_the_position_relative_to_the_element() {
    let _ = Executor::init_wasm_bindgen();
    let owner = Owner::new();
    owner.set();

    let el = append_box();
    let UseMouseInElementReturn {
        x,
        y,
        element_x,
        element_y,
        element_position_x,
        element_position_y,
        element_width,
        element_height,
        is_outside,
        ..
    } = use_mouse_in_element(el);
    Executor::tick().await;
    assert!(is_outside.get_untracked());

    move_mouse(130, 70);
    TimeoutFuture::new(20).await;

    assert_eq!((x.get_untracked(), y.get_untracked()), (130.0, 70.0));
    assert_eq!(element_position_x.get_untracked(), 100.0);
    assert_eq!(element_position_y.get_untracked(), 50.0);
    assert_eq!(element_width.get_untracked(), 200.0);
    assert_eq!(element_height.get_untracked(), 100.0);
    assert_eq!(
        (element_x.get_untracked(), element_y.get_untracked()),
        (30.0, 20.0)
    );
    assert!(!is_outside.get_untracked());

    move_mouse(10, 10);
    TimeoutFuture::new(20).await;

    assert!(is_outside.get_untracked());
    assert_eq!(
        (element_x.get_untracked(), element_y.get_untracked()),
        (-90.0, -40.0),
        "outside positions are reported by default"
    );
}

#[wasm_bindgen_test]
async fn keeps_the_last_inside_position_when_not_handling_outside() {
    let _ = Executor::init_wasm_bindgen();
    let owner = Owner::new();
    owner.set();

    let el = append_box();
    let UseMouseInElementReturn {
        element_x,
        element_y,
        is_outside,
        ..
    } = use_mouse_in_element_with_options(
        el,
        UseMouseInElementOptions::default().handle_outside(false),
    );
    Executor::tick().await;

    move_mouse(150, 60);
    TimeoutFuture::new(20).await;
    assert_eq!(
        (element_x.get_untracked(), element_y.get_untracked()),
        (50.0, 10.0)
    );

    move_mouse(5, 5);
    TimeoutFuture::new(20).await;
    assert!(is_outside.get_untracked());
    assert_eq!(
        (element_x.get_untracked(), element_y.get_untracked()),
        (50.0, 10.0)
    );
}

#[wasm_bindgen_test]
async fn stop_ends_tracking() {
    let _ = Executor::init_wasm_bindgen();
    let owner = Owner::new();
    owner.set();

    let el = append_box();
    let UseMouseInElementReturn {
        element_x, stop, ..
    } = use_mouse_in_element(el);
    Executor::tick().await;

    move_mouse(110, 60);
    TimeoutFuture::new(20).await;
    assert_eq!(element_x.get_untracked(), 10.0);

    stop();
    move_mouse(190, 60);
    TimeoutFuture::new(20).await;
    assert_eq!(element_x.get_untracked(), 10.0);
}
