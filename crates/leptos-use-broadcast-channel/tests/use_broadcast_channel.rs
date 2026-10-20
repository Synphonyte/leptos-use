#![cfg(target_arch = "wasm32")]

use any_spawner::Executor;
use codee::string::{FromToStringCodec, JsonSerdeCodec};
use gloo_timers::future::TimeoutFuture;
use leptos::prelude::*;
use leptos_use_broadcast_channel::{UseBroadcastChannelReturn, use_broadcast_channel};
use serde::{Deserialize, Serialize};
use wasm_bindgen_test::*;

wasm_bindgen_test_configure!(run_in_browser);

// A channel doesn't receive its own messages, but every other channel with
// the same name does, even within the same page.
#[wasm_bindgen_test]
async fn delivers_messages_to_other_channels() {
    let _ = Executor::init_wasm_bindgen();
    let owner = Owner::new();
    owner.set();

    let sender = use_broadcast_channel::<String, FromToStringCodec>("bc-test-text");
    let receiver = use_broadcast_channel::<String, FromToStringCodec>("bc-test-text");
    Executor::tick().await;
    assert!(sender.is_supported.get_untracked());

    (sender.post)(&"hello".to_string());
    TimeoutFuture::new(50).await;

    assert_eq!(receiver.message.get_untracked().as_deref(), Some("hello"));
    assert_eq!(
        sender.message.get_untracked(),
        None,
        "no echo to the sender"
    );
}

#[wasm_bindgen_test]
async fn receives_messages_from_plain_broadcast_channels() {
    let _ = Executor::init_wasm_bindgen();
    let owner = Owner::new();
    owner.set();

    let UseBroadcastChannelReturn { message, .. } =
        use_broadcast_channel::<bool, FromToStringCodec>("bc-test-plain");
    Executor::tick().await;

    let raw = web_sys::BroadcastChannel::new("bc-test-plain").unwrap();
    raw.post_message(&"true".into()).unwrap();
    TimeoutFuture::new(50).await;
    raw.close();

    assert_eq!(message.get_untracked(), Some(true));
}

#[derive(Serialize, Deserialize, Clone, Debug, PartialEq)]
struct Update {
    id: u32,
    text: String,
}

#[wasm_bindgen_test]
async fn sends_structs_with_a_codec_and_closes() {
    let _ = Executor::init_wasm_bindgen();
    let owner = Owner::new();
    owner.set();

    let sender = use_broadcast_channel::<Update, JsonSerdeCodec>("bc-test-json");
    let receiver = use_broadcast_channel::<Update, JsonSerdeCodec>("bc-test-json");
    Executor::tick().await;

    let update = Update {
        id: 1,
        text: "first".to_string(),
    };
    (sender.post)(&update);
    TimeoutFuture::new(50).await;
    assert_eq!(receiver.message.get_untracked(), Some(update));

    (receiver.close)();
    assert!(receiver.is_closed.get_untracked());

    (sender.post)(&Update {
        id: 2,
        text: "second".to_string(),
    });
    TimeoutFuture::new(50).await;
    assert_eq!(
        receiver.message.get_untracked().map(|u| u.id),
        Some(1),
        "a closed channel receives nothing"
    );
}
