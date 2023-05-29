use crate::core::ElementMaybeSignal;
use crate::use_supported;
use default_struct_builder::DefaultBuilder;
use leptos::*;
use std::cell::RefCell;
use std::rc::Rc;
use wasm_bindgen::{JsCast, JsValue};

pub fn use_resize_observer_with_options<El, T, F>(
    cx: Scope,
    target: El, // TODO : multiple elements?
    callback: F,
    options: web_sys::ResizeObserverOptions,
) where
    (Scope, El): Into<ElementMaybeSignal<T, web_sys::Element>>,
    T: Into<web_sys::Element> + Clone + 'static,
    F: FnMut(Vec<web_sys::ResizeObserverEntry>, web_sys::ResizeObserver) + 'static,
{
    let observer: Rc<RefCell<Option<web_sys::ResizeObserver>>> = Rc::new(RefCell::new(None));

    let is_supported = use_supported(cx, || JsValue::from("ResizeObserver").js_in(&window()));

    let obs = Rc::clone(&observer);
    let cleanup = move || {
        let observer = obs.borrow_mut();
        if let Some(o) = *observer {
            o.disconnect();
            *observer = None;
        }
    };

    let target = target.into();

    let clean = cleanup.clone();
    create_effect(cx, move |_| {
        clean();

        if is_supported() {
            let obs = web_sys::ResizeObserver::new(move |entries: &js_sys::Array, observer| {
                callback(
                    entries
                        .to_vec()
                        .into_iter()
                        .map(|v| v.unchecked_into::<web_sys::ResizeObserver>(&options)),
                    observer,
                );
            })
            .expect("failed to create ResizeObserver");

            observer.observe(&target.get());

            observer.replace(obs);
        }
    });

    let stop = move || {
        cleanup();
    };
}
