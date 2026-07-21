#![cfg(target_arch = "wasm32")]

use gloo_timers::future::TimeoutFuture;
use leptos::prelude::*;
use leptos_use::use_raf_fn;
use leptos_use::utils::Pausable;
use std::cell::RefCell;
use std::rc::Rc;
use wasm_bindgen_test::*;

wasm_bindgen_test_configure!(run_in_browser);

// `delta` is measured against the timestamp of the previously rendered frame.
// Pausing left that timestamp untouched, so the first frame after resuming
// reported the entire wall-clock duration of the pause as a single frame delta.
// Animation and physics code stepping by `delta` jumps by that whole amount.
#[wasm_bindgen_test]
async fn first_frame_after_resume_does_not_span_the_pause() {
    let owner = Owner::new();
    owner.set();

    let deltas: Rc<RefCell<Vec<f64>>> = Rc::new(RefCell::new(Vec::new()));

    let Pausable { pause, resume, .. } = use_raf_fn({
        let deltas = Rc::clone(&deltas);
        move |args| deltas.borrow_mut().push(args.delta)
    });

    // Let a few frames run.
    TimeoutFuture::new(200).await;

    pause();
    let frames_before_pause = deltas.borrow().len();
    assert!(
        frames_before_pause > 0,
        "the callback should have run while active"
    );

    // Stay paused far longer than a frame.
    TimeoutFuture::new(600).await;
    assert_eq!(
        deltas.borrow().len(),
        frames_before_pause,
        "the callback must not run while paused"
    );

    resume();
    TimeoutFuture::new(200).await;
    pause();

    let deltas = deltas.borrow();
    assert!(
        deltas.len() > frames_before_pause,
        "the callback should run again after resuming"
    );

    let first_after_resume = deltas[frames_before_pause];
    assert!(
        first_after_resume < 100.0,
        "the first delta after resuming was {first_after_resume}ms, which includes the paused time"
    );
}
