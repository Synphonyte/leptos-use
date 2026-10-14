#![cfg(target_arch = "wasm32")]

use any_spawner::Executor;
use gloo_timers::future::TimeoutFuture;
use leptos::prelude::*;
use leptos_use_timestamp::{
    TimestampInterval, UseTimestampOptions, UseTimestampReturn, use_timestamp,
    use_timestamp_with_controls_and_options, use_timestamp_with_options,
};
use wasm_bindgen_test::*;

wasm_bindgen_test_configure!(run_in_browser);

#[wasm_bindgen_test]
async fn follows_the_current_time() {
    let _ = Executor::init_wasm_bindgen();
    let owner = Owner::new();
    owner.set();

    let timestamp = use_timestamp();
    let first = timestamp.get_untracked();
    assert!((first - js_sys::Date::now()).abs() < 1000.0);

    TimeoutFuture::new(100).await;
    assert!(
        timestamp.get_untracked() - first >= 50.0,
        "updated on animation frames"
    );
}

#[wasm_bindgen_test]
async fn adds_the_offset() {
    let _ = Executor::init_wasm_bindgen();
    let owner = Owner::new();
    owner.set();

    let timestamp = use_timestamp_with_options(
        UseTimestampOptions::default()
            .offset(60_000.0)
            .interval(TimestampInterval::Interval(10)),
    );

    let diff = timestamp.get_untracked() - js_sys::Date::now();
    assert!((diff - 60_000.0).abs() < 1000.0, "diff {diff}");
}

#[wasm_bindgen_test]
async fn pause_stops_updating() {
    let _ = Executor::init_wasm_bindgen();
    let owner = Owner::new();
    owner.set();

    let UseTimestampReturn {
        timestamp,
        is_active,
        pause,
        resume,
    } = use_timestamp_with_controls_and_options(
        UseTimestampOptions::default().interval(TimestampInterval::Interval(10)),
    );
    TimeoutFuture::new(30).await;
    assert!(is_active.get_untracked());

    pause();
    assert!(!is_active.get_untracked());
    let paused_at = timestamp.get_untracked();
    TimeoutFuture::new(60).await;
    assert_eq!(timestamp.get_untracked(), paused_at);

    resume();
    TimeoutFuture::new(60).await;
    assert!(timestamp.get_untracked() > paused_at);
}
