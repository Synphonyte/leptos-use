#![cfg(target_arch = "wasm32")]

use codee::string::FromToStringCodec;
use gloo_timers::future::TimeoutFuture;
use leptos::prelude::*;
use leptos_use::core::ConnectionReadyState;
use leptos_use::{
    ReconnectLimit, UseWebSocketOptions, UseWebSocketReturn, use_websocket_with_options,
};
use std::sync::Arc;
use std::sync::atomic::{AtomicU32, Ordering};
use wasm_bindgen_test::*;

wasm_bindgen_test_configure!(run_in_browser);

/// Nothing listens on this port, so every connection attempt fails right away
/// and drives the reconnect logic without needing a server.
const UNREACHABLE: &str = "ws://127.0.0.1:49517/";

// `close()` records that the socket was closed on purpose, and that flag gates
// every future automatic reconnect. It was never cleared again, so after a
// deliberate close the hook still reconnected on demand via `open()` but then
// refused to recover from any later unintentional drop for the rest of its
// life.
#[wasm_bindgen_test]
async fn reopening_restores_automatic_reconnects() {
    let _ = any_spawner::Executor::init_wasm_bindgen();

    let owner = Owner::new();
    owner.set();

    let closes = Arc::new(AtomicU32::new(0));

    let UseWebSocketReturn {
        open,
        close,
        ready_state,
        ..
    } = {
        let closes = Arc::clone(&closes);

        use_websocket_with_options::<String, String, FromToStringCodec, _, _>(
            UNREACHABLE,
            UseWebSocketOptions::default()
                .immediate(false)
                .reconnect_interval(200)
                .reconnect_limit(ReconnectLimit::Limited(10))
                .on_close(move |_| {
                    closes.fetch_add(1, Ordering::SeqCst);
                }),
        )
    };

    // Open, let the attempt fail, then close deliberately.
    open();
    for _ in 0..40 {
        TimeoutFuture::new(25).await;
        if closes.load(Ordering::SeqCst) > 0 {
            break;
        }
    }
    close();
    TimeoutFuture::new(400).await;

    let before_reopen = closes.load(Ordering::SeqCst);

    // Reopen. This attempt fails too, and because the user did not close it
    // this time the hook has to retry on its own.
    open();
    TimeoutFuture::new(1200).await;

    assert!(
        closes.load(Ordering::SeqCst) >= before_reopen + 2,
        "after reopening, a dropped connection must be retried automatically; \
         saw {} close events, {before_reopen} of them before reopening",
        closes.load(Ordering::SeqCst)
    );
    assert_ne!(ready_state.get_untracked(), ConnectionReadyState::Open);
}
