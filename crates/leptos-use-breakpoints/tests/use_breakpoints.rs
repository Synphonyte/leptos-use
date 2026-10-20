#![cfg(target_arch = "wasm32")]

use any_spawner::Executor;
use gloo_timers::future::TimeoutFuture;
use leptos::prelude::*;
use leptos_use_breakpoints::{BreakpointsTailwind, breakpoints_tailwind, use_breakpoints};
use std::collections::HashMap;
use wasm_bindgen_test::*;

wasm_bindgen_test_configure!(run_in_browser);

#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash)]
enum Bp {
    Below,
    Exact,
    Above,
}

/// Breakpoints placed around the current viewport width.
fn breakpoints() -> HashMap<Bp, u32> {
    let width = window().inner_width().unwrap().as_f64().unwrap() as u32;
    HashMap::from([
        (Bp::Below, width - 100),
        (Bp::Exact, width),
        (Bp::Above, width + 100),
    ])
}

#[wasm_bindgen_test]
async fn compares_the_viewport_with_the_breakpoints() {
    let _ = Executor::init_wasm_bindgen();
    let owner = Owner::new();
    owner.set();

    let screen = use_breakpoints(breakpoints());

    let gt_below = screen.gt(Bp::Below);
    let ge_exact = screen.ge(Bp::Exact);
    let gt_exact = screen.gt(Bp::Exact);
    let le_exact = screen.le(Bp::Exact);
    let lt_exact = screen.lt(Bp::Exact);
    let lt_above = screen.lt(Bp::Above);
    let ge_above = screen.ge(Bp::Above);
    TimeoutFuture::new(20).await;

    assert!(gt_below.get_untracked());
    assert!(ge_exact.get_untracked());
    assert!(!gt_exact.get_untracked());
    assert!(le_exact.get_untracked());
    assert!(!lt_exact.get_untracked());
    assert!(lt_above.get_untracked());
    assert!(!ge_above.get_untracked());

    assert!(screen.is_gt(Bp::Below));
    assert!(screen.is_ge(Bp::Exact));
    assert!(!screen.is_lt(Bp::Exact));
    assert!(screen.is_le(Bp::Exact));
}

#[wasm_bindgen_test]
async fn between_is_inclusive_of_min_and_exclusive_of_max() {
    let _ = Executor::init_wasm_bindgen();
    let owner = Owner::new();
    owner.set();

    let screen = use_breakpoints(breakpoints());

    let below_above = screen.between(Bp::Below, Bp::Above);
    let below_exact = screen.between(Bp::Below, Bp::Exact);
    let exact_above = screen.between(Bp::Exact, Bp::Above);
    TimeoutFuture::new(20).await;

    assert!(below_above.get_untracked());
    assert!(!below_exact.get_untracked());
    assert!(exact_above.get_untracked());
    assert!(screen.is_between(Bp::Exact, Bp::Above));
}

#[wasm_bindgen_test]
async fn current_lists_the_reached_breakpoints() {
    let _ = Executor::init_wasm_bindgen();
    let owner = Owner::new();
    owner.set();

    let screen = use_breakpoints(breakpoints());
    let current = screen.current();
    TimeoutFuture::new(20).await;

    let mut current = current.get_untracked();
    current.sort_by_key(|bp| *bp as u8);
    assert_eq!(current, vec![Bp::Below, Bp::Exact]);
}

#[wasm_bindgen_test]
fn presets_have_ascending_values() {
    let tailwind = breakpoints_tailwind();

    assert_eq!(tailwind[&BreakpointsTailwind::Sm], 640);
    assert!(tailwind[&BreakpointsTailwind::Sm] < tailwind[&BreakpointsTailwind::Md]);
    assert!(tailwind[&BreakpointsTailwind::Xl] < tailwind[&BreakpointsTailwind::Xxl]);
}
