#![cfg(target_arch = "wasm32")]

use any_spawner::Executor;
use js_sys::{Array, Object, Reflect};
use leptos::prelude::*;
use leptos::wasm_bindgen::JsValue;
use leptos_use_locales::use_locales;
use wasm_bindgen_test::*;

wasm_bindgen_test_configure!(run_in_browser);

/// Shadows `navigator.languages`, which can't be changed in a headless browser.
fn set_languages(languages: &[&str]) {
    let value = languages
        .iter()
        .map(|l| JsValue::from(*l))
        .collect::<Array>();
    let descriptor = Object::new();
    Reflect::set(&descriptor, &"configurable".into(), &true.into()).unwrap();
    Reflect::set(&descriptor, &"value".into(), &value).unwrap();
    Object::define_property(
        &window().navigator().into(),
        &"languages".into(),
        &descriptor,
    );
}

fn reset_languages() {
    Reflect::delete_property(&window().navigator().into(), &"languages".into()).unwrap();
}

#[wasm_bindgen_test]
async fn reports_the_browser_languages() {
    let _ = Executor::init_wasm_bindgen();
    let owner = Owner::new();
    owner.set();

    let locales = use_locales();

    let expected: Vec<String> = window()
        .navigator()
        .languages()
        .iter()
        .filter_map(|l| l.as_string())
        .collect();
    assert!(!expected.is_empty());
    assert_eq!(locales.get_untracked(), expected);
}

#[wasm_bindgen_test]
async fn follows_language_changes() {
    let _ = Executor::init_wasm_bindgen();
    let owner = Owner::new();
    owner.set();

    set_languages(&["fr-FR", "en"]);
    let locales = use_locales();
    assert_eq!(locales.get_untracked(), vec!["fr-FR", "en"]);
    Executor::tick().await;

    set_languages(&["de-AT"]);
    window()
        .dispatch_event(&web_sys::Event::new("languagechange").unwrap())
        .unwrap();
    assert_eq!(locales.get_untracked(), vec!["de-AT"]);

    reset_languages();
}
