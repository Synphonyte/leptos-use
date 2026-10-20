#![cfg(target_arch = "wasm32")]

use any_spawner::Executor;
use js_sys::{Object, Reflect};
use leptos::prelude::*;
use leptos::wasm_bindgen::JsCast;
use leptos_use_document_visibility::use_document_visibility;
use wasm_bindgen_test::*;

wasm_bindgen_test_configure!(run_in_browser);

/// Headless browsers can't hide the tab, so the state is shadowed on the
/// document instance and the change event is dispatched by hand.
fn change_visibility_state(state: &str) {
    let descriptor = Object::new();
    Reflect::set(&descriptor, &"configurable".into(), &true.into()).unwrap();
    Reflect::set(&descriptor, &"value".into(), &state.into()).unwrap();
    Object::define_property(
        &document().unchecked_into::<Object>(),
        &"visibilityState".into(),
        &descriptor,
    );

    document()
        .dispatch_event(&web_sys::Event::new("visibilitychange").unwrap())
        .unwrap();
}

#[wasm_bindgen_test]
async fn starts_with_the_current_visibility() {
    let _ = Executor::init_wasm_bindgen();
    let owner = Owner::new();
    owner.set();

    let visibility = use_document_visibility();

    assert_eq!(visibility.get_untracked(), document().visibility_state());
}

#[wasm_bindgen_test]
async fn follows_visibility_changes() {
    let _ = Executor::init_wasm_bindgen();
    let owner = Owner::new();
    owner.set();

    let visibility = use_document_visibility();
    Executor::tick().await;

    change_visibility_state("hidden");
    assert_eq!(visibility.get_untracked(), web_sys::VisibilityState::Hidden);

    change_visibility_state("visible");
    assert_eq!(
        visibility.get_untracked(),
        web_sys::VisibilityState::Visible
    );

    Reflect::delete_property(
        &document().unchecked_into::<Object>(),
        &"visibilityState".into(),
    )
    .unwrap();
}
