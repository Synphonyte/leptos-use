#![cfg(target_arch = "wasm32")]

use sendwrap_fn::sendwrap_fn;
use std::cell::Cell;
use std::rc::Rc;
use wasm_bindgen_test::*;

wasm_bindgen_test_configure!(run_in_browser);

fn assert_send_sync<T: Send + Sync>(_: &T) {}

// The wrapped closure captures an `Rc`, which is neither `Send` nor `Sync`.
// The macro must still produce a `Send + Sync` closure that forwards its
// arguments and can be called repeatedly.
#[wasm_bindgen_test]
fn wraps_a_non_send_closure_with_typed_params() {
    let total = Rc::new(Cell::new(0));

    let add = sendwrap_fn!(move |a: i32, b: i32| {
        total.set(total.get() + a + b);
        total.get()
    });

    assert_send_sync(&add);
    assert_eq!(add(1, 2), 3);
    assert_eq!(add(10, 20), 33);
}

#[wasm_bindgen_test]
fn wraps_a_closure_without_params() {
    let calls = Rc::new(Cell::new(0));

    let f = sendwrap_fn!(move || {
        calls.set(calls.get() + 1);
        calls.get()
    });

    assert_send_sync(&f);
    assert_eq!(f(), 1);
    assert_eq!(f(), 2);
}

#[wasm_bindgen_test]
fn wraps_an_fn_once_closure() {
    let owned = Rc::new(String::from("moved out"));

    let f = sendwrap_fn!(once move |suffix: &str| format!("{} {}", Rc::try_unwrap(owned).unwrap(), suffix));

    assert_send_sync(&f);
    assert_eq!(f("once"), "moved out once");
}

#[wasm_bindgen_test]
fn wraps_an_fn_once_closure_without_params() {
    let owned = Rc::new(vec![1, 2, 3]);

    let f = sendwrap_fn!(once move || Rc::try_unwrap(owned).unwrap());

    assert_send_sync(&f);
    assert_eq!(f(), vec![1, 2, 3]);
}
