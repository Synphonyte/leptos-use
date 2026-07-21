#![cfg(target_arch = "wasm32")]

use gloo_timers::future::TimeoutFuture;
use leptos::prelude::*;
use leptos::web_sys::MediaStream;
use leptos_use::{UseUserMediaOptions, UseUserMediaReturn, use_user_media_with_options};
use wasm_bindgen_test::*;

wasm_bindgen_test_configure!(run_in_browser);

// The hook stops the stream's tracks when `enabled` turns false or when the
// returned `stop()` is called, but it never registered a cleanup handler.
// Disposing the owner (unmounting the component) therefore left the camera and
// microphone running for the lifetime of the page, with no handle left to stop
// them.
#[wasm_bindgen_test]
async fn tracks_are_stopped_when_the_owner_is_cleaned_up() {
    let _ = any_spawner::Executor::init_wasm_bindgen();

    let owner = Owner::new();
    owner.set();

    let UseUserMediaReturn { stream, .. } =
        use_user_media_with_options(UseUserMediaOptions::default().enabled(true.into()));

    // Give `getUserMedia` a chance to resolve.
    let mut media_stream: Option<MediaStream> = None;
    for _ in 0..50 {
        TimeoutFuture::new(100).await;

        match stream.get_untracked() {
            Some(Ok(s)) => {
                media_stream = Some(s);
                break;
            }
            Some(Err(err)) => panic!("getUserMedia failed: {err:?}"),
            None => {}
        }
    }

    let media_stream = media_stream.expect("the fake media stream should have started");

    assert!(
        media_stream.active(),
        "the stream should be running before the owner is cleaned up"
    );

    owner.cleanup();

    assert!(
        !media_stream.active(),
        "cleaning up the owner must stop the media tracks"
    );
}
