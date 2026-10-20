#![cfg(target_arch = "wasm32")]

use any_spawner::Executor;
use gloo_timers::future::TimeoutFuture;
use leptos::prelude::*;
use leptos_use_permission::{PermissionState, use_permission};
use wasm_bindgen_test::*;

wasm_bindgen_test_configure!(run_in_browser);

/// Asks the Permissions API directly for comparison.
async fn query(name: &str) -> PermissionState {
    let descriptor = js_sys::Object::new();
    js_sys::Reflect::set(&descriptor, &"name".into(), &name.into()).unwrap();
    let status = wasm_bindgen_futures::JsFuture::from(
        window()
            .navigator()
            .permissions()
            .unwrap()
            .query(&descriptor)
            .unwrap(),
    )
    .await
    .unwrap();
    let status: web_sys::PermissionStatus = leptos::wasm_bindgen::JsCast::unchecked_into(status);
    status.state().into()
}

#[wasm_bindgen_test]
async fn reports_the_permission_state() {
    let _ = Executor::init_wasm_bindgen();
    let owner = Owner::new();
    owner.set();

    let geolocation = use_permission("geolocation");
    assert!(
        geolocation.get_untracked() == PermissionState::Unknown,
        "queried asynchronously"
    );

    TimeoutFuture::new(100).await;
    let expected = query("geolocation").await;
    assert!(geolocation.get_untracked() == expected);
    assert_ne!(geolocation.get_untracked().to_string(), "unknown");
}

#[wasm_bindgen_test]
async fn unknown_permissions_fall_back_to_prompt() {
    let _ = Executor::init_wasm_bindgen();
    let owner = Owner::new();
    owner.set();

    let state = use_permission("definitely-not-a-permission");
    TimeoutFuture::new(100).await;

    assert_eq!(state.get_untracked().to_string(), "prompt");
}
