#![cfg(target_arch = "wasm32")]

use any_spawner::Executor;
use js_sys::{Array, Object, Reflect};
use leptos::prelude::*;
use leptos::wasm_bindgen::JsValue;
use leptos_use_locale::use_locale;
use unic_langid::{langid, langid_slice};
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
async fn picks_the_first_exact_match() {
    let _ = Executor::init_wasm_bindgen();
    let owner = Owner::new();
    owner.set();

    set_languages(&["it", "fr", "de"]);
    let locale = use_locale(langid_slice!["en", "de", "fr"]);

    assert_eq!(locale.get_untracked(), langid!("fr"));
    reset_languages();
}

#[wasm_bindgen_test]
async fn matches_regional_variants() {
    let _ = Executor::init_wasm_bindgen();
    let owner = Owner::new();
    owner.set();

    set_languages(&["de-AT", "en-US"]);
    let locale = use_locale(langid_slice!["en", "de"]);

    assert_eq!(locale.get_untracked(), langid!("de"));
    reset_languages();
}

#[wasm_bindgen_test]
async fn falls_back_to_the_first_supported_locale() {
    let _ = Executor::init_wasm_bindgen();
    let owner = Owner::new();
    owner.set();

    set_languages(&["ja"]);
    let locale = use_locale(langid_slice!["en", "de"]);

    assert_eq!(locale.get_untracked(), langid!("en"));
    reset_languages();
}
