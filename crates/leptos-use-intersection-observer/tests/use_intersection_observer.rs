#![cfg(target_arch = "wasm32")]

use any_spawner::Executor;
use gloo_timers::future::TimeoutFuture;
use leptos::prelude::*;
use leptos_use_intersection_observer::{
    UseIntersectionObserverOptions, UseIntersectionObserverReturn, use_intersection_observer,
    use_intersection_observer_with_options,
};
use std::cell::RefCell;
use std::rc::Rc;
use wasm_bindgen_test::*;

wasm_bindgen_test_configure!(run_in_browser);

fn append_box(parent: &web_sys::Element, style: &str) -> web_sys::Element {
    let el = document().create_element("div").unwrap();
    el.set_attribute("style", style).unwrap();
    parent.append_child(&el).unwrap();
    el
}

fn body() -> web_sys::Element {
    document().body().unwrap().into()
}

type Seen = Rc<RefCell<Vec<bool>>>;

fn recorder() -> (
    Seen,
    impl FnMut(Vec<web_sys::IntersectionObserverEntry>, web_sys::IntersectionObserver) + 'static,
) {
    let seen: Seen = Default::default();
    let cb = {
        let seen = Rc::clone(&seen);
        move |entries: Vec<web_sys::IntersectionObserverEntry>, _| {
            seen.borrow_mut()
                .extend(entries.iter().map(|e| e.is_intersecting()));
        }
    };
    (seen, cb)
}

#[wasm_bindgen_test]
async fn reports_intersection_changes() {
    let _ = Executor::init_wasm_bindgen();
    let owner = Owner::new();
    owner.set();

    let el = append_box(
        &body(),
        "position: fixed; left: 0; top: 0; width: 20px; height: 20px;",
    );
    let (seen, cb) = recorder();

    let UseIntersectionObserverReturn { is_active, .. } = use_intersection_observer(el.clone(), cb);
    TimeoutFuture::new(150).await;
    assert!(is_active.get_untracked());
    assert_eq!(*seen.borrow(), vec![true]);

    el.set_attribute(
        "style",
        "position: fixed; left: -100px; top: 0; width: 20px; height: 20px;",
    )
    .unwrap();
    TimeoutFuture::new(150).await;
    assert_eq!(*seen.borrow(), vec![true, false]);
}

#[wasm_bindgen_test]
async fn pause_and_resume_observing() {
    let _ = Executor::init_wasm_bindgen();
    let owner = Owner::new();
    owner.set();

    let el = append_box(
        &body(),
        "position: fixed; left: 0; top: 0; width: 20px; height: 20px;",
    );
    let (seen, cb) = recorder();

    let UseIntersectionObserverReturn {
        pause,
        resume,
        is_active,
        ..
    } = use_intersection_observer(el.clone(), cb);
    TimeoutFuture::new(150).await;
    assert_eq!(seen.borrow().len(), 1);

    pause();
    assert!(!is_active.get_untracked());
    el.set_attribute(
        "style",
        "position: fixed; left: -100px; top: 0; width: 20px; height: 20px;",
    )
    .unwrap();
    TimeoutFuture::new(150).await;
    assert_eq!(seen.borrow().len(), 1, "no callbacks while paused");

    resume();
    assert!(is_active.get_untracked());
    TimeoutFuture::new(150).await;
    assert_eq!(
        seen.borrow().last(),
        Some(&false),
        "resuming reports the current state"
    );
}

#[wasm_bindgen_test]
async fn observes_relative_to_a_root_element() {
    let _ = Executor::init_wasm_bindgen();
    let owner = Owner::new();
    owner.set();

    let root = append_box(&body(), "width: 100px; height: 100px; overflow: scroll;");
    let el = append_box(&root, "margin-top: 300px; width: 20px; height: 20px;");
    let (seen, cb) = recorder();

    let _ = use_intersection_observer_with_options(
        el.clone(),
        cb,
        UseIntersectionObserverOptions::default().root(Some(root.clone())),
    );
    TimeoutFuture::new(150).await;
    assert_eq!(
        *seen.borrow(),
        vec![false],
        "hidden below the root's scroll area"
    );

    root.set_scroll_top(300.0);
    TimeoutFuture::new(150).await;
    assert_eq!(*seen.borrow(), vec![false, true]);
}

#[wasm_bindgen_test]
async fn does_not_start_when_not_immediate() {
    let _ = Executor::init_wasm_bindgen();
    let owner = Owner::new();
    owner.set();

    let el = append_box(&body(), "width: 20px; height: 20px;");
    let (seen, cb) = recorder();

    let UseIntersectionObserverReturn {
        is_active, resume, ..
    } = use_intersection_observer_with_options(
        el,
        cb,
        UseIntersectionObserverOptions::<web_sys::Element, _>::default().immediate(false),
    );
    TimeoutFuture::new(150).await;
    assert!(!is_active.get_untracked());
    assert!(seen.borrow().is_empty());

    resume();
    TimeoutFuture::new(150).await;
    assert_eq!(*seen.borrow(), vec![true]);
}
