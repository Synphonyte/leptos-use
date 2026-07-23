#![cfg(target_arch = "wasm32")]

use gloo_timers::future::TimeoutFuture;
use leptos::prelude::*;
use leptos_use::{DebounceOptions, use_debounce_fn_with_options};
use std::cell::Cell;
use std::rc::Rc;
use wasm_bindgen_test::*;

wasm_bindgen_test_configure!(run_in_browser);

// `max_wait` caps how long a continuous stream of calls may delay the
// invocation. The slot holding its timer handle was never reset after the timer
// fired, and a new max_wait timer is only armed while that slot is empty. The
// cap therefore worked exactly once per hook instance and was silently dead
// afterwards, so a caller that keeps calling faster than `ms` never saw the
// callback again.
#[wasm_bindgen_test]
async fn max_wait_keeps_firing_for_a_continuous_stream_of_calls() {
    let owner = Owner::new();
    owner.set();

    let calls = Rc::new(Cell::new(0_u32));

    let debounced = use_debounce_fn_with_options(
        {
            let calls = Rc::clone(&calls);
            move || calls.set(calls.get() + 1)
        },
        100.0,
        DebounceOptions::default().max_wait(Some(150.0)),
    );

    // Call every 50ms for a second. The regular 100ms timer is always cancelled
    // before it can fire, so every invocation has to come from `max_wait`.
    for _ in 0..20 {
        debounced();
        TimeoutFuture::new(50).await;
    }

    let calls = calls.get();
    assert!(
        calls >= 4,
        "max_wait should have fired repeatedly over 1s, but the callback ran {calls} time(s)"
    );
}
