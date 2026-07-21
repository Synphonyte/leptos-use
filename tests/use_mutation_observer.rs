#![cfg(target_arch = "wasm32")]

use gloo_timers::future::TimeoutFuture;
use leptos::prelude::*;
use leptos_use::{UseMutationObserverOptions, use_mutation_observer_with_options};
use std::cell::Cell;
use std::rc::Rc;
use wasm_bindgen_test::*;

wasm_bindgen_test_configure!(run_in_browser);

fn append_div() -> web_sys::Element {
    let document = document();
    let el = document
        .create_element("div")
        .expect("failed to create div");
    document
        .body()
        .expect("no body")
        .append_child(&el)
        .expect("failed to append");
    el
}

// `attributes` is written to the options object unconditionally, so it is
// never absent and defaults to an explicit `false`. The DOM rejects
// `observe()` with a TypeError when `attributeFilter` or `attributeOldValue`
// is present while `attributes` is false, and that error was discarded, so
// configuring only a filter produced an observer that silently never fired.
#[wasm_bindgen_test]
async fn attribute_filter_alone_observes_attribute_changes() {
    let _ = any_spawner::Executor::init_wasm_bindgen();

    let owner = Owner::new();
    owner.set();

    let el = append_div();
    let fired = Rc::new(Cell::new(0_u32));

    use_mutation_observer_with_options(
        el.clone(),
        {
            let fired = Rc::clone(&fired);
            move |_, _| fired.set(fired.get() + 1)
        },
        UseMutationObserverOptions::default().attribute_filter(vec!["class".to_string()]),
    );

    TimeoutFuture::new(50).await;

    el.set_attribute("class", "changed")
        .expect("failed to set class");

    TimeoutFuture::new(100).await;

    assert!(
        fired.get() > 0,
        "an attribute_filter alone must observe attribute changes"
    );
}

// The same applies to `attribute_old_value`, which the DOM also rejects while
// `attributes` is explicitly false.
#[wasm_bindgen_test]
async fn attribute_old_value_alone_observes_attribute_changes() {
    let _ = any_spawner::Executor::init_wasm_bindgen();

    let owner = Owner::new();
    owner.set();

    let el = append_div();
    let fired = Rc::new(Cell::new(0_u32));

    use_mutation_observer_with_options(
        el.clone(),
        {
            let fired = Rc::clone(&fired);
            move |_, _| fired.set(fired.get() + 1)
        },
        UseMutationObserverOptions::default().attribute_old_value(true),
    );

    TimeoutFuture::new(50).await;

    el.set_attribute("data-state", "open")
        .expect("failed to set attribute");

    TimeoutFuture::new(100).await;

    assert!(
        fired.get() > 0,
        "attribute_old_value alone must observe attribute changes"
    );
}
