#![cfg(target_arch = "wasm32")]

use any_spawner::Executor;
use gloo_timers::future::TimeoutFuture;
use leptos::prelude::*;
use leptos_use_web_notification::{
    NotificationPermission, ShowOptions, UseWebNotificationOptions, UseWebNotificationReturn,
    use_web_notification_with_options,
};
use std::cell::Cell;
use std::rc::Rc;
use wasm_bindgen_test::*;

wasm_bindgen_test_configure!(run_in_browser);

/// Headless browsers deny notifications, so `Notification` is replaced by a
/// stub that records what was shown and grants the permission.
fn stub_notification() {
    js_sys::eval(
        r#"
        window.__notifications = [];
        window.Notification = class {
            constructor(title, options) {
                this.title = title;
                this.body = options.body;
                this.tag = options.tag;
                this.closed = false;
                window.__notifications.push(this);
            }
            close() { this.closed = true; }
            static requestPermission() { return Promise.resolve("granted"); }
            static get permission() { return "granted"; }
        };
        "#,
    )
    .unwrap();
}

fn eval_string(js: &str) -> Option<String> {
    js_sys::eval(js).unwrap().as_string()
}

#[wasm_bindgen_test]
async fn shows_and_closes_notifications() {
    let _ = Executor::init_wasm_bindgen();
    let owner = Owner::new();
    owner.set();
    stub_notification();

    let clicks = Rc::new(Cell::new(0));
    let UseWebNotificationReturn {
        is_supported,
        notification,
        show,
        close,
        permission,
    } = use_web_notification_with_options(
        UseWebNotificationOptions::default()
            .title("Default title")
            .body("Default body")
            .tag("test")
            .on_click({
                let clicks = Rc::clone(&clicks);
                move |_| clicks.set(clicks.get() + 1)
            }),
    );
    TimeoutFuture::new(20).await;
    assert!(is_supported.get_untracked());
    assert_eq!(permission.get_untracked(), NotificationPermission::Granted);

    show(ShowOptions::default().title("Hello World"));
    TimeoutFuture::new(20).await;

    assert!(notification.read_untracked().is_some());
    assert_eq!(
        eval_string("window.__notifications[0].title").as_deref(),
        Some("Hello World")
    );
    assert_eq!(
        eval_string("window.__notifications[0].body").as_deref(),
        Some("Default body")
    );
    assert_eq!(
        eval_string("window.__notifications[0].tag").as_deref(),
        Some("test")
    );

    js_sys::eval("window.__notifications[0].onclick(new Event('click'))").unwrap();
    assert_eq!(clicks.get(), 1);

    close();
    assert!(notification.read_untracked().is_none());
    assert_eq!(
        js_sys::eval("window.__notifications[0].closed")
            .unwrap()
            .as_bool(),
        Some(true)
    );
}

#[wasm_bindgen_test]
async fn closes_the_notification_when_the_page_becomes_visible() {
    let _ = Executor::init_wasm_bindgen();
    let owner = Owner::new();
    owner.set();
    stub_notification();

    let UseWebNotificationReturn {
        show, notification, ..
    } = use_web_notification_with_options(UseWebNotificationOptions::default().title("Hi"));
    TimeoutFuture::new(20).await;

    show(ShowOptions::default());
    TimeoutFuture::new(20).await;
    assert!(notification.read_untracked().is_some());

    document()
        .dispatch_event(&web_sys::Event::new("visibilitychange").unwrap())
        .unwrap();

    assert!(notification.read_untracked().is_none());
}
