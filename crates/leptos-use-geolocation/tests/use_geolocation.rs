#![cfg(target_arch = "wasm32")]

use any_spawner::Executor;
use leptos::prelude::*;
use leptos_use_geolocation::{
    UseGeolocationOptions, UseGeolocationReturn, use_geolocation, use_geolocation_with_options,
};
use wasm_bindgen_test::*;

wasm_bindgen_test_configure!(run_in_browser);

/// Replaces `navigator.geolocation` with a stub whose watchers are driven by the test.
fn stub_geolocation() {
    js_sys::eval(
        r#"
        window.__geo = { watchers: new Map(), next: 1 };
        Object.defineProperty(navigator, "geolocation", {
            configurable: true,
            value: {
                watchPosition(success, error, options) {
                    const id = window.__geo.next++;
                    window.__geo.watchers.set(id, { success, error, options });
                    return id;
                },
                clearWatch(id) { window.__geo.watchers.delete(id); },
                getCurrentPosition() {},
            },
        });
        window.__geoEmit = (latitude, longitude) => {
            for (const w of window.__geo.watchers.values()) {
                w.success({
                    timestamp: 1234,
                    coords: { latitude, longitude, accuracy: 5, altitude: null, altitudeAccuracy: null, heading: null, speed: null },
                });
            }
        };
        window.__geoFail = (code, message) => {
            for (const w of window.__geo.watchers.values()) w.error({ code, message });
        };
        "#,
    )
    .unwrap();
}

fn watchers() -> u32 {
    js_sys::eval("window.__geo.watchers.size")
        .unwrap()
        .as_f64()
        .unwrap() as u32
}

#[wasm_bindgen_test]
async fn reports_positions_and_errors() {
    let _ = Executor::init_wasm_bindgen();
    let owner = Owner::new();
    owner.set();
    stub_geolocation();

    let UseGeolocationReturn {
        coords,
        located_at,
        error,
        ..
    } = use_geolocation();
    assert_eq!(watchers(), 1, "watching immediately");

    js_sys::eval("window.__geoEmit(48.2, 16.37)").unwrap();
    assert_eq!(
        coords
            .read_untracked()
            .as_ref()
            .map(|c| (c.latitude(), c.longitude())),
        Some((48.2, 16.37))
    );
    assert_eq!(located_at.get_untracked(), Some(1234.0));

    js_sys::eval("window.__geoFail(1, 'denied')").unwrap();
    assert_eq!(error.read_untracked().as_ref().map(|e| e.code()), Some(1));

    js_sys::eval("window.__geoEmit(1, 2)").unwrap();
    assert!(
        error.read_untracked().is_none(),
        "a new position clears the error"
    );
}

#[wasm_bindgen_test]
async fn pause_and_resume_watching() {
    let _ = Executor::init_wasm_bindgen();
    let owner = Owner::new();
    owner.set();
    stub_geolocation();

    let UseGeolocationReturn {
        coords,
        pause,
        resume,
        ..
    } = use_geolocation_with_options(
        UseGeolocationOptions::default()
            .immediate(false)
            .enable_high_accuracy(true),
    );
    assert_eq!(watchers(), 0);

    resume();
    assert_eq!(watchers(), 1);
    let high_accuracy =
        js_sys::eval("[...window.__geo.watchers.values()][0].options.enableHighAccuracy")
            .unwrap()
            .as_bool();
    assert_eq!(high_accuracy, Some(true));

    pause();
    assert_eq!(watchers(), 0);
    js_sys::eval("window.__geoEmit(3, 4)").unwrap();
    assert!(coords.read_untracked().is_none());
}

#[wasm_bindgen_test]
async fn stops_watching_on_cleanup() {
    let _ = Executor::init_wasm_bindgen();
    stub_geolocation();

    let owner = Owner::new();
    owner.with(|| {
        let _ = use_geolocation();
    });
    assert_eq!(watchers(), 1);

    owner.cleanup();
    assert_eq!(watchers(), 0);
}
