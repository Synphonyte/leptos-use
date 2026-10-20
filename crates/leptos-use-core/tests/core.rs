#![cfg(target_arch = "wasm32")]

use leptos_use_core::{ConnectionReadyState, Direction, Directions, PointerType, ReconnectLimit};
use std::str::FromStr;
use wasm_bindgen_test::*;

wasm_bindgen_test_configure!(run_in_browser);

#[wasm_bindgen_test]
fn directions_set_and_get_each_direction_independently() {
    let all = [
        Direction::Top,
        Direction::Bottom,
        Direction::Left,
        Direction::Right,
    ];

    for direction in all {
        let directions = Directions::default().set_direction(direction, true);

        for other in all {
            assert_eq!(
                directions.get_direction(other),
                other == direction,
                "setting {direction:?} must only affect {direction:?}, checked {other:?}"
            );
        }
    }

    let directions = Directions::default()
        .set_direction(Direction::Left, true)
        .set_direction(Direction::Left, false);
    assert!(!directions.left);
}

#[wasm_bindgen_test]
fn pointer_type_round_trips_through_its_dom_name() {
    for pointer_type in [PointerType::Mouse, PointerType::Touch, PointerType::Pen] {
        assert_eq!(
            PointerType::from_str(&pointer_type.to_string()),
            Ok(pointer_type)
        );
    }

    assert_eq!(PointerType::Mouse.to_string(), "mouse");
    assert_eq!(PointerType::from_str("stylus"), Err(()));
}

#[wasm_bindgen_test]
fn reconnect_limit_is_exceeded_at_the_limit() {
    assert_eq!(ReconnectLimit::default(), ReconnectLimit::Limited(3));

    let limit = ReconnectLimit::Limited(2);
    assert!(!limit.is_exceeded_by(1));
    assert!(limit.is_exceeded_by(2));
    assert!(limit.is_exceeded_by(3));

    assert!(!ReconnectLimit::Infinite.is_exceeded_by(u64::MAX));
}

#[wasm_bindgen_test]
fn connection_ready_state_displays_its_name() {
    assert_eq!(ConnectionReadyState::Connecting.to_string(), "Connecting");
    assert_eq!(ConnectionReadyState::Open.to_string(), "Open");
    assert_eq!(ConnectionReadyState::Closing.to_string(), "Closing");
    assert_eq!(ConnectionReadyState::Closed.to_string(), "Closed");
}
