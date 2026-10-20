#![cfg(target_arch = "wasm32")]

use any_spawner::Executor;
use leptos::prelude::*;
use leptos_use_watch_pausable::{WatchPausableReturn, watch_pausable};
use std::sync::{Arc, Mutex};
use wasm_bindgen_test::*;

wasm_bindgen_test_configure!(run_in_browser);

#[wasm_bindgen_test]
async fn pause_and_resume_the_watcher() {
    let _ = Executor::init_wasm_bindgen();
    let owner = Owner::new();
    owner.set();

    let (source, set_source) = signal("foo".to_string());
    let seen = Arc::new(Mutex::new(Vec::new()));

    let WatchPausableReturn {
        pause,
        resume,
        is_active,
        stop,
    } = watch_pausable(move || source.get(), {
        let seen = Arc::clone(&seen);
        move |v: &String, _, _| seen.lock().unwrap().push(v.clone())
    });
    Executor::tick().await;
    assert!(is_active.get_untracked());

    set_source.set("bar".to_string());
    Executor::tick().await;

    pause();
    assert!(!is_active.get_untracked());
    set_source.set("foobar".to_string());
    Executor::tick().await;

    resume();
    assert!(is_active.get_untracked());
    set_source.set("hello".to_string());
    Executor::tick().await;

    stop();
    set_source.set("stopped".to_string());
    Executor::tick().await;

    assert_eq!(
        *seen.lock().unwrap(),
        vec!["bar".to_string(), "hello".to_string()]
    );
}
