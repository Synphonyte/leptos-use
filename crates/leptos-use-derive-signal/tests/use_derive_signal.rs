#![cfg(target_arch = "wasm32")]

use leptos::prelude::*;
use leptos_use_derive_signal::use_derive_signal;
use wasm_bindgen_test::*;

wasm_bindgen_test_configure!(run_in_browser);

use_derive_signal!(
    /// Length of the contained string
    use_len<String> -> usize
    |value| value.len()
);

use_derive_signal!(
    use_count<Vec<T>, T: 'static> -> usize
    |value| value.len()
);

#[wasm_bindgen_test]
fn derives_a_signal_from_a_signal() {
    let owner = Owner::new();
    owner.set();

    let (text, set_text) = signal("four".to_string());
    let len = use_len(text);

    assert_eq!(len.get_untracked(), 4);

    set_text.set("eleven char".to_string());
    assert_eq!(len.get_untracked(), 11);
}

#[wasm_bindgen_test]
fn derives_a_signal_with_type_parameters() {
    let owner = Owner::new();
    owner.set();

    let (items, set_items) = signal(Vec::<u8>::new());
    let count = use_count::<_, u8>(items);

    assert_eq!(count.get_untracked(), 0);

    set_items.set(vec![7, 8]);
    assert_eq!(count.get_untracked(), 2);
}

#[wasm_bindgen_test]
fn accepts_plain_values() {
    let owner = Owner::new();
    owner.set();

    assert_eq!(use_len("abc".to_string()).get_untracked(), 3);
}
