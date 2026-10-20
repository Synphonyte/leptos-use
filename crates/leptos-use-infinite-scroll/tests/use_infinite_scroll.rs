#![cfg(target_arch = "wasm32")]

use any_spawner::Executor;
use gloo_timers::future::TimeoutFuture;
use leptos::prelude::*;
use leptos_use_infinite_scroll::{UseInfiniteScrollOptions, use_infinite_scroll_with_options};
use send_wrapper::SendWrapper;
use std::sync::Arc;
use std::sync::atomic::{AtomicUsize, Ordering};
use wasm_bindgen_test::*;

wasm_bindgen_test_configure!(run_in_browser);

fn append_item(list: &web_sys::Element) {
    let item = document().create_element("div").unwrap();
    item.set_attribute("style", "height: 50px;").unwrap();
    list.append_child(&item).unwrap();
}

#[wasm_bindgen_test]
async fn loads_more_when_unfilled_and_again_at_the_bottom() {
    let _ = Executor::init_wasm_bindgen();
    let owner = Owner::new();
    owner.set();

    let list = document().create_element("div").unwrap();
    list.set_attribute("style", "height: 200px; overflow-y: scroll;")
        .unwrap();
    document().body().unwrap().append_child(&list).unwrap();
    append_item(&list);

    let loads = Arc::new(AtomicUsize::new(0));

    let is_loading = use_infinite_scroll_with_options(
        list.clone(),
        {
            let loads = Arc::clone(&loads);
            let list = SendWrapper::new(list.clone());
            move |_| {
                loads.fetch_add(1, Ordering::SeqCst);
                for _ in 0..5 {
                    append_item(&list);
                }
                async {}
            }
        },
        UseInfiniteScrollOptions::default().interval(10.0),
    );

    // The single item doesn't fill the list, so more is loaded right away.
    TimeoutFuture::new(400).await;
    assert!(!is_loading.get_untracked());
    let initial_loads = loads.load(Ordering::SeqCst);
    assert_eq!(initial_loads, 1, "the unfilled list loads more");
    assert!(
        list.scroll_height() > list.client_height(),
        "the list is filled"
    );

    TimeoutFuture::new(200).await;
    assert_eq!(
        loads.load(Ordering::SeqCst),
        initial_loads,
        "a filled list stops loading"
    );

    list.set_scroll_top(list.scroll_height() as f64);
    TimeoutFuture::new(400).await;
    assert!(
        loads.load(Ordering::SeqCst) > initial_loads,
        "reaching the bottom loads more"
    );
}

// `scrollHeight` never drops below `clientHeight`, so it doesn't grow while
// the content is still shorter than the list. Loading must continue anyway.
#[wasm_bindgen_test]
async fn keeps_loading_until_the_list_is_filled() {
    let _ = Executor::init_wasm_bindgen();
    let owner = Owner::new();
    owner.set();

    let list = document().create_element("div").unwrap();
    list.set_attribute("style", "height: 200px; overflow-y: scroll;")
        .unwrap();
    document().body().unwrap().append_child(&list).unwrap();
    append_item(&list);

    let _ = use_infinite_scroll_with_options(
        list.clone(),
        {
            let list = SendWrapper::new(list.clone());
            move |_| {
                append_item(&list);
                async {}
            }
        },
        UseInfiniteScrollOptions::default().interval(10.0),
    );

    TimeoutFuture::new(600).await;
    assert!(
        list.scroll_height() > list.client_height(),
        "loading continues until the list overflows"
    );
}

// A load that adds nothing must not trigger the next one, or an exhausted
// data source would be asked again every `interval` forever.
#[wasm_bindgen_test]
async fn stops_loading_when_nothing_new_arrives() {
    let _ = Executor::init_wasm_bindgen();
    let owner = Owner::new();
    owner.set();

    let list = document().create_element("div").unwrap();
    list.set_attribute("style", "height: 200px; overflow-y: scroll;")
        .unwrap();
    document().body().unwrap().append_child(&list).unwrap();
    append_item(&list);

    let loads = Arc::new(AtomicUsize::new(0));
    let _ = use_infinite_scroll_with_options(
        list.clone(),
        {
            let loads = Arc::clone(&loads);
            move |_| {
                loads.fetch_add(1, Ordering::SeqCst);
                async {}
            }
        },
        UseInfiniteScrollOptions::default().interval(10.0),
    );

    TimeoutFuture::new(400).await;
    assert_eq!(loads.load(Ordering::SeqCst), 1);
}
