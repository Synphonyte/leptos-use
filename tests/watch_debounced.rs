#![cfg(target_arch = "wasm32")]

use gloo_timers::future::TimeoutFuture;
use leptos::prelude::*;
use leptos_use::watch_debounced;
use std::cell::RefCell;
use std::rc::Rc;
use wasm_bindgen_test::*;

wasm_bindgen_test_configure!(run_in_browser);

// `watch_*` passes the callback's own previous return value as the third
// argument. The bookkeeping cell was overwritten on every dependency change
// with whatever the filter happened to be holding, even when the filter had
// only rescheduled its timer without invoking the callback. Two rapid changes
// inside one debounce window therefore destroyed the previous return value and
// the callback saw `None` instead.
#[wasm_bindgen_test]
async fn debounced_watch_keeps_the_previous_return_value() {
    let _ = any_spawner::Executor::init_wasm_bindgen();

    let owner = Owner::new();
    owner.set();

    let (source, set_source) = signal(0);
    let seen: Rc<RefCell<Vec<i32>>> = Rc::new(RefCell::new(Vec::new()));

    let _stop = watch_debounced(
        move || source.get(),
        {
            let seen = Rc::clone(&seen);

            move |_value, _prev_value, prev_return: Option<i32>| {
                let next = prev_return.unwrap_or(0) + 1;
                seen.borrow_mut().push(next);
                next
            }
        },
        100.0,
    );

    // Let the effect perform its initial run, which does not invoke the
    // callback because `immediate` defaults to false.
    TimeoutFuture::new(50).await;

    // A single change that is allowed to settle: the callback runs once and
    // returns 1.
    set_source.set(1);
    TimeoutFuture::new(400).await;

    // Two changes inside one debounce window. Only the second one reaches the
    // callback, and it has to see the 1 returned by the previous invocation.
    set_source.set(2);
    TimeoutFuture::new(20).await;
    set_source.set(3);
    TimeoutFuture::new(400).await;

    assert_eq!(
        *seen.borrow(),
        vec![1, 2],
        "the second invocation must receive the previous return value"
    );
}
