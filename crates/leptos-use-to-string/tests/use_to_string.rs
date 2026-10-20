#![cfg(target_arch = "wasm32")]

use leptos::prelude::*;
use leptos_use_to_string::use_to_string;
use wasm_bindgen_test::*;

wasm_bindgen_test_configure!(run_in_browser);

#[wasm_bindgen_test]
fn converts_the_current_value_to_a_string() {
    let owner = Owner::new();
    owner.set();

    let (number, set_number) = signal(3.5);
    let text = use_to_string::<_, f64>(number);

    assert_eq!(text.get_untracked(), "3.5");

    set_number.set(-12.0);
    assert_eq!(text.get_untracked(), "-12");
}

#[wasm_bindgen_test]
fn accepts_plain_values() {
    let owner = Owner::new();
    owner.set();

    let text = use_to_string::<_, bool>(true);

    assert_eq!(text.get_untracked(), "true");
}
