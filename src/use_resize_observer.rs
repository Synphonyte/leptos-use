use crate::core::ElementMaybeSignal;
use crate::{use_supported, watch};
use default_struct_builder::DefaultBuilder;
use leptos::*;
use std::cell::RefCell;
use std::rc::Rc;
use wasm_bindgen::prelude::*;

/// Reports changes to the dimensions of an Element's content or the border-box.
///
/// > This function requires `--cfg=web_sys_unstable_apis` to be activated as
/// [described in the wasm-bindgen guide](https://rustwasm.github.io/docs/wasm-bindgen/web-sys/unstable-apis.html).
///
/// Please refer to [ResizeObserver on MDN](https://developer.mozilla.org/en-US/docs/Web/API/ResizeObserver)
/// for more details.
///
/// ## Demo
///
/// [Link to Demo](https://github.com/Synphonyte/leptos-use/tree/main/examples/use_resize_observer)
///
/// ## Usage
///
/// ```
/// # use leptos::*;
/// # use leptos_use::use_resize_observer;
/// #
/// # #[component]
/// # fn Demo(cx: Scope) -> impl IntoView {
/// let el = create_node_ref(cx);
/// let (text, set_text) = create_signal(cx, "".to_string());
///
/// use_resize_observer(
///     cx,
///     el,
///     move |entries, observer| {
///         let rect = entries[0].content_rect();
///         set_text(format!("width: {}\nheight: {}", rect.width(), rect.height()));
///     },
/// );
///
/// view! { cx,
///     <div node_ref=el>{ text }</div>
/// }
/// # }
/// ```
/// ## See also
///
/// - [`use_element_size`]
pub fn use_resize_observer<El, T, F>(
    cx: Scope,
    target: El, // TODO : multiple elements?
    callback: F,
) -> UseResizeObserverReturn
where
    (Scope, El): Into<ElementMaybeSignal<T, web_sys::Element>>,
    T: Into<web_sys::Element> + Clone + 'static,
    F: FnMut(Vec<web_sys::ResizeObserverEntry>, web_sys::ResizeObserver) + Clone + 'static,
{
    use_resize_observer_with_options(cx, target, callback, UseResizeObserverOptions::default())
}

/// Version of [`use_resize_observer`] that takes a `web_sys::ResizeObserverOptions`. See [`use_resize_observer`] for how to use.
pub fn use_resize_observer_with_options<El, T, F>(
    cx: Scope,
    target: El, // TODO : multiple elements?
    callback: F,
    options: UseResizeObserverOptions,
) -> UseResizeObserverReturn
where
    (Scope, El): Into<ElementMaybeSignal<T, web_sys::Element>>,
    T: Into<web_sys::Element> + Clone + 'static,
    F: FnMut(Vec<web_sys::ResizeObserverEntry>, web_sys::ResizeObserver) + Clone + 'static,
{
    let observer: Rc<RefCell<Option<web_sys::ResizeObserver>>> = Rc::new(RefCell::new(None));

    let is_supported = use_supported(cx, || JsValue::from("ResizeObserver").js_in(&window()));

    let obs = Rc::clone(&observer);
    let cleanup = move || {
        let mut observer = obs.borrow_mut();
        if let Some(o) = observer.as_ref() {
            o.disconnect();
            *observer = None;
        }
    };

    let target = (cx, target).into();

    let clean = cleanup.clone();
    let stop_watch = watch(
        cx,
        move || target.get(),
        move |target, _, _| {
            clean();

            if is_supported() {
                if let Some(target) = target {
                    let mut callback = callback.clone();
                    let closure = Closure::<dyn FnMut(js_sys::Array, web_sys::ResizeObserver)>::new(
                        move |entries: js_sys::Array, observer| {
                            callback(
                                entries
                                    .to_vec()
                                    .into_iter()
                                    .map(|v| v.unchecked_into::<web_sys::ResizeObserverEntry>())
                                    .collect(),
                                observer,
                            );
                        },
                    );
                    let obs = web_sys::ResizeObserver::new(closure.as_ref().unchecked_ref())
                        .expect("failed to create ResizeObserver");

                    closure.forget();

                    let target: web_sys::Element = target.clone().into();
                    obs.observe_with_options(&target, &options.clone().into());

                    observer.replace(Some(obs));
                }
            }
        },
        true,
    );

    let stop = move || {
        cleanup();
        stop_watch();
    };

    on_cleanup(cx, stop.clone());

    UseResizeObserverReturn {
        is_supported,
        stop: Box::new(stop),
    }
}

#[derive(DefaultBuilder, Clone)]
/// Options for [`use_resize_observer_with_options`].
pub struct UseResizeObserverOptions {
    /// The box that is used to determine the dimensions of the target. Defaults to `ContentBox`.
    pub box_: web_sys::ResizeObserverBoxOptions,
}

impl Default for UseResizeObserverOptions {
    fn default() -> Self {
        Self {
            box_: web_sys::ResizeObserverBoxOptions::ContentBox,
        }
    }
}

impl Into<web_sys::ResizeObserverOptions> for UseResizeObserverOptions {
    fn into(self) -> web_sys::ResizeObserverOptions {
        let mut options = web_sys::ResizeObserverOptions::new();
        options.box_(self.box_);
        options
    }
}

/// The return value of [`use_resize_observer`].
pub struct UseResizeObserverReturn {
    /// Whether the browser supports the ResizeObserver API
    pub is_supported: Signal<bool>,
    /// A function to stop and detach the ResizeObserver
    pub stop: Box<dyn Fn()>,
}
