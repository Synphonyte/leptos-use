#![cfg(target_arch = "wasm32")]

use any_spawner::Executor;
use gloo_timers::future::TimeoutFuture;
use leptos::prelude::*;
use leptos::wasm_bindgen::JsCast;
use leptos_use_resize_observer::{
    UseResizeObserverOptions, UseResizeObserverReturn, use_resize_observer,
    use_resize_observer_with_options,
};
use std::cell::RefCell;
use std::rc::Rc;
use wasm_bindgen_test::*;

wasm_bindgen_test_configure!(run_in_browser);

fn append_box(style: &str) -> web_sys::Element {
    let el = document().create_element("div").unwrap();
    el.set_attribute("style", style).unwrap();
    document().body().unwrap().append_child(&el).unwrap();
    el
}

type Sizes = Rc<RefCell<Vec<(f64, f64)>>>;

fn recorder() -> (
    Sizes,
    impl FnMut(Vec<web_sys::ResizeObserverEntry>, web_sys::ResizeObserver) + 'static,
) {
    let sizes: Sizes = Default::default();
    let cb = {
        let sizes = Rc::clone(&sizes);
        move |entries: Vec<web_sys::ResizeObserverEntry>, _| {
            for entry in entries {
                let rect = entry.content_rect();
                sizes.borrow_mut().push((rect.width(), rect.height()));
            }
        }
    };
    (sizes, cb)
}

#[wasm_bindgen_test]
async fn reports_size_changes_until_stopped() {
    let _ = Executor::init_wasm_bindgen();
    let owner = Owner::new();
    owner.set();

    let el = append_box("width: 50px; height: 20px;");
    let (sizes, cb) = recorder();

    let UseResizeObserverReturn { is_supported, stop } = use_resize_observer(el.clone(), cb);
    TimeoutFuture::new(100).await;
    assert!(is_supported.get_untracked());
    assert_eq!(*sizes.borrow(), vec![(50.0, 20.0)]);

    el.set_attribute("style", "width: 80px; height: 30px;")
        .unwrap();
    TimeoutFuture::new(100).await;
    assert_eq!(*sizes.borrow(), vec![(50.0, 20.0), (80.0, 30.0)]);

    stop();
    el.set_attribute("style", "width: 10px; height: 10px;")
        .unwrap();
    TimeoutFuture::new(100).await;
    assert_eq!(sizes.borrow().len(), 2, "no callbacks after stop");
}

#[wasm_bindgen_test]
async fn observes_multiple_elements() {
    let _ = Executor::init_wasm_bindgen();
    let owner = Owner::new();
    owner.set();

    let a = append_box("width: 10px; height: 10px;");
    let b = append_box("width: 20px; height: 20px;");
    let (sizes, cb) = recorder();

    let _ = use_resize_observer(vec![a, b], cb);
    TimeoutFuture::new(100).await;

    let mut sizes = sizes.borrow().clone();
    sizes.sort_by(|x, y| x.partial_cmp(y).unwrap());
    assert_eq!(sizes, vec![(10.0, 10.0), (20.0, 20.0)]);
}

#[wasm_bindgen_test]
async fn border_box_reports_the_border_box_size() {
    let _ = Executor::init_wasm_bindgen();
    let owner = Owner::new();
    owner.set();

    let el = append_box("width: 50px; height: 20px; padding: 5px;");
    let widths = Rc::new(RefCell::new(Vec::new()));

    let _ = use_resize_observer_with_options(
        el,
        {
            let widths = Rc::clone(&widths);
            move |entries, _| {
                for entry in entries {
                    let size: web_sys::ResizeObserverSize =
                        entry.border_box_size().get(0).unchecked_into();
                    widths.borrow_mut().push(size.inline_size());
                }
            }
        },
        UseResizeObserverOptions::default().box_(web_sys::ResizeObserverBoxOptions::BorderBox),
    );
    TimeoutFuture::new(100).await;

    assert_eq!(*widths.borrow(), vec![60.0]);
}
