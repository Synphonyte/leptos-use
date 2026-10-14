#![cfg(target_arch = "wasm32")]

use any_spawner::Executor;
use leptos::prelude::*;
use leptos_use_watch_with_options::WatchOptions;
use leptos_use_whenever::{whenever, whenever_with_options};
use std::sync::{Arc, Mutex};
use wasm_bindgen_test::*;

wasm_bindgen_test_configure!(run_in_browser);

#[wasm_bindgen_test]
async fn calls_back_only_when_the_source_becomes_true() {
    let _ = Executor::init_wasm_bindgen();
    let owner = Owner::new();
    owner.set();

    let (ready, set_ready) = signal(false);
    let calls = Arc::new(Mutex::new(Vec::new()));

    let _stop = whenever(move || ready.get(), {
        let calls = Arc::clone(&calls);
        move |v, prev, _| calls.lock().unwrap().push((v, prev))
    });
    Executor::tick().await;

    set_ready.set(true);
    Executor::tick().await;
    set_ready.set(false);
    Executor::tick().await;
    set_ready.set(true);
    Executor::tick().await;

    assert_eq!(
        *calls.lock().unwrap(),
        vec![(true, Some(false)), (true, Some(false))],
        "false values must not call back"
    );
}

#[wasm_bindgen_test]
async fn immediate_calls_back_when_already_true() {
    let _ = Executor::init_wasm_bindgen();
    let owner = Owner::new();
    owner.set();

    let (ready, _) = signal(true);
    let calls = Arc::new(Mutex::new(0));

    let _stop = whenever_with_options(
        move || ready.get(),
        {
            let calls = Arc::clone(&calls);
            move |_, _, _| *calls.lock().unwrap() += 1
        },
        WatchOptions::default().immediate(true),
    );
    Executor::tick().await;

    assert_eq!(*calls.lock().unwrap(), 1);
}
