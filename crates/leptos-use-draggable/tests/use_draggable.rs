#![cfg(target_arch = "wasm32")]

use any_spawner::Executor;
use leptos::prelude::*;
use leptos_use_core::{PointerType, Position};
use leptos_use_draggable::{UseDraggableOptions, UseDraggableReturn, use_draggable_with_options};
use std::sync::{Arc, Mutex};
use wasm_bindgen_test::*;

wasm_bindgen_test_configure!(run_in_browser);

fn append_box() -> web_sys::Element {
    let el = document().create_element("div").unwrap();
    el.set_attribute(
        "style",
        "position: fixed; left: 40px; top: 40px; width: 50px; height: 50px;",
    )
    .unwrap();
    document().body().unwrap().append_child(&el).unwrap();
    el
}

fn pointer(target: &web_sys::EventTarget, name: &str, x: i32, y: i32, pointer_type: &str) {
    let init = web_sys::PointerEventInit::new();
    init.set_pointer_type(pointer_type);
    init.set_client_x(x);
    init.set_client_y(y);
    init.set_bubbles(true);
    let event = web_sys::PointerEvent::new_with_event_init_dict(name, &init).unwrap();
    target.dispatch_event(&event).unwrap();
}

#[wasm_bindgen_test]
async fn drags_the_element_with_the_pointer() {
    let _ = Executor::init_wasm_bindgen();
    let owner = Owner::new();
    owner.set();

    let el = append_box();
    let events = Arc::new(Mutex::new(Vec::new()));

    let UseDraggableReturn {
        x,
        y,
        is_dragging,
        style,
        ..
    } = use_draggable_with_options(
        el.clone(),
        UseDraggableOptions::default()
            .initial_value(Position { x: 40.0, y: 40.0 })
            .on_start({
                let events = Arc::clone(&events);
                move |_| {
                    events.lock().unwrap().push("start");
                    true
                }
            })
            .on_end({
                let events = Arc::clone(&events);
                move |_| events.lock().unwrap().push("end")
            }),
    );
    Executor::tick().await;
    assert!(!is_dragging.get_untracked());

    // grab the box 10px from its top left corner
    pointer(&el, "pointerdown", 50, 50, "mouse");
    assert!(is_dragging.get_untracked());

    pointer(&window(), "pointermove", 110, 80, "mouse");
    assert_eq!((x.get_untracked(), y.get_untracked()), (100.0, 70.0));
    assert_eq!(style.get_untracked(), "left: 100px; top: 70px;");

    pointer(&window(), "pointerup", 110, 80, "mouse");
    assert!(!is_dragging.get_untracked());

    pointer(&window(), "pointermove", 200, 200, "mouse");
    assert_eq!(
        (x.get_untracked(), y.get_untracked()),
        (100.0, 70.0),
        "no dragging after release"
    );

    assert_eq!(*events.lock().unwrap(), vec!["start", "end"]);
}

#[wasm_bindgen_test]
async fn on_start_can_cancel_the_drag() {
    let _ = Executor::init_wasm_bindgen();
    let owner = Owner::new();
    owner.set();

    let el = append_box();

    let UseDraggableReturn { x, is_dragging, .. } = use_draggable_with_options(
        el.clone(),
        UseDraggableOptions::default().on_start(|_| false),
    );
    Executor::tick().await;

    pointer(&el, "pointerdown", 50, 50, "mouse");
    pointer(&window(), "pointermove", 110, 80, "mouse");

    assert!(!is_dragging.get_untracked());
    assert_eq!(x.get_untracked(), 0.0);
}

#[wasm_bindgen_test]
async fn ignores_filtered_pointer_types() {
    let _ = Executor::init_wasm_bindgen();
    let owner = Owner::new();
    owner.set();

    let el = append_box();

    let UseDraggableReturn { is_dragging, .. } = use_draggable_with_options(
        el.clone(),
        UseDraggableOptions::default().pointer_types(vec![PointerType::Touch]),
    );
    Executor::tick().await;

    pointer(&el, "pointerdown", 50, 50, "mouse");
    assert!(!is_dragging.get_untracked());

    pointer(&el, "pointerdown", 50, 50, "touch");
    assert!(is_dragging.get_untracked());
}

#[wasm_bindgen_test]
async fn set_position_moves_the_element() {
    let _ = Executor::init_wasm_bindgen();
    let owner = Owner::new();
    owner.set();

    let el = append_box();

    let UseDraggableReturn {
        position,
        set_position,
        ..
    } = use_draggable_with_options(el, UseDraggableOptions::default());

    set_position.set(Position { x: 5.0, y: 6.0 });
    assert_eq!(position.get_untracked(), Position { x: 5.0, y: 6.0 });
}
