#![cfg(target_arch = "wasm32")]

use any_spawner::Executor;
use leptos::prelude::*;
use leptos_use_drop_zone::{UseDropZoneOptions, UseDropZoneReturn, use_drop_zone_with_options};
use std::sync::{Arc, Mutex};
use wasm_bindgen_test::*;

wasm_bindgen_test_configure!(run_in_browser);

fn append_div() -> web_sys::Element {
    let el = document().create_element("div").unwrap();
    document().body().unwrap().append_child(&el).unwrap();
    el
}

fn file(name: &str) -> web_sys::File {
    let parts = js_sys::Array::of1(&"content".into());
    web_sys::File::new_with_str_sequence(&parts, name).unwrap()
}

fn drag(el: &web_sys::Element, name: &str, files: &[web_sys::File]) {
    let data_transfer = web_sys::DataTransfer::new().unwrap();
    for f in files {
        data_transfer.items().add_with_file(f).unwrap();
    }
    let init = web_sys::DragEventInit::new();
    init.set_data_transfer(Some(&data_transfer));
    init.set_bubbles(true);
    init.set_cancelable(true);
    let event = web_sys::DragEvent::new_with_event_init_dict(name, &init).unwrap();
    el.dispatch_event(&event).unwrap();
}

#[wasm_bindgen_test]
async fn tracks_dragging_over_and_dropping_files() {
    let _ = Executor::init_wasm_bindgen();
    let owner = Owner::new();
    owner.set();

    let el = append_div();
    let events = Arc::new(Mutex::new(Vec::new()));
    let record = |name: &'static str| {
        let events = Arc::clone(&events);
        move |event: leptos_use_drop_zone::UseDropZoneEvent| {
            let names: Vec<_> = event.files.iter().map(|f| f.name()).collect();
            events.lock().unwrap().push((name, names));
        }
    };

    let UseDropZoneReturn {
        files,
        is_over_drop_zone,
    } = use_drop_zone_with_options(
        el.clone(),
        UseDropZoneOptions::default()
            .on_enter(record("enter"))
            .on_leave(record("leave"))
            .on_drop(record("drop")),
    );
    Executor::tick().await;
    assert!(!is_over_drop_zone.get_untracked());

    let dropped = [file("a.txt"), file("b.txt")];

    drag(&el, "dragenter", &dropped);
    assert!(is_over_drop_zone.get_untracked());

    drag(&el, "dragover", &dropped);
    drag(&el, "drop", &dropped);
    assert!(!is_over_drop_zone.get_untracked());

    let names: Vec<_> = files.get_untracked().iter().map(|f| f.name()).collect();
    assert_eq!(names, vec!["a.txt", "b.txt"]);

    let events = events.lock().unwrap();
    assert_eq!(events.first().map(|e| e.0), Some("enter"));
    assert_eq!(
        events.last().cloned(),
        Some(("drop", vec!["a.txt".to_string(), "b.txt".to_string()]))
    );
}

// Entering a child fires another `dragenter` before the `dragleave` of the
// parent, so the zone must only count as left when every enter was matched.
#[wasm_bindgen_test]
async fn nested_enters_and_leaves_are_counted() {
    let _ = Executor::init_wasm_bindgen();
    let owner = Owner::new();
    owner.set();

    let el = append_div();
    let UseDropZoneReturn {
        is_over_drop_zone, ..
    } = use_drop_zone_with_options(el.clone(), UseDropZoneOptions::default());
    Executor::tick().await;

    drag(&el, "dragenter", &[]);
    drag(&el, "dragenter", &[]);
    drag(&el, "dragleave", &[]);
    assert!(is_over_drop_zone.get_untracked(), "still over the zone");

    drag(&el, "dragleave", &[]);
    assert!(!is_over_drop_zone.get_untracked());
}
