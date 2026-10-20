#![cfg(all(target_arch = "wasm32", web_sys_unstable_apis))]

use gloo_timers::future::TimeoutFuture;
use leptos_use_web_lock::{LockMode, UseWebLockOptions, use_web_lock, use_web_lock_with_options};
use std::cell::RefCell;
use std::rc::Rc;
use wasm_bindgen_test::*;

wasm_bindgen_test_configure!(run_in_browser);

#[wasm_bindgen_test]
async fn runs_the_callback_while_holding_the_lock() {
    let result = use_web_lock("web-lock-test-basic", |lock: web_sys::Lock| async move {
        (lock.name(), lock.mode())
    })
    .await;

    let (name, mode) = result.unwrap();
    assert_eq!(name, "web-lock-test-basic");
    assert_eq!(mode, LockMode::Exclusive);
}

#[wasm_bindgen_test]
async fn exclusive_locks_run_one_after_another() {
    let log = Rc::new(RefCell::new(Vec::new()));

    let first = {
        let log = Rc::clone(&log);
        use_web_lock("web-lock-test-exclusive", move |_| async move {
            log.borrow_mut().push("first start");
            TimeoutFuture::new(50).await;
            log.borrow_mut().push("first end");
        })
    };
    let second = {
        let log = Rc::clone(&log);
        use_web_lock("web-lock-test-exclusive", move |_| async move {
            log.borrow_mut().push("second");
        })
    };

    let (a, b) = futures_util::join!(first, second);
    a.unwrap();
    b.unwrap();

    assert_eq!(*log.borrow(), vec!["first start", "first end", "second"]);
}

#[wasm_bindgen_test]
async fn shared_locks_run_concurrently() {
    let log = Rc::new(RefCell::new(Vec::new()));
    let shared = || UseWebLockOptions::default().mode(LockMode::Shared);

    let first = {
        let log = Rc::clone(&log);
        use_web_lock_with_options(
            "web-lock-test-shared",
            move |_| async move {
                log.borrow_mut().push("first start");
                TimeoutFuture::new(50).await;
                log.borrow_mut().push("first end");
            },
            shared(),
        )
    };
    let second = {
        let log = Rc::clone(&log);
        use_web_lock_with_options(
            "web-lock-test-shared",
            move |_| async move {
                log.borrow_mut().push("second");
            },
            shared(),
        )
    };

    let (a, b) = futures_util::join!(first, second);
    a.unwrap();
    b.unwrap();

    assert_eq!(*log.borrow(), vec!["first start", "second", "first end"]);
}

// With `if_available` the callback gets `null` instead of waiting for the lock.
#[wasm_bindgen_test]
async fn if_available_does_not_wait_for_a_held_lock() {
    let holder = use_web_lock("web-lock-test-available", |_| async {
        TimeoutFuture::new(100).await;
    });
    let attempt = async {
        TimeoutFuture::new(10).await;
        use_web_lock_with_options(
            "web-lock-test-available",
            |lock: web_sys::Lock| async move { lock.is_null() },
            UseWebLockOptions::default().if_available(true),
        )
        .await
    };

    let (held, got_null) = futures_util::join!(holder, attempt);
    held.unwrap();
    assert!(got_null.unwrap(), "the lock was not available");
}
