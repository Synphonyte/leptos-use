#![cfg(target_arch = "wasm32")]

use leptos::prelude::*;
use leptos_use_math::{
    use_abs, use_and, use_ceil, use_floor, use_max, use_min, use_not, use_or, use_round,
};
use wasm_bindgen_test::*;

wasm_bindgen_test_configure!(run_in_browser);

#[wasm_bindgen_test]
fn simple_math_follows_the_signal() {
    let owner = Owner::new();
    owner.set();

    let (x, set_x) = signal(-2.5_f64);

    let abs = use_abs::<_, f64>(x);
    let ceil = use_ceil::<_, f64>(x);
    let floor = use_floor::<_, f64>(x);
    let round = use_round::<_, f64>(x);

    assert_eq!(abs.get_untracked(), 2.5);
    assert_eq!(ceil.get_untracked(), -2.0);
    assert_eq!(floor.get_untracked(), -3.0);
    assert_eq!(round.get_untracked(), -3.0);

    set_x.set(1.4);

    assert_eq!(abs.get_untracked(), 1.4);
    assert_eq!(ceil.get_untracked(), 2.0);
    assert_eq!(floor.get_untracked(), 1.0);
    assert_eq!(round.get_untracked(), 1.0);
}

#[wasm_bindgen_test]
fn min_and_max_follow_the_container() {
    let owner = Owner::new();
    owner.set();

    let (values, set_values) = signal(vec![3, 9, -1, 4]);

    let max = use_max::<Vec<i32>, _, i32>(values);
    let min = use_min::<Vec<i32>, _, i32>(values);

    assert_eq!(max.get_untracked(), Some(9));
    assert_eq!(min.get_untracked(), Some(-1));

    set_values.update(|v| v.push(12));
    assert_eq!(max.get_untracked(), Some(12));

    set_values.set(vec![]);
    assert_eq!(max.get_untracked(), None);
    assert_eq!(min.get_untracked(), None);
}

#[wasm_bindgen_test]
fn logic_follows_the_signals() {
    let owner = Owner::new();
    owner.set();

    let (a, set_a) = signal(false);
    let (b, set_b) = signal(true);

    let and = use_and(a, b);
    let or = use_or(a, b);
    let not = use_not(a);

    assert!(!and.get_untracked());
    assert!(or.get_untracked());
    assert!(not.get_untracked());

    set_a.set(true);
    assert!(and.get_untracked());
    assert!(!not.get_untracked());

    set_a.set(false);
    set_b.set(false);
    assert!(!or.get_untracked());
}
