#![cfg(target_arch = "wasm32")]

use leptos::prelude::*;
use leptos_use_is::{is_err, is_none, is_ok, is_some};
use wasm_bindgen_test::*;

wasm_bindgen_test_configure!(run_in_browser);

#[wasm_bindgen_test]
fn option_checks_follow_the_signal() {
    let owner = Owner::new();
    owner.set();

    let (value, set_value) = signal(None::<u32>);
    let some = is_some::<_, u32>(value);
    let none = is_none::<_, u32>(value);

    assert!(!some.get_untracked());
    assert!(none.get_untracked());

    set_value.set(Some(1));
    assert!(some.get_untracked());
    assert!(!none.get_untracked());

    set_value.set(None);
    assert!(!some.get_untracked());
    assert!(none.get_untracked());
}

#[wasm_bindgen_test]
fn result_checks_follow_the_signal() {
    let owner = Owner::new();
    owner.set();

    let (value, set_value) = signal(Ok::<_, ()>("fine"));
    let ok = is_ok::<_, &str>(value);
    let err = is_err::<_, &str>(value);

    assert!(ok.get_untracked());
    assert!(!err.get_untracked());

    set_value.set(Err(()));
    assert!(!ok.get_untracked());
    assert!(err.get_untracked());
}

#[wasm_bindgen_test]
async fn checks_are_tracked_by_effects() {
    let _ = any_spawner::Executor::init_wasm_bindgen();
    let owner = Owner::new();
    owner.set();

    let (value, set_value) = signal(None::<&'static str>);
    let some = is_some::<_, &str>(value);
    let (seen, set_seen) = signal(Vec::new());

    Effect::new(move |_| {
        let current = some.get();
        set_seen.update(|seen| seen.push(current));
    });

    any_spawner::Executor::tick().await;
    set_value.set(Some("now"));
    any_spawner::Executor::tick().await;

    assert_eq!(seen.get_untracked(), vec![false, true]);
}
