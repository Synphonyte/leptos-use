#![cfg(target_arch = "wasm32")]

use any_spawner::Executor;
use leptos::prelude::*;
use leptos_use_maybe_rw_signal::MaybeRwSignal;
use wasm_bindgen_test::*;

wasm_bindgen_test_configure!(run_in_browser);

#[wasm_bindgen_test]
fn a_static_value_becomes_a_fresh_writable_signal() {
    let owner = Owner::new();
    owner.set();

    let value: MaybeRwSignal<i32> = 5.into();
    assert!(matches!(value, MaybeRwSignal::Static(5)));

    let (read, write) = value.into_signal();
    assert_eq!(read.get_untracked(), 5);

    write.set(6);
    assert_eq!(read.get_untracked(), 6);
}

#[wasm_bindgen_test]
fn a_signal_pair_is_used_as_is() {
    let owner = Owner::new();
    owner.set();

    let (a, set_a) = signal(1);
    let (read, write) = MaybeRwSignal::<_>::from((a, set_a)).into_signal();

    write.set(2);
    assert_eq!(a.get_untracked(), 2, "writes go to the original signal");

    set_a.set(3);
    assert_eq!(read.get_untracked(), 3);
}

#[wasm_bindgen_test]
fn an_rw_signal_is_used_as_is() {
    let owner = Owner::new();
    owner.set();

    let rw = RwSignal::new("a".to_string());
    let (read, write) = MaybeRwSignal::<_>::from(rw).into_signal();

    write.set("b".to_string());
    assert_eq!(rw.get_untracked(), "b");
    assert_eq!(read.get_untracked(), "b");
}

// A read-only source is copied into a local signal that keeps following the
// source, while writes to the local signal leave the source untouched.
#[wasm_bindgen_test]
async fn a_read_only_signal_is_followed() {
    let _ = Executor::init_wasm_bindgen();
    let owner = Owner::new();
    owner.set();

    let (source, set_source) = signal(10);
    let (read, write) = MaybeRwSignal::<_>::from(source).into_signal();
    assert_eq!(read.get_untracked(), 10);

    Executor::tick().await;
    set_source.set(11);
    Executor::tick().await;
    assert_eq!(read.get_untracked(), 11);

    write.set(99);
    assert_eq!(read.get_untracked(), 99);
    assert_eq!(source.get_untracked(), 11);
}

#[wasm_bindgen_test]
fn str_converts_to_a_string_value() {
    let value: MaybeRwSignal<String> = "text".into();

    assert!(matches!(value, MaybeRwSignal::Static(ref s) if s == "text"));
}
