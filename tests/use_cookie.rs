#![cfg(target_arch = "wasm32")]

use codee::string::FromToStringCodec;
use gloo_timers::future::TimeoutFuture;
use leptos::prelude::*;
use leptos_use::{UseCookieOptions, use_cookie_with_options};
use wasm_bindgen_test::*;

wasm_bindgen_test_configure!(run_in_browser);

// Every write rebuilds the cookie with a fresh `Max-Age`, so the real cookie
// stays alive. The timer that clears the signal was armed once, when the hook
// was created, and was never restarted. The signal therefore reported the
// cookie as gone while `document.cookie` still held a perfectly valid value -
// and a reload brought the value back.
#[wasm_bindgen_test]
async fn writing_the_cookie_refreshes_the_expiration_countdown() {
    let _ = any_spawner::Executor::init_wasm_bindgen();

    let owner = Owner::new();
    owner.set();

    let (cookie, set_cookie) = use_cookie_with_options::<u32, FromToStringCodec>(
        "leptos_use_expiry_test",
        UseCookieOptions::default().max_age(600),
    );

    set_cookie.set(Some(1));
    TimeoutFuture::new(400).await;

    // Refreshes the cookie, and with it its Max-Age, to 600ms from now.
    set_cookie.set(Some(2));
    TimeoutFuture::new(400).await;

    // 800ms after the hook was created, past the original deadline, but only
    // 400ms after the last write.
    assert_eq!(
        cookie.get_untracked(),
        Some(2),
        "the signal must not expire while the cookie itself is still valid"
    );
}

// The countdown still has to fire when nothing refreshes it.
#[wasm_bindgen_test]
async fn the_cookie_still_expires_when_it_is_not_rewritten() {
    let _ = any_spawner::Executor::init_wasm_bindgen();

    let owner = Owner::new();
    owner.set();

    let (cookie, set_cookie) = use_cookie_with_options::<u32, FromToStringCodec>(
        "leptos_use_expiry_test_idle",
        UseCookieOptions::default().max_age(300),
    );

    set_cookie.set(Some(1));
    TimeoutFuture::new(800).await;

    assert_eq!(
        cookie.get_untracked(),
        None,
        "an untouched cookie has to expire"
    );
}
