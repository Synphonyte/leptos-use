#![cfg(target_arch = "wasm32")]

use any_spawner::Executor;
use gloo_timers::future::TimeoutFuture;
use leptos::prelude::*;
use leptos::wasm_bindgen::JsCast;
use leptos_use_textarea_autosize::{
    UseTextareaAutosizeOptions, UseTextareaAutosizeReturn, use_textarea_autosize,
    use_textarea_autosize_with_options,
};
use wasm_bindgen_test::*;

wasm_bindgen_test_configure!(run_in_browser);

const STYLE: &str =
    "width: 200px; font: 16px/20px monospace; padding: 0; border: 0; box-sizing: content-box;";

fn append_textarea() -> web_sys::HtmlTextAreaElement {
    let el = document().create_element("textarea").unwrap();
    el.set_attribute("style", STYLE).unwrap();
    el.set_attribute("rows", "1").unwrap();
    document().body().unwrap().append_child(&el).unwrap();
    el.unchecked_into()
}

fn height_px(el: &web_sys::HtmlElement) -> f64 {
    el.style()
        .get_property_value("height")
        .unwrap()
        .trim_end_matches("px")
        .parse()
        .unwrap_or_default()
}

#[wasm_bindgen_test]
async fn grows_and_shrinks_with_the_content() {
    let _ = Executor::init_wasm_bindgen();
    let owner = Owner::new();
    owner.set();

    let textarea = append_textarea();
    let UseTextareaAutosizeReturn {
        content,
        set_content,
        ..
    } = use_textarea_autosize(textarea.clone());
    TimeoutFuture::new(50).await;
    let one_line = height_px(&textarea);
    assert!(one_line > 0.0);

    let five_lines = "1\n2\n3\n4\n5".to_string();
    textarea.set_value(&five_lines);
    set_content.set(five_lines.clone());
    TimeoutFuture::new(50).await;
    assert_eq!(content.get_untracked(), five_lines);
    let grown = height_px(&textarea);
    assert!(
        grown >= one_line + 4.0 * 20.0,
        "five lines need more height: {one_line} -> {grown}"
    );

    textarea.set_value("1");
    set_content.set("1".to_string());
    TimeoutFuture::new(50).await;
    assert_eq!(height_px(&textarea), one_line);
}

#[wasm_bindgen_test]
async fn sets_the_height_on_the_style_target() {
    let _ = Executor::init_wasm_bindgen();
    let owner = Owner::new();
    owner.set();

    let textarea = append_textarea();
    let wrapper: web_sys::HtmlElement = document().create_element("div").unwrap().unchecked_into();
    document().body().unwrap().append_child(&wrapper).unwrap();

    let resized = std::sync::Arc::new(std::sync::atomic::AtomicUsize::new(0));
    let UseTextareaAutosizeReturn { set_content, .. } = use_textarea_autosize_with_options(
        textarea.clone(),
        UseTextareaAutosizeOptions::default()
            .style_target(wrapper.clone())
            .on_resize({
                let resized = std::sync::Arc::clone(&resized);
                move || {
                    resized.fetch_add(1, std::sync::atomic::Ordering::SeqCst);
                }
            }),
    );
    TimeoutFuture::new(50).await;
    let before = height_px(&wrapper);
    assert!(before > 0.0);

    textarea.set_value("1\n2\n3");
    set_content.set("1\n2\n3".to_string());
    TimeoutFuture::new(50).await;

    assert!(height_px(&wrapper) > before);
    assert!(resized.load(std::sync::atomic::Ordering::SeqCst) >= 1);
}
