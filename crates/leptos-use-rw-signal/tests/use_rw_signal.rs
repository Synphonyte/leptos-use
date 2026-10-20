#![cfg(target_arch = "wasm32")]

use any_spawner::Executor;
use leptos::prelude::*;
use leptos_use_rw_signal::UseRwSignal;
use wasm_bindgen_test::*;

wasm_bindgen_test_configure!(run_in_browser);

#[wasm_bindgen_test]
fn reads_and_writes_through_an_rw_signal() {
    let owner = Owner::new();
    owner.set();

    let rw = RwSignal::new(1);
    let signal: UseRwSignal<_, _, i32> = rw.into();

    assert_eq!(*signal.read_untracked(), 1);

    signal.update(|v| *v = 2);
    assert_eq!(rw.get_untracked(), 2);
    assert_eq!(*signal.read_untracked(), 2);
}

#[wasm_bindgen_test]
fn reads_and_writes_through_a_signal_pair() {
    let owner = Owner::new();
    owner.set();

    let (read, write) = signal("a".to_string());
    let signal: UseRwSignal<_, _, String> = (read, write).into();

    signal.update(|s| s.push('b'));
    assert_eq!(read.get_untracked(), "ab");
    assert_eq!(*signal.read_untracked(), "ab");
}

#[wasm_bindgen_test]
fn default_holds_the_default_value() {
    let owner = Owner::new();
    owner.set();

    let signal: UseRwSignal<RwSignal<u8>, RwSignal<u8>, u8> = Default::default();

    assert_eq!(*signal.read_untracked(), 0);
}

#[wasm_bindgen_test]
async fn tracks_the_read_half() {
    let _ = Executor::init_wasm_bindgen();
    let owner = Owner::new();
    owner.set();

    let (read, write) = signal(0);
    let signal: UseRwSignal<_, _, i32> = (read, write).into();
    let (seen, set_seen) = leptos::prelude::signal(Vec::new());

    Effect::new(move |_| {
        let value = *signal.read();
        set_seen.update(|seen| seen.push(value));
    });

    Executor::tick().await;
    signal.update(|v| *v = 5);
    Executor::tick().await;

    assert_eq!(seen.get_untracked(), vec![0, 5]);
}
