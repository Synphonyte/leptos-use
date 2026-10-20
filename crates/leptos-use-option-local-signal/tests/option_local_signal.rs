#![cfg(target_arch = "wasm32")]

use any_spawner::Executor;
use leptos::prelude::*;
use leptos_use_option_local_signal::OptionLocalRwSignal;
use std::rc::Rc;
use wasm_bindgen_test::*;

wasm_bindgen_test_configure!(run_in_browser);

#[wasm_bindgen_test]
fn starts_empty_and_holds_non_send_values() {
    let owner = Owner::new();
    owner.set();

    let signal = OptionLocalRwSignal::<Rc<String>>::new();
    assert!(signal.read_untracked().is_none());

    signal.set(Some(Rc::new("local".to_string())));
    assert_eq!(
        signal.read_untracked().as_deref().map(|s| s.as_str()),
        Some("local")
    );
}

// The read-only half keeps sharing the value of the read-write signal.
#[wasm_bindgen_test]
fn read_only_shares_the_value() {
    let owner = Owner::new();
    owner.set();

    let rw = OptionLocalRwSignal::<u32>::new();
    let read_only = rw.read_only();

    rw.set(Some(7));
    assert_eq!(*read_only.read_untracked(), Some(7));

    rw.set(None);
    assert_eq!(*read_only.read_untracked(), None);
}

#[wasm_bindgen_test]
async fn notifies_effects_on_write() {
    let _ = Executor::init_wasm_bindgen();
    let owner = Owner::new();
    owner.set();

    let rw = OptionLocalRwSignal::<u32>::new();
    let read_only = rw.read_only();
    let (seen, set_seen) = signal(Vec::new());

    Effect::new(move |_| {
        let value = *read_only.read();
        set_seen.update(|seen| seen.push(value));
    });

    Executor::tick().await;
    rw.set(Some(1));
    Executor::tick().await;
    rw.update(|v| *v = v.map(|v| v + 1));
    Executor::tick().await;

    assert_eq!(seen.get_untracked(), vec![None, Some(1), Some(2)]);
}
