#![cfg(target_arch = "wasm32")]

use gloo_timers::future::TimeoutFuture;
use leptos::prelude::*;
use leptos_use_filters::{
    DebounceOptions, FilterOptions, ThrottleOptions, create_filter_wrapper,
    create_filter_wrapper_with_arg, debounce_filter, throttle_filter,
};
use std::sync::Arc;
use std::sync::atomic::{AtomicUsize, Ordering};
use wasm_bindgen_test::*;

wasm_bindgen_test_configure!(run_in_browser);

fn counter() -> (Arc<AtomicUsize>, impl Fn() + Clone + Send + Sync + 'static) {
    let count = Arc::new(AtomicUsize::new(0));
    let inc = {
        let count = Arc::clone(&count);
        move || {
            count.fetch_add(1, Ordering::SeqCst);
        }
    };
    (count, inc)
}

#[wasm_bindgen_test]
async fn debounce_runs_once_after_calls_stop() {
    let owner = Owner::new();
    owner.set();

    let (count, inc) = counter();
    let debounced = create_filter_wrapper(
        Arc::new(debounce_filter(50.0, DebounceOptions::default())),
        inc,
    );

    debounced();
    debounced();
    TimeoutFuture::new(20).await;
    debounced();
    assert_eq!(
        count.load(Ordering::SeqCst),
        0,
        "nothing runs while calls keep coming"
    );

    TimeoutFuture::new(120).await;
    assert_eq!(count.load(Ordering::SeqCst), 1);
}

#[wasm_bindgen_test]
async fn debounce_max_wait_forces_a_call() {
    let owner = Owner::new();
    owner.set();

    let (count, inc) = counter();
    let debounced = create_filter_wrapper(
        Arc::new(debounce_filter(
            80.0,
            DebounceOptions::default().max_wait(Some(100.0)),
        )),
        inc,
    );

    // keep calling more often than the debounce time for longer than max_wait
    for _ in 0..6 {
        debounced();
        TimeoutFuture::new(40).await;
    }
    assert!(
        count.load(Ordering::SeqCst) >= 1,
        "max_wait must invoke even though calls never stopped"
    );
}

#[wasm_bindgen_test]
async fn throttle_runs_leading_and_trailing() {
    let owner = Owner::new();
    owner.set();

    let (count, inc) = counter();
    let throttled = create_filter_wrapper(
        Arc::new(throttle_filter(80.0, ThrottleOptions::default())),
        inc,
    );

    throttled();
    assert_eq!(
        count.load(Ordering::SeqCst),
        1,
        "leading call runs immediately"
    );

    throttled();
    throttled();
    assert_eq!(count.load(Ordering::SeqCst), 1);

    TimeoutFuture::new(150).await;
    assert_eq!(count.load(Ordering::SeqCst), 2, "one trailing call");
}

#[wasm_bindgen_test]
async fn throttle_without_trailing_drops_calls() {
    let owner = Owner::new();
    owner.set();

    let (count, inc) = counter();
    let throttled = create_filter_wrapper(
        Arc::new(throttle_filter(
            80.0,
            ThrottleOptions {
                trailing: false,
                leading: true,
            },
        )),
        inc,
    );

    throttled();
    throttled();
    TimeoutFuture::new(150).await;
    assert_eq!(count.load(Ordering::SeqCst), 1);

    throttled();
    assert_eq!(
        count.load(Ordering::SeqCst),
        2,
        "a new window starts after the interval"
    );
}

#[wasm_bindgen_test]
fn no_filter_invokes_directly_and_returns_the_value() {
    let owner = Owner::new();
    owner.set();

    let wrapped = create_filter_wrapper_with_arg(FilterOptions::None.filter_fn(), |x: i32| x * 2);

    let result = wrapped(21);
    assert_eq!(*result.lock().unwrap(), Some(42));
}

#[wasm_bindgen_test]
async fn filter_options_build_the_matching_filter() {
    let owner = Owner::new();
    owner.set();

    let (count, inc) = counter();
    let debounced = create_filter_wrapper(FilterOptions::debounce(30.0).filter_fn(), inc.clone());
    debounced();
    assert_eq!(count.load(Ordering::SeqCst), 0);
    TimeoutFuture::new(80).await;
    assert_eq!(count.load(Ordering::SeqCst), 1);

    let throttled = create_filter_wrapper(FilterOptions::throttle(30.0).filter_fn(), inc);
    throttled();
    assert_eq!(count.load(Ordering::SeqCst), 2);
}
