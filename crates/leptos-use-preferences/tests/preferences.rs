#![cfg(target_arch = "wasm32")]

use any_spawner::Executor;
use gloo_timers::future::TimeoutFuture;
use leptos::prelude::*;
use leptos_use_preferences::{
    PreferredContrast, use_preferred_contrast, use_preferred_dark, use_prefers_reduced_motion,
};
use wasm_bindgen_test::*;

wasm_bindgen_test_configure!(run_in_browser);

fn matches(query: &str) -> bool {
    window().match_media(query).unwrap().unwrap().matches()
}

#[wasm_bindgen_test]
async fn preferred_dark_matches_the_color_scheme() {
    let _ = Executor::init_wasm_bindgen();
    let owner = Owner::new();
    owner.set();

    let is_dark = use_preferred_dark();
    TimeoutFuture::new(20).await;

    assert_eq!(
        is_dark.get_untracked(),
        matches("(prefers-color-scheme: dark)")
    );
}

#[wasm_bindgen_test]
async fn reduced_motion_matches_the_media_feature() {
    let _ = Executor::init_wasm_bindgen();
    let owner = Owner::new();
    owner.set();

    let reduced = use_prefers_reduced_motion();
    TimeoutFuture::new(20).await;

    assert_eq!(
        reduced.get_untracked(),
        matches("(prefers-reduced-motion: reduce)")
    );
}

#[wasm_bindgen_test]
async fn preferred_contrast_matches_the_media_feature() {
    let _ = Executor::init_wasm_bindgen();
    let owner = Owner::new();
    owner.set();

    let contrast = use_preferred_contrast();
    TimeoutFuture::new(20).await;

    let expected = if matches("(prefers-contrast: more)") {
        PreferredContrast::More
    } else if matches("(prefers-contrast: less)") {
        PreferredContrast::Less
    } else if matches("(prefers-contrast: custom)") {
        PreferredContrast::Custom
    } else {
        PreferredContrast::NoPreference
    };
    assert_eq!(contrast.get_untracked(), expected);
}
