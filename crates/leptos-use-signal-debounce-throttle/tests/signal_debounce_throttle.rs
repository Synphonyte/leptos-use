#![cfg(target_arch = "wasm32")]

use any_spawner::Executor;
use gloo_timers::future::TimeoutFuture;
use leptos::prelude::*;
use leptos_use_signal_debounce_throttle::{signal_debounced, signal_throttled};
use wasm_bindgen_test::*;

wasm_bindgen_test_configure!(run_in_browser);

#[wasm_bindgen_test]
async fn debounced_signal_settles_on_the_last_value() {
    let _ = Executor::init_wasm_bindgen();
    let owner = Owner::new();
    owner.set();

    let (value, set_value) = signal(0);
    let debounced: Signal<i32> = signal_debounced(value, 60.0);

    assert_eq!(debounced.get_untracked(), 0);

    Executor::tick().await;
    set_value.set(1);
    Executor::tick().await;
    set_value.set(2);
    Executor::tick().await;
    assert_eq!(
        debounced.get_untracked(),
        0,
        "still waiting for the value to settle"
    );

    TimeoutFuture::new(150).await;
    assert_eq!(debounced.get_untracked(), 2);
}

#[wasm_bindgen_test]
async fn throttled_signal_updates_at_most_once_per_interval() {
    let _ = Executor::init_wasm_bindgen();
    let owner = Owner::new();
    owner.set();

    let (value, set_value) = signal(0);
    let throttled: Signal<i32> = signal_throttled(value, 100.0);

    Executor::tick().await;
    set_value.set(1);
    Executor::tick().await;
    assert_eq!(
        throttled.get_untracked(),
        1,
        "the leading change goes through"
    );

    set_value.set(2);
    Executor::tick().await;
    set_value.set(3);
    Executor::tick().await;
    assert_eq!(throttled.get_untracked(), 1);

    TimeoutFuture::new(200).await;
    assert_eq!(
        throttled.get_untracked(),
        3,
        "the trailing change goes through"
    );
}
