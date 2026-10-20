#![cfg(target_arch = "wasm32")]

use any_spawner::Executor;
use gloo_timers::future::TimeoutFuture;
use leptos::prelude::*;
use leptos_use_interval::{
    UseIntervalOptions, UseIntervalReturn, use_interval, use_interval_with_options,
};
use std::cell::RefCell;
use std::rc::Rc;
use wasm_bindgen_test::*;

wasm_bindgen_test_configure!(run_in_browser);

#[wasm_bindgen_test]
async fn counts_pauses_resumes_and_resets() {
    let _ = Executor::init_wasm_bindgen();
    let owner = Owner::new();
    owner.set();

    let UseIntervalReturn {
        counter,
        reset,
        is_active,
        pause,
        resume,
    } = use_interval(30_u64);
    assert!(is_active.get_untracked());

    TimeoutFuture::new(110).await;
    let counted = counter.get_untracked();
    assert!((2..=4).contains(&counted), "counted {counted}");

    pause();
    assert!(!is_active.get_untracked());
    TimeoutFuture::new(100).await;
    assert_eq!(counter.get_untracked(), counted, "no counting while paused");

    reset();
    assert_eq!(counter.get_untracked(), 0);

    resume();
    TimeoutFuture::new(80).await;
    assert!(counter.get_untracked() >= 1);
}

#[wasm_bindgen_test]
async fn calls_back_with_the_count_and_can_start_paused() {
    let _ = Executor::init_wasm_bindgen();
    let owner = Owner::new();
    owner.set();

    let seen = Rc::new(RefCell::new(Vec::new()));
    let UseIntervalReturn {
        is_active, resume, ..
    } = use_interval_with_options(
        20_u64,
        UseIntervalOptions::default().immediate(false).callback({
            let seen = Rc::clone(&seen);
            move |count| seen.borrow_mut().push(count)
        }),
    );
    TimeoutFuture::new(60).await;
    assert!(!is_active.get_untracked());
    assert!(seen.borrow().is_empty());

    resume();
    TimeoutFuture::new(70).await;
    let seen = seen.borrow();
    assert!(seen.len() >= 2);
    assert_eq!(seen[..2], [1, 2]);
}
