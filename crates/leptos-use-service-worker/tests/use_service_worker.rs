#![cfg(target_arch = "wasm32")]

use any_spawner::Executor;
use gloo_timers::future::TimeoutFuture;
use leptos::prelude::*;
use leptos_use_service_worker::{
    ServiceWorkerRegistrationError, UseServiceWorkerOptions, UseServiceWorkerReturn,
    use_service_worker_with_options,
};
use wasm_bindgen_test::*;

wasm_bindgen_test_configure!(run_in_browser);

/// Waits up to two seconds for `condition` to become true.
async fn eventually(condition: impl Fn() -> bool) -> bool {
    for _ in 0..40 {
        if condition() {
            return true;
        }
        TimeoutFuture::new(50).await;
    }
    condition()
}

// The test runner serves files from the crate directory, so
// `test-service-worker.js` is a real same-origin service worker script.
#[wasm_bindgen_test]
async fn registers_and_activates_the_service_worker() {
    let _ = Executor::init_wasm_bindgen();
    let owner = Owner::new();
    owner.set();

    let UseServiceWorkerReturn {
        registration,
        active,
        waiting,
        check_for_update,
        ..
    } = use_service_worker_with_options(
        UseServiceWorkerOptions::default().script_url("/test-service-worker.js"),
    );
    assert!(matches!(
        registration.get_untracked(),
        Err(ServiceWorkerRegistrationError::NeverQueried)
    ));

    assert!(
        eventually(|| registration.with_untracked(|r| r.is_ok())).await,
        "registration succeeds"
    );
    let scope = registration.with_untracked(|r| r.as_ref().unwrap().scope());
    assert!(scope.ends_with('/'), "scope {scope}");

    assert!(
        eventually(|| active.get_untracked()).await,
        "the worker activates"
    );
    assert!(!waiting.get_untracked());

    // nothing changed, so checking for an update keeps the registration
    check_for_update();
    TimeoutFuture::new(100).await;
    assert!(registration.with_untracked(|r| r.is_ok()));
}

#[wasm_bindgen_test]
async fn reports_failed_registrations() {
    let _ = Executor::init_wasm_bindgen();
    let owner = Owner::new();
    owner.set();

    let UseServiceWorkerReturn {
        registration,
        active,
        ..
    } = use_service_worker_with_options(
        UseServiceWorkerOptions::default().script_url("/does-not-exist.js"),
    );

    assert!(
        eventually(|| matches!(
            registration.get_untracked(),
            Err(ServiceWorkerRegistrationError::Js(_))
        ))
        .await
    );
    assert!(!active.get_untracked());
}
