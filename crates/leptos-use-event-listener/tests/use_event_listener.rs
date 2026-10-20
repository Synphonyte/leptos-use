#![cfg(target_arch = "wasm32")]

use any_spawner::Executor;
use leptos::ev::Custom;
use leptos::prelude::*;
use leptos_use_event_listener::{
    UseEventListenerOptions, use_event_listener, use_event_listener_with_options,
};
use std::cell::Cell;
use std::rc::Rc;
use wasm_bindgen_test::*;

wasm_bindgen_test_configure!(run_in_browser);

fn append_div() -> web_sys::Element {
    let el = document().create_element("div").unwrap();
    document().body().unwrap().append_child(&el).unwrap();
    el
}

fn fire(target: &web_sys::EventTarget, name: &str) {
    target
        .dispatch_event(&web_sys::Event::new(name).unwrap())
        .unwrap();
}

fn ping() -> Custom<web_sys::Event> {
    Custom::new("ping")
}

fn counter() -> (Rc<Cell<u32>>, impl FnMut(web_sys::Event) + 'static) {
    let count = Rc::new(Cell::new(0));
    let handler = {
        let count = Rc::clone(&count);
        move |_: web_sys::Event| count.set(count.get() + 1)
    };
    (count, handler)
}

async fn setup() -> Owner {
    let _ = Executor::init_wasm_bindgen();
    let owner = Owner::new();
    owner.set();
    owner
}

#[wasm_bindgen_test]
async fn listens_until_stopped() {
    let _owner = setup().await;

    let el = append_div();
    let (count, handler) = counter();

    let stop = use_event_listener(el.clone(), ping(), handler);
    Executor::tick().await;

    fire(&el, "ping");
    fire(&el, "ping");
    assert_eq!(count.get(), 2);

    stop();
    fire(&el, "ping");
    assert_eq!(count.get(), 2, "no events after stop");
}

#[wasm_bindgen_test]
async fn once_listens_only_once() {
    let _owner = setup().await;

    let el = append_div();
    let (count, handler) = counter();

    let _stop = use_event_listener_with_options(
        el.clone(),
        ping(),
        handler,
        UseEventListenerOptions::default().once(true),
    );
    Executor::tick().await;

    fire(&el, "ping");
    fire(&el, "ping");
    assert_eq!(count.get(), 1);
}

#[wasm_bindgen_test]
async fn follows_a_reactive_target() {
    let _owner = setup().await;

    let first = append_div();
    let second = append_div();
    let (target, set_target) = signal_local(None::<web_sys::Element>);
    let (count, handler) = counter();

    let _stop = use_event_listener(target, ping(), handler);
    Executor::tick().await;

    fire(&first, "ping");
    assert_eq!(count.get(), 0, "no target yet");

    set_target.set(Some(first.clone()));
    Executor::tick().await;
    fire(&first, "ping");
    assert_eq!(count.get(), 1);

    set_target.set(Some(second.clone()));
    Executor::tick().await;
    fire(&first, "ping");
    assert_eq!(count.get(), 1, "the old target is no longer listened to");
    fire(&second, "ping");
    assert_eq!(count.get(), 2);
}

#[wasm_bindgen_test]
async fn is_removed_when_the_owner_is_cleaned_up() {
    let _ = Executor::init_wasm_bindgen();

    let el = append_div();
    let (count, handler) = counter();

    let owner = Owner::new();
    owner.with(|| {
        let _ = use_event_listener(el.clone(), ping(), handler);
    });
    Executor::tick().await;

    fire(&el, "ping");
    assert_eq!(count.get(), 1);

    owner.cleanup();
    fire(&el, "ping");
    assert_eq!(count.get(), 1);
}

#[wasm_bindgen_test]
async fn listens_on_the_window() {
    let _owner = setup().await;

    let (count, handler) = counter();
    let _stop = use_event_listener(window(), ping(), handler);
    Executor::tick().await;

    fire(&window(), "ping");
    assert_eq!(count.get(), 1);
}
