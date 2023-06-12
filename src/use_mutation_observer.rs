use crate::core::ElementsMaybeSignal;
use crate::{use_supported, watch};
use leptos::*;
use std::cell::RefCell;
use std::rc::Rc;
use wasm_bindgen::prelude::*;
use web_sys::MutationObserverInit;

/// Reactive [MutationObserver](https://developer.mozilla.org/en-US/docs/Web/API/MutationObserver).
///
/// Watch for changes being made to the DOM tree.
///
/// ## Demo
///
/// [Link to Demo](https://github.com/Synphonyte/leptos-use/tree/main/examples/use_mutation_observer)
///
/// ## Usage
///
/// ```
/// # use leptos::*;
/// # use leptos_use::use_mutation_observer_with_options;
/// #
/// # #[component]
/// # fn Demo(cx: Scope) -> impl IntoView {
/// let el = create_node_ref(cx);
/// let (text, set_text) = create_signal(cx, "".to_string());
///
/// let mut init = web_sys::MutationObserverInit::new();
/// init.attributes(true);
///
/// use_mutation_observer_with_options(
///     cx,
///     el,
///     move |mutations, _| {
///         if let Some(mutation) = mutations.first() {
///             set_text.update(|text| *text = format!("{text}\n{:?}", mutation.attribute_name()));
///         }
///     },
///     init,
/// );
///
/// view! { cx,
///     <pre node_ref=el>{ text }</pre>
/// }
/// # }
/// ```
pub fn use_mutation_observer<El, T, F>(
    cx: Scope,
    target: El,
    callback: F,
) -> UseMutationObserverReturn<impl Fn() + Clone>
where
    (Scope, El): Into<ElementsMaybeSignal<T, web_sys::Element>>,
    T: Into<web_sys::Element> + Clone + 'static,
    F: FnMut(Vec<web_sys::MutationRecord>, web_sys::MutationObserver) + 'static,
{
    use_mutation_observer_with_options(cx, target, callback, MutationObserverInit::default())
}

/// Version of [`use_mutation_observer`] that takes a `web_sys::MutationObserverInit`. See [`use_mutation_observer`] for how to use.
pub fn use_mutation_observer_with_options<El, T, F>(
    cx: Scope,
    target: El,
    mut callback: F,
    options: web_sys::MutationObserverInit,
) -> UseMutationObserverReturn<impl Fn() + Clone>
where
    (Scope, El): Into<ElementsMaybeSignal<T, web_sys::Element>>,
    T: Into<web_sys::Element> + Clone + 'static,
    F: FnMut(Vec<web_sys::MutationRecord>, web_sys::MutationObserver) + 'static,
{
    let closure_js = Closure::<dyn FnMut(js_sys::Array, web_sys::MutationObserver)>::new(
        move |entries: js_sys::Array, observer| {
            callback(
                entries
                    .to_vec()
                    .into_iter()
                    .map(|v| v.unchecked_into::<web_sys::MutationRecord>())
                    .collect(),
                observer,
            );
        },
    )
    .into_js_value();

    let observer: Rc<RefCell<Option<web_sys::MutationObserver>>> = Rc::new(RefCell::new(None));

    let is_supported = use_supported(cx, || JsValue::from("MutationObserver").js_in(&window()));

    let obs = Rc::clone(&observer);
    let cleanup = move || {
        let mut observer = obs.borrow_mut();
        if let Some(o) = observer.as_ref() {
            o.disconnect();
            *observer = None;
        }
    };

    let targets = (cx, target).into();

    let clean = cleanup.clone();
    let stop_watch = watch(
        cx,
        move || targets.get(),
        move |targets, _, _| {
            clean();

            if is_supported() && !targets.is_empty() {
                let obs = web_sys::MutationObserver::new(closure_js.as_ref().unchecked_ref())
                    .expect("failed to create MutationObserver");

                for target in targets.iter().flatten() {
                    let target: web_sys::Element = target.clone().into();
                    let _ = obs.observe_with_options(&target, &options.clone());
                }

                observer.replace(Some(obs));
            }
        },
    );

    let stop = move || {
        cleanup();
        stop_watch();
    };

    on_cleanup(cx, stop.clone());

    UseMutationObserverReturn { is_supported, stop }
}

/// The return value of [`use_mutation_observer`].
pub struct UseMutationObserverReturn<F: Fn() + Clone> {
    /// Whether the browser supports the MutationObserver API
    pub is_supported: Signal<bool>,
    /// A function to stop and detach the MutationObserver
    pub stop: F,
}
