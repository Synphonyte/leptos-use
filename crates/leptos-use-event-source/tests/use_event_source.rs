#![cfg(target_arch = "wasm32")]

use any_spawner::Executor;
use codee::string::{FromToStringCodec, JsonSerdeCodec};
use gloo_timers::future::TimeoutFuture;
use leptos::prelude::*;
use leptos_use_core::ConnectionReadyState;
use leptos_use_event_source::{
    UseEventSourceOptions, UseEventSourceReturn, use_event_source, use_event_source_with_options,
};
use serde::{Deserialize, Serialize};
use wasm_bindgen_test::*;

wasm_bindgen_test_configure!(run_in_browser);

/// Serves a fixed event stream from a blob URL. The long `retry` keeps the
/// browser from replaying it while the test runs.
fn stream_url(body: &str) -> String {
    let parts = js_sys::Array::of1(&format!("retry: 100000\n\n{body}").into());
    let options = web_sys::BlobPropertyBag::new();
    options.set_type("text/event-stream");
    let blob = web_sys::Blob::new_with_str_sequence_and_options(&parts, &options).unwrap();
    web_sys::Url::create_object_url_with_blob(&blob).unwrap()
}

#[wasm_bindgen_test]
async fn receives_messages() {
    let _ = Executor::init_wasm_bindgen();
    let owner = Owner::new();
    owner.set();

    let url = stream_url("data: first\n\nid: 7\ndata: second\n\n");
    let UseEventSourceReturn {
        message,
        ready_state,
        close,
        ..
    } = use_event_source::<String, FromToStringCodec>(url);
    TimeoutFuture::new(200).await;

    let message = message.get_untracked().expect("a message");
    assert_eq!(message.data, "second");
    assert_eq!(message.event_type, "message");
    assert_eq!(message.last_event_id, "7");

    close();
    assert_eq!(ready_state.get_untracked(), ConnectionReadyState::Closed);
}

#[derive(Serialize, Deserialize, Clone, Debug, PartialEq)]
struct Notice {
    level: u8,
}

#[wasm_bindgen_test]
async fn decodes_named_events() {
    let _ = Executor::init_wasm_bindgen();
    let owner = Owner::new();
    owner.set();

    let url = stream_url("event: notice\ndata: {\"level\":3}\n\n");
    let UseEventSourceReturn { message, close, .. } =
        use_event_source_with_options::<Notice, JsonSerdeCodec>(
            url,
            UseEventSourceOptions::default().named_events(["notice".to_string()]),
        );
    TimeoutFuture::new(200).await;

    let message = message.get_untracked().expect("a named event");
    assert_eq!(message.event_type, "notice");
    assert_eq!(message.data, Notice { level: 3 });
    close();
}

#[wasm_bindgen_test]
async fn connects_only_when_opened() {
    let _ = Executor::init_wasm_bindgen();
    let owner = Owner::new();
    owner.set();

    let url = stream_url("data: hello\n\n");
    let UseEventSourceReturn {
        message,
        ready_state,
        open,
        close,
        ..
    } = use_event_source_with_options::<String, FromToStringCodec>(
        url,
        UseEventSourceOptions::default().immediate(false),
    );
    TimeoutFuture::new(100).await;
    assert_eq!(ready_state.get_untracked(), ConnectionReadyState::Closed);
    assert!(message.get_untracked().is_none());

    open();
    TimeoutFuture::new(200).await;
    assert_eq!(
        message.get_untracked().map(|m| m.data).as_deref(),
        Some("hello")
    );
    close();
}
