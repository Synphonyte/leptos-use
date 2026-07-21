#![cfg(target_arch = "wasm32")]

use gloo_timers::future::TimeoutFuture;
use leptos::prelude::*;
use leptos_use::{UseCssVarOptions, use_css_var_with_options};
use wasm_bindgen::JsCast;
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

// `initial_value` is documented as "the default value if the variable isn't
// defined on the target". `getPropertyValue` never fails for an undefined
// custom property - it returns an empty string - so the `Ok(..)` arm always
// won and the fallback was unreachable. Callers got "" instead of the value
// they configured.
#[wasm_bindgen_test]
async fn falls_back_to_initial_value_when_the_variable_is_undefined() {
    let _ = any_spawner::Executor::init_wasm_bindgen();

    let owner = Owner::new();
    owner.set();

    let el = append_div();

    let (color, _) = use_css_var_with_options(
        "--leptos-use-undefined-color",
        UseCssVarOptions::default()
            .target(el)
            .initial_value("#eee")
            .observe(false),
    );

    TimeoutFuture::new(100).await;

    assert_eq!(
        color.get_untracked(),
        "#eee".to_string(),
        "an undefined custom property must fall back to initial_value"
    );
}

// A variable that is actually set still has to win over `initial_value`.
#[wasm_bindgen_test]
async fn reads_the_variable_when_it_is_defined() {
    let _ = any_spawner::Executor::init_wasm_bindgen();

    let owner = Owner::new();
    owner.set();

    let el = append_div();
    el.unchecked_ref::<web_sys::HtmlElement>()
        .style()
        .set_property("--leptos-use-defined-color", "#123456")
        .expect("failed to set property");

    let (color, _) = use_css_var_with_options(
        "--leptos-use-defined-color",
        UseCssVarOptions::default()
            .target(el)
            .initial_value("#eee")
            .observe(false),
    );

    TimeoutFuture::new(100).await;

    assert_eq!(color.get_untracked(), "#123456".to_string());
}
