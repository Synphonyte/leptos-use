#![cfg(target_arch = "wasm32")]

use any_spawner::Executor;
use gloo_timers::future::TimeoutFuture;
use leptos::prelude::*;
use leptos_use_watch_with_options::{WatchOptions, watch_with_options};
use std::sync::{Arc, Mutex};
use wasm_bindgen_test::*;

wasm_bindgen_test_configure!(run_in_browser);

type Calls = Arc<Mutex<Vec<(i32, Option<i32>, Option<usize>)>>>;

fn recorder() -> (
    Calls,
    impl Fn(&i32, Option<&i32>, Option<usize>) -> usize + Clone + 'static,
) {
    let calls: Calls = Default::default();
    let cb = {
        let calls = Arc::clone(&calls);
        move |new: &i32, prev: Option<&i32>, prev_ret: Option<usize>| {
            let mut calls = calls.lock().unwrap();
            calls.push((*new, prev.copied(), prev_ret));
            calls.len()
        }
    };
    (calls, cb)
}

async fn setup() -> Owner {
    let _ = Executor::init_wasm_bindgen();
    let owner = Owner::new();
    owner.set();
    owner
}

#[wasm_bindgen_test]
async fn calls_back_with_new_and_previous_values() {
    let _owner = setup().await;

    let (source, set_source) = signal(0);
    let (calls, cb) = recorder();

    let _stop = watch_with_options(move || source.get(), cb, WatchOptions::default());
    Executor::tick().await;
    assert!(calls.lock().unwrap().is_empty(), "not immediate by default");

    set_source.set(1);
    Executor::tick().await;
    set_source.set(2);
    Executor::tick().await;

    assert_eq!(
        *calls.lock().unwrap(),
        vec![(1, Some(0), None), (2, Some(1), Some(1))]
    );
}

#[wasm_bindgen_test]
async fn immediate_calls_back_right_away() {
    let _owner = setup().await;

    let (source, _) = signal(5);
    let (calls, cb) = recorder();

    let _stop = watch_with_options(
        move || source.get(),
        cb,
        WatchOptions::default().immediate(true),
    );
    Executor::tick().await;

    assert_eq!(*calls.lock().unwrap(), vec![(5, None, None)]);
}

#[wasm_bindgen_test]
async fn stop_ends_watching() {
    let _owner = setup().await;

    let (source, set_source) = signal(0);
    let (calls, cb) = recorder();

    let stop = watch_with_options(move || source.get(), cb, WatchOptions::default());
    Executor::tick().await;

    stop();
    set_source.set(1);
    Executor::tick().await;

    assert!(calls.lock().unwrap().is_empty());
}

#[wasm_bindgen_test]
async fn debounce_only_reports_the_settled_value() {
    let _owner = setup().await;

    let (source, set_source) = signal(0);
    let (calls, cb) = recorder();

    let _stop = watch_with_options(
        move || source.get(),
        cb,
        WatchOptions::default().debounce(50.0),
    );
    Executor::tick().await;

    for v in 1..=3 {
        set_source.set(v);
        Executor::tick().await;
    }
    assert!(calls.lock().unwrap().is_empty());

    TimeoutFuture::new(120).await;
    let calls = calls.lock().unwrap();
    assert_eq!(calls.len(), 1);
    assert_eq!(calls[0].0, 3);
}
