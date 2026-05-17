#![cfg(target_arch = "wasm32")]

use std::sync::Arc;
use std::sync::atomic::{AtomicUsize, Ordering};

use gloo_timers::future::TimeoutFuture;
use leptos::prelude::*;
use leptos_use::{UseTimeoutFnReturn, use_timeout_fn};
use wasm_bindgen_test::*;

wasm_bindgen_test_configure!(run_in_browser);

// Regression test for https://github.com/Synphonyte/leptos-use/issues/306:
// Calling `start()` while a previous timer is still pending must cancel the
// previous timer (matching VueUse's `useTimeoutFn`). Before the fix, both
// timers ran to completion and the original callback fired at its
// originally-scheduled time.
#[wasm_bindgen_test]
async fn start_cancels_pending_timer() {
    let owner = Owner::new();
    owner.set();

    let count = Arc::new(AtomicUsize::new(0));

    let UseTimeoutFnReturn { start, .. } = use_timeout_fn(
        {
            let count = Arc::clone(&count);
            move |_: ()| {
                count.fetch_add(1, Ordering::SeqCst);
            }
        },
        200.0_f64,
    );

    start(());
    TimeoutFuture::new(50).await;
    start(());
    TimeoutFuture::new(400).await;

    assert_eq!(
        count.load(Ordering::SeqCst),
        1,
        "second start() must cancel the first timer; both callbacks fired"
    );
}
