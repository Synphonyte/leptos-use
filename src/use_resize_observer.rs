use crate::core::ElementsMaybeSignal;
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
) -> UseResizeObserverReturn<impl Fn() + Clone>
where
    (Scope, El): Into<ElementsMaybeSignal<T, web_sys::Element>>,
    T: Into<web_sys::Element> + Clone + 'static,
    F: FnMut(Vec<web_sys::ResizeObserverEntry>, web_sys::ResizeObserver) + 'static,
{
    use_resize_observer_with_options(cx, target, callback, UseResizeObserverOptions::default())
}

/// Version of [`use_resize_observer`] that takes a `web_sys::ResizeObserverOptions`. See [`use_resize_observer`] for how to use.
pub fn use_resize_observer_with_options<El, T, F>(
    cx: Scope,
    target: El, // TODO : multiple elements?
    mut callback: F,
    options: UseResizeObserverOptions,
) -> UseResizeObserverReturn<impl Fn() + Clone>
where
    (Scope, El): Into<ElementsMaybeSignal<T, web_sys::Element>>,
    T: Into<web_sys::Element> + Clone + 'static,
    F: FnMut(Vec<web_sys::ResizeObserverEntry>, web_sys::ResizeObserver) + 'static,
{
    let closure_js = Closure::<dyn FnMut(js_sys::Array, web_sys::ResizeObserver)>::new(
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
    )
    .into_js_value();

    let observer: Rc<RefCell<Option<web_sys::ResizeObserver>>> = Rc::new(RefCell::new(None));

    let is_supported = use_supported(cx, || JsValue::from("ResizeObserver").js_in(&window()));

    let cleanup = {
        let observer = Rc::clone(&observer);

        move || {
            let mut observer = observer.borrow_mut();
            if let Some(o) = observer.as_ref() {
                o.disconnect();
                *observer = None;
            }
        }
    };

    let targets = (cx, target).into();

    let stop_watch = {
        let cleanup = cleanup.clone();

        watch(
            cx,
            move || targets.get(),
            move |targets, _, _| {
                cleanup();

                if is_supported() && !targets.is_empty() {
                    let obs =
                        web_sys::ResizeObserver::new(closure_js.clone().as_ref().unchecked_ref())
                            .expect("failed to create ResizeObserver");

                    for target in targets.iter().flatten() {
                        let target: web_sys::Element = target.clone().into();
                        obs.observe_with_options(&target, &options.clone().into());
                    }

                    observer.replace(Some(obs));
                }
            },
        )
    };

    let stop = move || {
        cleanup();
        stop_watch();
    };

    on_cleanup(cx, stop.clone());

    UseResizeObserverReturn { is_supported, stop }
}

/// Options for [`use_resize_observer_with_options`].
#[derive(DefaultBuilder, Clone)]
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

impl From<UseResizeObserverOptions> for web_sys::ResizeObserverOptions {
    fn from(val: UseResizeObserverOptions) -> Self {
        let mut options = web_sys::ResizeObserverOptions::new();
        options.box_(val.box_);
        options
    }
}

/// The return value of [`use_resize_observer`].
pub struct UseResizeObserverReturn<F: Fn() + Clone> {
    /// Whether the browser supports the ResizeObserver API
    pub is_supported: Signal<bool>,
    /// A function to stop and detach the ResizeObserver
    pub stop: F,
}
