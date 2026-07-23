#![cfg(target_arch = "wasm32")]

use js_sys::{Object, Reflect};
use leptos::web_sys;
use leptos_use::{UseMouseCoordType, UseMouseEventExtractor};
use std::convert::Infallible;
use wasm_bindgen::JsCast;
use wasm_bindgen_test::*;

wasm_bindgen_test_configure!(run_in_browser);

/// A stand-in for a `Touch`, which cannot be constructed directly. The
/// extractor only reads the coordinate properties.
fn touch(screen: (i32, i32), client: (i32, i32), page: (i32, i32)) -> web_sys::Touch {
    let obj = Object::new();

    for (key, value) in [
        ("screenX", screen.0),
        ("screenY", screen.1),
        ("clientX", client.0),
        ("clientY", client.1),
        ("pageX", page.0),
        ("pageY", page.1),
    ] {
        Reflect::set(&obj, &key.into(), &value.into()).expect("failed to set property");
    }

    obj.unchecked_into()
}

// `UseMouseCoordType::Screen` paired the touch's screen-space x with its
// client-space y, so the reported point mixed two coordinate spaces and its y
// was off by the window's offset from the physical screen.
#[wasm_bindgen_test]
fn screen_coord_type_reads_both_touch_axes_in_screen_space() {
    let touch = touch((500, 800), (120, 700), (120, 900));

    let coord_type = UseMouseCoordType::<Infallible>::Screen;

    assert_eq!(
        coord_type.extract_touch_coords(&touch),
        Some((500.0, 800.0)),
        "Screen must report screenX/screenY"
    );
}

// Guards against regressing the neighbouring arms while fixing `Screen`.
#[wasm_bindgen_test]
fn other_coord_types_stay_in_their_own_space() {
    let touch = touch((500, 800), (120, 700), (120, 900));

    assert_eq!(
        UseMouseCoordType::<Infallible>::Client.extract_touch_coords(&touch),
        Some((120.0, 700.0))
    );
    assert_eq!(
        UseMouseCoordType::<Infallible>::Page.extract_touch_coords(&touch),
        Some((120.0, 900.0))
    );
}
