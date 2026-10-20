#![cfg(target_arch = "wasm32")]

use leptos::prelude::*;
use leptos_use_sorted::{use_sorted, use_sorted_by, use_sorted_by_key};
use wasm_bindgen_test::*;

wasm_bindgen_test_configure!(run_in_browser);

#[wasm_bindgen_test]
fn sorts_and_follows_the_source() {
    let owner = Owner::new();
    owner.set();

    let (source, set_source) = signal(vec![10, 3, 5, 7, 2]);
    let sorted: Signal<Vec<i32>> = use_sorted(source);

    assert_eq!(sorted.get_untracked(), vec![2, 3, 5, 7, 10]);

    set_source.update(|v| v.push(1));
    assert_eq!(sorted.get_untracked(), vec![1, 2, 3, 5, 7, 10]);
    assert_eq!(
        source.get_untracked(),
        vec![10, 3, 5, 7, 2, 1],
        "the source stays unsorted"
    );
}

#[wasm_bindgen_test]
fn sorts_with_a_compare_function() {
    let owner = Owner::new();
    owner.set();

    let sorted: Signal<Vec<i32>> = use_sorted_by(vec![1, 3, 2], |a: &i32, b: &i32| b.cmp(a));

    assert_eq!(sorted.get_untracked(), vec![3, 2, 1]);
}

#[wasm_bindgen_test]
fn sorts_by_key() {
    let owner = Owner::new();
    owner.set();

    let sorted: Signal<Vec<(&str, u32)>> = use_sorted_by_key(
        vec![("c", 30), ("a", 10), ("b", 20)],
        |(_, age): &(&str, u32)| *age,
    );

    assert_eq!(
        sorted.get_untracked(),
        vec![("a", 10), ("b", 20), ("c", 30)]
    );
}
