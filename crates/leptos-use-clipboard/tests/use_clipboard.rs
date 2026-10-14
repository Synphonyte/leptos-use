#![cfg(target_arch = "wasm32")]

use any_spawner::Executor;
use gloo_timers::future::TimeoutFuture;
use leptos::prelude::*;
use leptos_use_clipboard::{UseClipboardOptions, UseClipboardReturn, use_clipboard_with_options};
use wasm_bindgen_test::*;

wasm_bindgen_test_configure!(run_in_browser);

/// Headless browsers deny clipboard access, so it is replaced by an in-memory stub.
fn stub_clipboard() {
    js_sys::eval(
        r#"
        window.__clipboard = "";
        Object.defineProperty(navigator, "clipboard", {
            configurable: true,
            value: {
                writeText: async (text) => { window.__clipboard = text; },
                readText: async () => window.__clipboard,
            },
        });
        "#,
    )
    .unwrap();
}

fn clipboard_content() -> String {
    js_sys::eval("window.__clipboard")
        .unwrap()
        .as_string()
        .unwrap()
}

#[wasm_bindgen_test]
async fn copies_text_and_resets_copied() {
    let _ = Executor::init_wasm_bindgen();
    let owner = Owner::new();
    owner.set();
    stub_clipboard();

    let UseClipboardReturn {
        is_supported,
        text,
        copied,
        copy,
    } = use_clipboard_with_options(UseClipboardOptions::default().copied_reset_delay(100.0));
    Executor::tick().await;
    assert!(is_supported.get_untracked());

    copy("Hello!");
    TimeoutFuture::new(20).await;

    assert_eq!(clipboard_content(), "Hello!");
    assert_eq!(text.get_untracked().as_deref(), Some("Hello!"));
    assert!(copied.get_untracked());

    TimeoutFuture::new(150).await;
    assert!(!copied.get_untracked(), "copied resets after the delay");
}

#[wasm_bindgen_test]
async fn reads_the_clipboard_on_copy_and_cut() {
    let _ = Executor::init_wasm_bindgen();
    let owner = Owner::new();
    owner.set();
    stub_clipboard();

    let UseClipboardReturn { text, .. } =
        use_clipboard_with_options(UseClipboardOptions::default().read(true));
    Executor::tick().await;

    js_sys::eval(r#"window.__clipboard = "copied elsewhere""#).unwrap();
    window()
        .dispatch_event(&web_sys::Event::new("copy").unwrap())
        .unwrap();
    TimeoutFuture::new(20).await;

    assert_eq!(text.get_untracked().as_deref(), Some("copied elsewhere"));
}
