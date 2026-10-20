#![cfg(target_arch = "wasm32")]

use any_spawner::Executor;
use gloo_timers::future::TimeoutFuture;
use leptos::prelude::*;
use leptos_use_media_query::use_media_query;
use wasm_bindgen_test::*;

wasm_bindgen_test_configure!(run_in_browser);

fn viewport_width() -> u32 {
    window().inner_width().unwrap().as_f64().unwrap() as u32
}

#[wasm_bindgen_test]
async fn evaluates_the_query() {
    let _ = Executor::init_wasm_bindgen();
    let owner = Owner::new();
    owner.set();

    let width = viewport_width();
    let wide_enough = use_media_query(format!("(min-width: {}px)", width / 2));
    let too_wide = use_media_query(format!("(min-width: {}px)", width * 2));
    TimeoutFuture::new(20).await;

    assert!(wide_enough.get_untracked());
    assert!(!too_wide.get_untracked());
}

#[wasm_bindgen_test]
async fn follows_a_reactive_query() {
    let _ = Executor::init_wasm_bindgen();
    let owner = Owner::new();
    owner.set();

    let width = viewport_width();
    let (query, set_query) = signal(format!("(max-width: {}px)", width / 2));
    let matches = use_media_query(query);
    TimeoutFuture::new(20).await;
    assert!(!matches.get_untracked());

    set_query.set(format!("(max-width: {}px)", width * 2));
    TimeoutFuture::new(20).await;
    assert!(matches.get_untracked());
}

#[wasm_bindgen_test]
async fn agrees_with_match_media() {
    let _ = Executor::init_wasm_bindgen();
    let owner = Owner::new();
    owner.set();

    let query = "(prefers-color-scheme: dark)";
    let matches = use_media_query(query);
    TimeoutFuture::new(20).await;

    assert_eq!(
        matches.get_untracked(),
        window().match_media(query).unwrap().unwrap().matches()
    );
}
