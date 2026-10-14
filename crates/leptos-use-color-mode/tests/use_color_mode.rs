#![cfg(target_arch = "wasm32")]

use any_spawner::Executor;
use gloo_timers::future::TimeoutFuture;
use leptos::prelude::*;
use leptos_use_color_mode::{
    ColorMode, UseColorModeOptions, UseColorModeReturn, use_color_mode_with_options,
};
use wasm_bindgen_test::*;

wasm_bindgen_test_configure!(run_in_browser);

/// The target option takes a selector, so every element gets an id.
fn append_div(id: &str) -> web_sys::Element {
    let el = document().create_element("div").unwrap();
    el.set_id(id);
    document().body().unwrap().append_child(&el).unwrap();
    el
}

fn local() -> web_sys::Storage {
    window().local_storage().unwrap().unwrap()
}

fn system_mode() -> ColorMode {
    if window()
        .match_media("(prefers-color-scheme: dark)")
        .unwrap()
        .unwrap()
        .matches()
    {
        ColorMode::Dark
    } else {
        ColorMode::Light
    }
}

async fn setup() -> Owner {
    let _ = Executor::init_wasm_bindgen();
    let owner = Owner::new();
    owner.set();
    owner
}

#[wasm_bindgen_test]
async fn applies_the_mode_as_a_class() {
    let _owner = setup().await;

    let target = append_div("color-mode-class");
    let UseColorModeReturn {
        mode,
        set_mode,
        store,
        system,
        ..
    } = use_color_mode_with_options(
        UseColorModeOptions::default()
            .target("#color-mode-class")
            .storage_enabled(false),
    );
    TimeoutFuture::new(20).await;

    assert_eq!(store.get_untracked(), ColorMode::Auto);
    assert_eq!(system.get_untracked(), system_mode());
    assert_eq!(
        mode.get_untracked(),
        system_mode(),
        "auto resolves to the system mode"
    );

    set_mode.set(ColorMode::Dark);
    TimeoutFuture::new(20).await;
    assert_eq!(mode.get_untracked(), ColorMode::Dark);
    assert!(target.class_list().contains("dark"));
    assert!(!target.class_list().contains("light"));

    set_mode.set(ColorMode::Light);
    TimeoutFuture::new(20).await;
    assert!(target.class_list().contains("light"));
    assert!(!target.class_list().contains("dark"));
}

#[wasm_bindgen_test]
async fn applies_the_mode_to_a_custom_attribute() {
    let _owner = setup().await;

    let target = append_div("color-mode-attribute");
    let UseColorModeReturn { set_mode, .. } = use_color_mode_with_options(
        UseColorModeOptions::default()
            .target("#color-mode-attribute")
            .attribute("data-theme")
            .custom_modes(vec!["sepia".to_string()])
            .storage_enabled(false),
    );
    TimeoutFuture::new(20).await;

    set_mode.set(ColorMode::Custom("sepia".to_string()));
    TimeoutFuture::new(20).await;

    assert_eq!(target.get_attribute("data-theme").as_deref(), Some("sepia"));
}

#[wasm_bindgen_test]
async fn emit_auto_reports_auto_instead_of_the_resolved_mode() {
    let _owner = setup().await;

    append_div("color-mode-emit-auto");
    let UseColorModeReturn { mode, state, .. } = use_color_mode_with_options(
        UseColorModeOptions::default()
            .target("#color-mode-emit-auto")
            .emit_auto(true)
            .storage_enabled(false),
    );
    TimeoutFuture::new(20).await;

    assert_eq!(mode.get_untracked(), ColorMode::Auto);
    assert_eq!(state.get_untracked(), system_mode());
}

#[wasm_bindgen_test]
async fn persists_the_mode_in_local_storage() {
    let _owner = setup().await;
    local().remove_item("color-mode-test").unwrap();
    append_div("color-mode-persist-a");
    append_div("color-mode-persist-b");

    let UseColorModeReturn { set_mode, .. } = use_color_mode_with_options(
        UseColorModeOptions::default()
            .target("#color-mode-persist-a")
            .storage_key("color-mode-test"),
    );
    TimeoutFuture::new(20).await;

    set_mode.set(ColorMode::Dark);
    TimeoutFuture::new(50).await;
    assert_eq!(
        local().get_item("color-mode-test").unwrap().as_deref(),
        Some("dark")
    );

    // a new instance starts with the persisted mode
    let UseColorModeReturn { store, .. } = use_color_mode_with_options(
        UseColorModeOptions::default()
            .target("#color-mode-persist-b")
            .storage_key("color-mode-test"),
    );
    TimeoutFuture::new(20).await;
    assert_eq!(store.get_untracked(), ColorMode::Dark);

    local().remove_item("color-mode-test").unwrap();
}

#[wasm_bindgen_test]
fn parses_and_displays_modes() {
    assert_eq!(ColorMode::from("dark"), ColorMode::Dark);
    assert_eq!(ColorMode::from("light"), ColorMode::Light);
    assert_eq!(ColorMode::from(""), ColorMode::Auto);
    assert_eq!(
        ColorMode::from("sepia"),
        ColorMode::Custom("sepia".to_string())
    );
    assert_eq!(ColorMode::Dark.to_string(), "dark");
}
