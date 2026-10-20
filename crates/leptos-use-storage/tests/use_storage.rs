#![cfg(target_arch = "wasm32")]

use any_spawner::Executor;
use codee::string::{FromToStringCodec, JsonSerdeCodec};
use gloo_timers::future::TimeoutFuture;
use leptos::prelude::*;
use leptos_use_storage::{
    StorageType, UseStorageOptions, use_local_storage, use_local_storage_with_options,
    use_session_storage, use_storage,
};
use serde::{Deserialize, Serialize};
use wasm_bindgen_test::*;

wasm_bindgen_test_configure!(run_in_browser);

fn local() -> web_sys::Storage {
    window().local_storage().unwrap().unwrap()
}

fn session() -> web_sys::Storage {
    window().session_storage().unwrap().unwrap()
}

async fn setup() -> Owner {
    let _ = Executor::init_wasm_bindgen();
    let owner = Owner::new();
    owner.set();
    owner
}

#[wasm_bindgen_test]
async fn writes_and_removes_the_stored_value() {
    let _owner = setup().await;
    local().remove_item("storage-test-count").unwrap();

    let (count, set_count, remove) =
        use_local_storage::<i32, FromToStringCodec>("storage-test-count");
    Executor::tick().await;
    assert_eq!(count.get_untracked(), 0, "defaults without a stored value");

    set_count.set(42);
    TimeoutFuture::new(20).await;
    assert_eq!(
        local().get_item("storage-test-count").unwrap().as_deref(),
        Some("42")
    );

    remove();
    TimeoutFuture::new(20).await;
    assert_eq!(local().get_item("storage-test-count").unwrap(), None);
    assert_eq!(count.get_untracked(), 0);
}

#[wasm_bindgen_test]
async fn reads_an_existing_value() {
    let _owner = setup().await;
    session().set_item("storage-test-flag", "true").unwrap();

    let (flag, _, _) = use_session_storage::<bool, FromToStringCodec>("storage-test-flag");
    Executor::tick().await;

    assert!(flag.get_untracked());
    session().remove_item("storage-test-flag").unwrap();
}

#[derive(Serialize, Deserialize, Clone, Debug, Default, PartialEq)]
struct State {
    name: String,
    visits: u32,
}

#[wasm_bindgen_test]
async fn stores_structs_with_a_codec() {
    let _owner = setup().await;
    session().remove_item("storage-test-state").unwrap();

    let (state, set_state, _) =
        use_storage::<State, JsonSerdeCodec>(StorageType::Session, "storage-test-state");
    Executor::tick().await;

    set_state.set(State {
        name: "leptos".to_string(),
        visits: 3,
    });
    TimeoutFuture::new(20).await;

    let stored = session().get_item("storage-test-state").unwrap().unwrap();
    assert_eq!(stored, r#"{"name":"leptos","visits":3}"#);
    assert_eq!(state.get_untracked().visits, 3);
    session().remove_item("storage-test-state").unwrap();
}

#[wasm_bindgen_test]
async fn uses_the_initial_value_without_a_stored_value() {
    let _owner = setup().await;
    local().remove_item("storage-test-initial").unwrap();

    let (value, _, _) = use_local_storage_with_options::<String, FromToStringCodec>(
        "storage-test-initial",
        UseStorageOptions::default().initial_value("initial".to_string()),
    );
    Executor::tick().await;

    assert_eq!(value.get_untracked(), "initial");
}

// Hooks bound to the same key stay in sync within the page.
#[wasm_bindgen_test]
async fn syncs_hooks_with_the_same_key() {
    let _owner = setup().await;
    local().remove_item("storage-test-shared").unwrap();

    let (a, set_a, _) = use_local_storage::<String, FromToStringCodec>("storage-test-shared");
    let (b, _, _) = use_local_storage::<String, FromToStringCodec>("storage-test-shared");
    Executor::tick().await;

    set_a.set("shared".to_string());
    TimeoutFuture::new(50).await;

    assert_eq!(a.get_untracked(), "shared");
    assert_eq!(b.get_untracked(), "shared");
    local().remove_item("storage-test-shared").unwrap();
}

// Other tabs notify changes with a `storage` event.
#[wasm_bindgen_test]
async fn follows_changes_from_other_tabs() {
    let _owner = setup().await;
    local().remove_item("storage-test-other-tab").unwrap();

    let (value, _, _) = use_local_storage::<String, FromToStringCodec>("storage-test-other-tab");
    Executor::tick().await;

    local()
        .set_item("storage-test-other-tab", "from elsewhere")
        .unwrap();
    let init = web_sys::StorageEventInit::new();
    init.set_key(Some("storage-test-other-tab"));
    init.set_storage_area(Some(&local()));
    window()
        .dispatch_event(&web_sys::StorageEvent::new_with_event_init_dict("storage", &init).unwrap())
        .unwrap();
    TimeoutFuture::new(20).await;

    assert_eq!(value.get_untracked(), "from elsewhere");
    local().remove_item("storage-test-other-tab").unwrap();
}
