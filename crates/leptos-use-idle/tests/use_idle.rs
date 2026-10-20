#![cfg(target_arch = "wasm32")]

use any_spawner::Executor;
use gloo_timers::future::TimeoutFuture;
use leptos::prelude::*;
use leptos_use_idle::{UseIdleOptions, UseIdleReturn, use_idle, use_idle_with_options};
use wasm_bindgen_test::*;

wasm_bindgen_test_configure!(run_in_browser);

fn fire(name: &str) {
    window()
        .dispatch_event(&web_sys::Event::new(name).unwrap())
        .unwrap();
}

#[wasm_bindgen_test]
async fn becomes_idle_without_activity() {
    let _ = Executor::init_wasm_bindgen();
    let owner = Owner::new();
    owner.set();

    let UseIdleReturn {
        idle, last_active, ..
    } = use_idle(80);
    Executor::tick().await;
    assert!(!idle.get_untracked());

    TimeoutFuture::new(50).await;
    fire("mousemove");
    let active_at = last_active.get_untracked();
    TimeoutFuture::new(50).await;
    assert!(!idle.get_untracked(), "activity restarts the timeout");

    TimeoutFuture::new(100).await;
    assert!(idle.get_untracked());

    fire("keydown");
    assert!(!idle.get_untracked(), "activity ends idling");
    assert!(last_active.get_untracked() >= active_at);
}

#[wasm_bindgen_test]
async fn reset_restarts_the_timeout() {
    let _ = Executor::init_wasm_bindgen();
    let owner = Owner::new();
    owner.set();

    let UseIdleReturn { idle, reset, .. } = use_idle(60);
    TimeoutFuture::new(100).await;
    assert!(idle.get_untracked());

    reset();
    assert!(!idle.get_untracked());

    TimeoutFuture::new(30).await;
    assert!(!idle.get_untracked());
    TimeoutFuture::new(90).await;
    assert!(idle.get_untracked());
}

#[wasm_bindgen_test]
async fn only_listens_to_the_configured_events() {
    let _ = Executor::init_wasm_bindgen();
    let owner = Owner::new();
    owner.set();

    let UseIdleReturn { idle, .. } = use_idle_with_options(
        40,
        UseIdleOptions::default().events(vec!["custom-activity".to_string()]),
    );
    TimeoutFuture::new(80).await;
    assert!(idle.get_untracked());

    fire("mousemove");
    assert!(idle.get_untracked());

    fire("custom-activity");
    assert!(!idle.get_untracked());
}

// `resize` is only fired on the window.
#[wasm_bindgen_test]
async fn resizing_the_window_counts_as_activity() {
    let _ = Executor::init_wasm_bindgen();
    let owner = Owner::new();
    owner.set();

    let UseIdleReturn { idle, .. } = use_idle(40);
    TimeoutFuture::new(80).await;
    assert!(idle.get_untracked());

    fire("resize");
    assert!(!idle.get_untracked());
}

// Real activity events are dispatched on elements and bubble up to the window.
#[wasm_bindgen_test]
async fn bubbling_events_from_elements_count_as_activity() {
    let _ = Executor::init_wasm_bindgen();
    let owner = Owner::new();
    owner.set();

    let UseIdleReturn { idle, .. } = use_idle(40);
    TimeoutFuture::new(80).await;
    assert!(idle.get_untracked());

    let init = web_sys::EventInit::new();
    init.set_bubbles(true);
    document()
        .body()
        .unwrap()
        .dispatch_event(&web_sys::Event::new_with_event_init_dict("mousedown", &init).unwrap())
        .unwrap();
    assert!(!idle.get_untracked());
}
