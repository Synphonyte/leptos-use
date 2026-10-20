#![cfg(target_arch = "wasm32")]

use js_sys::{Object, Promise};
use wasm_bindgen::JsValue;
use wasm_bindgen_test::*;
use wasm_js_macros::{js, js_fut, js_value_from_to_string};

wasm_bindgen_test_configure!(run_in_browser);

#[wasm_bindgen_test]
fn sets_and_gets_object_properties() {
    let obj = Object::new();

    assert_eq!(js!(obj["answer"] = 42), Ok(true));
    assert_eq!(js!(obj["answer"]), Ok(JsValue::from(42)));
    assert_eq!(js!(obj["missing"]), Ok(JsValue::UNDEFINED));
}

#[wasm_bindgen_test]
fn checks_whether_an_attribute_is_in_an_object() {
    let obj = Object::new();
    js!(obj["present"] = true).unwrap();

    let key = "present";
    assert!(js!("present" in &obj));
    assert!(js!([key] in &obj));
    assert!(!js!("absent" in &obj));
}

#[wasm_bindgen_test]
fn finds_browser_apis_on_the_global_object() {
    let global = js_sys::global();

    assert!(js!("document" in &global));
    assert!(!js!("definitelyNotABrowserApi" in &global));
}

#[wasm_bindgen_test]
async fn turns_a_promise_into_a_future() {
    let value = js_fut!(Promise::resolve(&JsValue::from("resolved")))
        .await
        .unwrap();

    assert_eq!(value, JsValue::from("resolved"));
}

struct Greeting(&'static str);

impl std::fmt::Display for Greeting {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "hello {}", self.0)
    }
}

js_value_from_to_string!(Greeting);

#[wasm_bindgen_test]
fn converts_display_types_into_js_strings() {
    let value: JsValue = Greeting("world").into();

    assert_eq!(value.as_string().as_deref(), Some("hello world"));
}
