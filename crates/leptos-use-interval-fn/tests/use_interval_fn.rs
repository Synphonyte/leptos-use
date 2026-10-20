#![cfg(target_arch = "wasm32")]

use any_spawner::Executor;
use gloo_timers::future::TimeoutFuture;
use leptos::prelude::*;
use leptos_use_filters::Pausable;
use leptos_use_interval_fn::{UseIntervalFnOptions, use_interval_fn, use_interval_fn_with_options};
use std::cell::Cell;
use std::rc::Rc;
use wasm_bindgen_test::*;

wasm_bindgen_test_configure!(run_in_browser);

fn counter() -> (Rc<Cell<u32>>, impl Fn() + Clone + 'static) {
    let count = Rc::new(Cell::new(0));
    let inc = {
        let count = Rc::clone(&count);
        move || count.set(count.get() + 1)
    };
    (count, inc)
}

#[wasm_bindgen_test]
async fn calls_repeatedly_until_paused() {
    let _ = Executor::init_wasm_bindgen();
    let owner = Owner::new();
    owner.set();

    let (count, inc) = counter();
    let Pausable {
        pause,
        resume,
        is_active,
    } = use_interval_fn(inc, 30_u64);
    assert!(is_active.get_untracked());
    assert_eq!(count.get(), 0, "no immediate callback by default");

    TimeoutFuture::new(110).await;
    let counted = count.get();
    assert!((2..=4).contains(&counted), "called {counted} times");

    pause();
    assert!(!is_active.get_untracked());
    TimeoutFuture::new(100).await;
    assert_eq!(count.get(), counted);

    resume();
    TimeoutFuture::new(50).await;
    assert!(count.get() > counted);
}

#[wasm_bindgen_test]
async fn immediate_callback_calls_on_resume() {
    let _ = Executor::init_wasm_bindgen();
    let owner = Owner::new();
    owner.set();

    let (count, inc) = counter();
    let Pausable {
        resume, is_active, ..
    } = use_interval_fn_with_options(
        inc,
        1000_u64,
        UseIntervalFnOptions::default()
            .immediate(false)
            .immediate_callback(true),
    );
    assert!(!is_active.get_untracked());
    assert_eq!(count.get(), 0);

    resume();
    assert_eq!(count.get(), 1);
}

#[wasm_bindgen_test]
async fn follows_a_reactive_interval() {
    let _ = Executor::init_wasm_bindgen();
    let owner = Owner::new();
    owner.set();

    let (count, inc) = counter();
    let (interval, set_interval) = signal(10_000_u64);
    let _ = use_interval_fn(inc, interval);
    Executor::tick().await;

    TimeoutFuture::new(60).await;
    assert_eq!(count.get(), 0);

    set_interval.set(20);
    TimeoutFuture::new(90).await;
    assert!(count.get() >= 2, "called {} times", count.get());
}
