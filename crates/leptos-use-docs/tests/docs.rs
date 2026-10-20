#![cfg(target_arch = "wasm32")]

use any_spawner::Executor;
use leptos::prelude::*;
use leptos::wasm_bindgen::JsCast;
use leptos_use_docs::{BooleanDisplay, Note, demo_or_body};
use wasm_bindgen_test::*;

wasm_bindgen_test_configure!(run_in_browser);

fn mount_container() -> web_sys::HtmlElement {
    let el = document().create_element("div").unwrap();
    document().body().unwrap().append_child(&el).unwrap();
    el.unchecked_into()
}

#[wasm_bindgen_test]
async fn boolean_display_follows_the_value() {
    let _ = Executor::init_wasm_bindgen();

    let container = mount_container();
    let (value, set_value) = signal(false);

    let _handle = leptos::mount::mount_to(container.clone(), move || {
        view! { <BooleanDisplay value=value true_str="on" false_str="off" class="extra" /> }
    });

    let span = container.query_selector("span").unwrap().unwrap();
    assert_eq!(span.text_content().as_deref(), Some("off"));
    assert!(span.class_list().contains("extra"));
    assert!(!span.class_list().contains("text-green-600"));

    set_value.set(true);
    Executor::tick().await;

    assert_eq!(span.text_content().as_deref(), Some("on"));
    assert!(span.class_list().contains("text-green-600"));
}

#[wasm_bindgen_test]
async fn note_wraps_its_children() {
    let _ = Executor::init_wasm_bindgen();

    let container = mount_container();

    let _handle = leptos::mount::mount_to(container.clone(), || {
        view! { <Note class="hint">"Remember this"</Note> }
    });

    let note = container.query_selector("div.note").unwrap().unwrap();
    assert!(note.class_list().contains("hint"));
    assert_eq!(note.text_content().as_deref(), Some("Remember this"));
}

#[wasm_bindgen_test]
fn demo_or_body_prefers_the_demo_anchor() {
    assert_eq!(demo_or_body(), document().body().unwrap());

    let anchor = mount_container();
    anchor.set_id("demo-anchor");
    assert_eq!(demo_or_body(), anchor);
    anchor.remove();
}
