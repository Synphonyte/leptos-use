#![cfg(target_arch = "wasm32")]

use leptos::prelude::*;
use leptos_use_toggle::{UseToggleReturn, use_toggle};
use wasm_bindgen_test::*;

wasm_bindgen_test_configure!(run_in_browser);

#[wasm_bindgen_test]
fn toggle_flips_the_value() {
    let owner = Owner::new();
    owner.set();

    let UseToggleReturn {
        toggle,
        value,
        set_value,
    } = use_toggle(false);

    assert!(!value.get_untracked());

    toggle();
    assert!(value.get_untracked());

    toggle();
    assert!(!value.get_untracked());

    set_value.set(true);
    assert!(value.get_untracked());
    toggle();
    assert!(!value.get_untracked());
}

// When given a signal, toggling writes through to it.
#[wasm_bindgen_test]
fn toggle_writes_through_to_a_passed_signal() {
    let owner = Owner::new();
    owner.set();

    let (flag, set_flag) = signal(true);

    let UseToggleReturn { toggle, value, .. } = use_toggle((flag, set_flag));

    toggle();
    assert!(!flag.get_untracked());
    assert!(!value.get_untracked());

    set_flag.set(true);
    assert!(value.get_untracked());
}
