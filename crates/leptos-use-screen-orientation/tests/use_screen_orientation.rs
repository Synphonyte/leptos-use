#![cfg(target_arch = "wasm32")]

use any_spawner::Executor;
use leptos::prelude::*;
use leptos_use_screen_orientation::{
    ScreenOrientation, UseScreenOrientationReturn, use_screen_orientation,
};
use wasm_bindgen_test::*;

wasm_bindgen_test_configure!(run_in_browser);

fn screen_orientation() -> web_sys::ScreenOrientation {
    window().screen().unwrap().orientation()
}

#[wasm_bindgen_test]
async fn reports_the_screen_orientation() {
    let _ = Executor::init_wasm_bindgen();
    let owner = Owner::new();
    owner.set();

    let UseScreenOrientationReturn {
        orientation, angle, ..
    } = use_screen_orientation();

    let expected: ScreenOrientation = screen_orientation().type_().unwrap().into();
    assert_eq!(orientation.get_untracked(), expected);
    assert_eq!(angle.get_untracked(), screen_orientation().angle().unwrap());
}

// The orientation can't be rotated headlessly, so it is shadowed on the
// `ScreenOrientation` instance before the change event is dispatched.
#[wasm_bindgen_test]
async fn follows_orientation_changes() {
    let _ = Executor::init_wasm_bindgen();
    let owner = Owner::new();
    owner.set();

    let UseScreenOrientationReturn {
        orientation, angle, ..
    } = use_screen_orientation();
    Executor::tick().await;

    js_sys::eval(
        r#"
        Object.defineProperty(screen.orientation, "type", { configurable: true, value: "portrait-secondary" });
        Object.defineProperty(screen.orientation, "angle", { configurable: true, value: 180 });
        window.dispatchEvent(new Event("orientationchange"));
        delete screen.orientation.type;
        delete screen.orientation.angle;
        "#,
    )
    .unwrap();

    assert_eq!(
        orientation.get_untracked(),
        ScreenOrientation::PortraitSecondary
    );
    assert_eq!(angle.get_untracked(), 180);
}
