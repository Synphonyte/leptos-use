#![cfg(target_arch = "wasm32")]

use any_spawner::Executor;
use leptos::prelude::*;
use leptos_use_device_orientation::{UseDeviceOrientationReturn, use_device_orientation};
use wasm_bindgen_test::*;

wasm_bindgen_test_configure!(run_in_browser);

fn fire_orientation(alpha: f64, beta: f64, gamma: f64, absolute: bool) {
    let init = web_sys::DeviceOrientationEventInit::new();
    init.set_alpha(Some(alpha));
    init.set_beta(Some(beta));
    init.set_gamma(Some(gamma));
    init.set_absolute(absolute);
    let event =
        web_sys::DeviceOrientationEvent::new_with_event_init_dict("deviceorientation", &init)
            .unwrap();
    window().dispatch_event(&event).unwrap();
}

#[wasm_bindgen_test]
async fn follows_device_orientation_events() {
    let _ = Executor::init_wasm_bindgen();
    let owner = Owner::new();
    owner.set();

    let UseDeviceOrientationReturn {
        is_supported,
        absolute,
        alpha,
        beta,
        gamma,
    } = use_device_orientation();
    Executor::tick().await;
    assert!(is_supported.get_untracked());
    assert_eq!(alpha.get_untracked(), None);

    fire_orientation(10.0, 20.0, 30.0, true);

    assert_eq!(alpha.get_untracked(), Some(10.0));
    assert_eq!(beta.get_untracked(), Some(20.0));
    assert_eq!(gamma.get_untracked(), Some(30.0));
    assert!(absolute.get_untracked());
}
