#![cfg(target_arch = "wasm32")]

use gloo_timers::future::TimeoutFuture;
use leptos::prelude::*;
use leptos_use::{UseScrollReturn, use_scroll};
use wasm_bindgen_test::*;

wasm_bindgen_test_configure!(run_in_browser);

/// A horizontally scrollable box whose content is far wider than the box.
fn scroll_box(dir: &str) -> web_sys::Element {
    let document = document();

    let el = document
        .create_element("div")
        .expect("failed to create div");
    el.set_attribute("dir", dir).expect("failed to set dir");
    el.set_attribute("style", "width: 100px; height: 50px; overflow-x: scroll;")
        .expect("failed to set style");

    let content = document
        .create_element("div")
        .expect("failed to create div");
    content
        .set_attribute("style", "width: 600px; height: 20px;")
        .expect("failed to set style");
    el.append_child(&content).expect("failed to append");

    document
        .body()
        .expect("no body")
        .append_child(&el)
        .expect("failed to append");

    el
}

// Browsers use the "negative scrollLeft" model for right-to-left containers:
// `scrollLeft` is 0 at the right-hand (reading start) edge and decreases towards
// the left edge. The arrived state was computed from `scroll_left.abs()` and only
// swapped for `flex-direction: row-reverse`, so a plain `dir="rtl"` box reported
// the two horizontal edges the wrong way round.
#[wasm_bindgen_test]
async fn rtl_container_starts_at_its_right_edge() {
    let _ = any_spawner::Executor::init_wasm_bindgen();

    let owner = Owner::new();
    owner.set();

    let el = scroll_box("rtl");

    // Documents the scroll model this test relies on.
    assert_eq!(
        el.scroll_left(),
        0.0,
        "an untouched rtl container should sit at scrollLeft 0"
    );

    let UseScrollReturn {
        arrived_state,
        measure,
        ..
    } = use_scroll(el.clone());

    TimeoutFuture::new(50).await;
    measure();

    let arrived = arrived_state.get_untracked();
    assert!(
        arrived.right,
        "scrollLeft 0 in an rtl container is the right edge"
    );
    assert!(
        !arrived.left,
        "the content extends to the left, so the left edge is not reached"
    );
}

// The ordinary left-to-right case must keep working unchanged.
#[wasm_bindgen_test]
async fn ltr_container_starts_at_its_left_edge() {
    let _ = any_spawner::Executor::init_wasm_bindgen();

    let owner = Owner::new();
    owner.set();

    let el = scroll_box("ltr");

    let UseScrollReturn {
        arrived_state,
        measure,
        ..
    } = use_scroll(el.clone());

    TimeoutFuture::new(50).await;
    measure();

    let arrived = arrived_state.get_untracked();
    assert!(
        arrived.left,
        "scrollLeft 0 in an ltr container is the left edge"
    );
    assert!(!arrived.right, "the content extends to the right");
}
