#![cfg(target_arch = "wasm32")]

use any_spawner::Executor;
use leptos::prelude::*;
use leptos::wasm_bindgen::JsCast;
use leptos_use_on_click_outside::{
    OnClickOutsideOptions, on_click_outside, on_click_outside_with_options,
};
use std::cell::Cell;
use std::rc::Rc;
use wasm_bindgen_test::*;

wasm_bindgen_test_configure!(run_in_browser);

fn append_div(parent: &web_sys::Node) -> web_sys::HtmlElement {
    let el = document().create_element("div").unwrap();
    parent.append_child(&el).unwrap();
    el.unchecked_into()
}

fn body() -> web_sys::Node {
    document().body().unwrap().into()
}

fn counter() -> (Rc<Cell<u32>>, impl FnMut(web_sys::Event) + Clone + 'static) {
    let count = Rc::new(Cell::new(0));
    let handler = {
        let count = Rc::clone(&count);
        move |_| count.set(count.get() + 1)
    };
    (count, handler)
}

#[wasm_bindgen_test]
async fn calls_the_handler_only_for_clicks_outside() {
    let _ = Executor::init_wasm_bindgen();
    let owner = Owner::new();
    owner.set();

    let target = append_div(&body());
    let inner = append_div(&target);
    let outside = append_div(&body());
    let (count, handler) = counter();

    let _stop = on_click_outside(target.clone(), handler);
    Executor::tick().await;

    target.click();
    inner.click();
    assert_eq!(count.get(), 0, "clicks inside the target are ignored");

    outside.click();
    assert_eq!(count.get(), 1);

    document().body().unwrap().click();
    assert_eq!(count.get(), 2);
}

#[wasm_bindgen_test]
async fn ignores_the_configured_elements() {
    let _ = Executor::init_wasm_bindgen();
    let owner = Owner::new();
    owner.set();

    let target = append_div(&body());
    let ignored = append_div(&body());
    let ignored_child = append_div(&ignored);
    let outside = append_div(&body());
    let (count, handler) = counter();

    let _stop = on_click_outside_with_options(
        target,
        handler,
        OnClickOutsideOptions::default().ignore(vec![ignored.clone()]),
    );
    Executor::tick().await;

    ignored.click();
    ignored_child.click();
    assert_eq!(count.get(), 0);

    outside.click();
    assert_eq!(count.get(), 1);
}

#[wasm_bindgen_test]
async fn stop_removes_the_listeners() {
    let _ = Executor::init_wasm_bindgen();
    let owner = Owner::new();
    owner.set();

    let target = append_div(&body());
    let outside = append_div(&body());
    let (count, handler) = counter();

    let stop = on_click_outside(target, handler);
    Executor::tick().await;

    stop();
    outside.click();
    assert_eq!(count.get(), 0);
}
