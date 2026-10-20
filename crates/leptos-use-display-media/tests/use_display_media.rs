#![cfg(target_arch = "wasm32")]

use any_spawner::Executor;
use gloo_timers::future::TimeoutFuture;
use leptos::prelude::*;
use leptos::wasm_bindgen::JsCast;
use leptos_use_display_media::{
    UseDisplayMediaOptions, UseDisplayMediaReturn, use_display_media,
    use_display_media_with_options,
};
use wasm_bindgen_test::*;

wasm_bindgen_test_configure!(run_in_browser);

/// Screen capture needs a user gesture, so `getDisplayMedia` is replaced by a
/// stub that returns the stream of a canvas.
fn stub_display_media() {
    js_sys::eval(
        r#"
        window.__displayCalls = [];
        Object.defineProperty(navigator, "mediaDevices", {
            configurable: true,
            value: {
                getDisplayMedia: async (constraints) => {
                    window.__displayCalls.push(constraints);
                    return document.createElement("canvas").captureStream();
                },
            },
        });
        "#,
    )
    .unwrap();
}

fn tracks(stream: &web_sys::MediaStream) -> Vec<web_sys::MediaStreamTrack> {
    stream
        .get_tracks()
        .iter()
        .map(|t| t.unchecked_into())
        .collect()
}

#[wasm_bindgen_test]
async fn starts_and_stops_the_stream() {
    let _ = Executor::init_wasm_bindgen();
    let owner = Owner::new();
    owner.set();
    stub_display_media();

    let UseDisplayMediaReturn {
        stream,
        start,
        stop,
        enabled,
        ..
    } = use_display_media();
    Executor::tick().await;
    assert!(!enabled.get_untracked());
    assert!(stream.read_untracked().is_none());

    start();
    TimeoutFuture::new(50).await;
    assert!(enabled.get_untracked());
    let media = stream.read_untracked().clone().unwrap().unwrap();
    let tracks = tracks(&media);
    assert!(!tracks.is_empty());
    assert!(
        tracks
            .iter()
            .all(|t| t.ready_state() == web_sys::MediaStreamTrackState::Live)
    );

    stop();
    assert!(!enabled.get_untracked());
    assert!(stream.read_untracked().is_none());
    assert!(
        tracks
            .iter()
            .all(|t| t.ready_state() == web_sys::MediaStreamTrackState::Ended)
    );
}

#[wasm_bindgen_test]
async fn enabled_signal_controls_the_stream_and_audio_is_requested() {
    let _ = Executor::init_wasm_bindgen();
    let owner = Owner::new();
    owner.set();
    stub_display_media();

    let (enabled, set_enabled) = signal(false);
    let UseDisplayMediaReturn { stream, .. } = use_display_media_with_options(
        UseDisplayMediaOptions::default()
            .enabled((enabled, set_enabled).into())
            .audio(true),
    );
    Executor::tick().await;

    set_enabled.set(true);
    TimeoutFuture::new(50).await;
    assert!(matches!(*stream.read_untracked(), Some(Ok(_))));
    let audio = js_sys::eval("window.__displayCalls[0].audio")
        .unwrap()
        .as_bool();
    assert_eq!(audio, Some(true));

    set_enabled.set(false);
    Executor::tick().await;
    assert!(stream.read_untracked().is_none());
}

#[wasm_bindgen_test]
async fn reports_errors() {
    let _ = Executor::init_wasm_bindgen();
    let owner = Owner::new();
    owner.set();
    js_sys::eval(
        r#"
        Object.defineProperty(navigator, "mediaDevices", {
            configurable: true,
            value: { getDisplayMedia: async () => { throw new Error("denied"); } },
        });
        "#,
    )
    .unwrap();

    let UseDisplayMediaReturn {
        stream,
        start,
        enabled,
        ..
    } = use_display_media();
    start();
    TimeoutFuture::new(50).await;

    assert!(matches!(*stream.read_untracked(), Some(Err(_))));
    assert!(!enabled.get_untracked());
}
