#![cfg(target_arch = "wasm32")]

use any_spawner::Executor;
use leptos::prelude::*;
use leptos_use_sync_signal::{
    SyncDirection, SyncSignalOptions, sync_signal, sync_signal_with_options,
};
use std::str::FromStr;
use wasm_bindgen_test::*;

wasm_bindgen_test_configure!(run_in_browser);

async fn setup() -> Owner {
    let _ = Executor::init_wasm_bindgen();
    let owner = Owner::new();
    owner.set();
    owner
}

#[wasm_bindgen_test]
async fn syncs_both_ways() {
    let _owner = setup().await;

    let (a, set_a) = signal(1);
    let (b, set_b) = signal(2);

    let _stop = sync_signal((a, set_a), (b, set_b));
    Executor::tick().await;
    assert_eq!(b.get_untracked(), 1, "immediately synced left to right");

    set_b.set(3);
    Executor::tick().await;
    assert_eq!(a.get_untracked(), 3);

    set_a.set(4);
    Executor::tick().await;
    assert_eq!(b.get_untracked(), 4);
}

#[wasm_bindgen_test]
async fn syncs_rw_signals() {
    let _owner = setup().await;

    let a = RwSignal::new(1);
    let b = RwSignal::new(2);

    let _stop = sync_signal(a, b);
    Executor::tick().await;

    b.set(5);
    Executor::tick().await;
    assert_eq!(a.get_untracked(), 5);
}

#[wasm_bindgen_test]
async fn syncs_in_one_direction_only() {
    let _owner = setup().await;

    let (a, set_a) = signal(1);
    let (b, set_b) = signal(2);

    let _stop = sync_signal_with_options(
        (a, set_a),
        (b, set_b),
        SyncSignalOptions::default().direction(SyncDirection::LeftToRight),
    );
    Executor::tick().await;
    assert_eq!(b.get_untracked(), 1);

    set_b.set(3);
    Executor::tick().await;
    assert_eq!(a.get_untracked(), 1, "right to left is not synced");

    set_a.set(4);
    Executor::tick().await;
    assert_eq!(b.get_untracked(), 4);
}

#[wasm_bindgen_test]
async fn does_not_sync_initially_when_not_immediate() {
    let _owner = setup().await;

    let (a, set_a) = signal(1);
    let (b, set_b) = signal(2);

    let _stop = sync_signal_with_options(
        (a, set_a),
        (b, set_b),
        SyncSignalOptions::default().immediate(false),
    );
    Executor::tick().await;
    assert_eq!(b.get_untracked(), 2);

    set_a.set(7);
    Executor::tick().await;
    assert_eq!(b.get_untracked(), 7);
}

#[wasm_bindgen_test]
async fn transforms_between_types() {
    let _owner = setup().await;

    let (a, set_a) = signal("10".to_string());
    let (b, set_b) = signal(2);

    let _stop = sync_signal_with_options(
        (a, set_a),
        (b, set_b),
        SyncSignalOptions::with_transforms(
            |left: &String| i32::from_str(left).unwrap_or_default(),
            |right: &i32| right.to_string(),
        ),
    );
    Executor::tick().await;
    assert_eq!(b.get_untracked(), 10);

    set_b.set(42);
    Executor::tick().await;
    assert_eq!(a.get_untracked(), "42");
}

#[wasm_bindgen_test]
async fn assigns_into_a_field() {
    let _owner = setup().await;

    #[derive(Clone, Debug, PartialEq)]
    struct Foo {
        bar: i32,
        other: &'static str,
    }

    let (a, set_a) = signal(Foo {
        bar: 10,
        other: "kept",
    });
    let (b, set_b) = signal(2);

    let _stop = sync_signal_with_options(
        (a, set_a),
        (b, set_b),
        SyncSignalOptions::with_assigns(
            |b: &mut i32, a: &Foo| *b = a.bar,
            |a: &mut Foo, b: &i32| a.bar = *b,
        ),
    );
    Executor::tick().await;
    assert_eq!(b.get_untracked(), 10);

    set_b.set(3);
    Executor::tick().await;
    assert_eq!(
        a.get_untracked(),
        Foo {
            bar: 3,
            other: "kept"
        }
    );
}

#[wasm_bindgen_test]
async fn stop_ends_syncing() {
    let _owner = setup().await;

    let (a, set_a) = signal(1);
    let (b, set_b) = signal(1);

    let stop = sync_signal((a, set_a), (b, set_b));
    Executor::tick().await;

    stop();
    set_a.set(2);
    Executor::tick().await;
    assert_eq!(b.get_untracked(), 1);
}
