use crate::core::ElementsMaybeSignal;
use cfg_if::cfg_if;
use default_struct_builder::DefaultBuilder;
use leptos::prelude::wrappers::read::Signal;

cfg_if! { if #[cfg(not(feature = "ssr"))] {
    use crate::use_supported;
    use std::cell::RefCell;
    use std::rc::Rc;
    use wasm_bindgen::prelude::*;
    use leptos::prelude::*;
}}

/// Reports changes to the dimensions of an Element's content or the border-box.
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
/// # use leptos::{html::Div, *};
/// # use leptos_use::use_resize_observer;
/// #
/// # #[component]
/// # fn Demo() -> impl IntoView {
/// let el = create_node_ref::<Div>();
/// let (text, set_text) = signal("".to_string());
///
/// use_resize_observer(
///     el,
///     move |entries, observer| {
///         let rect = entries[0].content_rect();
///         set_text.set(format!("width: {}\nheight: {}", rect.width(), rect.height()));
///     },
/// );
///
/// view! {
///     <div node_ref=el>{ move || text.get() }</div>
/// }
/// # }
/// ```
///
/// ## Server-Side Rendering
///
/// On the server this amounts to a no-op.
///
/// ## See also
///
/// - [`use_element_size`]
pub fn use_resize_observer<El, T, F>(
    target: El, // TODO : multiple elements?
    callback: F,
) -> UseResizeObserverReturn<impl Fn() + Clone>
where
    El: Into<ElementsMaybeSignal<T, web_sys::Element>>,
    T: Into<web_sys::Element> + Clone + 'static,
    F: FnMut(Vec<web_sys::ResizeObserverEntry>, web_sys::ResizeObserver) + 'static,
{
    use_resize_observer_with_options(target, callback, UseResizeObserverOptions::default())
}

/// Version of [`use_resize_observer`] that takes a `web_sys::ResizeObserverOptions`. See [`use_resize_observer`] for how to use.
#[cfg_attr(feature = "ssr", allow(unused_variables, unused_mut))]
pub fn use_resize_observer_with_options<El, T, F>(
    target: El, // TODO : multiple elements?
    mut callback: F,
    options: UseResizeObserverOptions,
) -> UseResizeObserverReturn<impl Fn() + Clone>
where
    El: Into<ElementsMaybeSignal<T, web_sys::Element>>,
    T: Into<web_sys::Element> + Clone + 'static,
    F: FnMut(Vec<web_sys::ResizeObserverEntry>, web_sys::ResizeObserver) + 'static,
{
    #[cfg(feature = "ssr")]
    {
        UseResizeObserverReturn {
            is_supported: Signal::derive(|| true),
            stop: || {},
        }
    }

    #[cfg(not(feature = "ssr"))]
    {
        use crate::js;

        let closure_js = Closure::<dyn FnMut(js_sys::Array, web_sys::ResizeObserver)>::new(
            move |entries: js_sys::Array, observer| {
                #[cfg(debug_assertions)]
                let _z = leptos::prelude::diagnostics::SpecialNonReactiveZone::enter();

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

        let is_supported = use_supported(|| js!("ResizeObserver" in &window()));

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

        let targets = target.into();

        let stop_watch = {
            let cleanup = cleanup.clone();

            watch(
                move || targets.get(),
                move |targets, _, _| {
                    cleanup();

                    if is_supported.get_untracked() && !targets.is_empty() {
                        let obs = web_sys::ResizeObserver::new(
                            closure_js.clone().as_ref().unchecked_ref(),
                        )
                        .expect("failed to create ResizeObserver");

                        for target in targets.iter().flatten() {
                            let target: web_sys::Element = target.clone().into();
                            obs.observe_with_options(&target, &options.clone().into());
                        }
                        observer.replace(Some(obs));
                    }
                },
                true,
            )
        };

        let stop = move || {
            cleanup();
            stop_watch();
        };

        on_cleanup({
            let stop = send_wrapper::SendWrapper::new(stop.clone());
            move || stop()
        });

        UseResizeObserverReturn { is_supported, stop }
    }
}

/// Options for [`use_resize_observer_with_options`].
#[derive(DefaultBuilder, Clone, Default)]
pub struct UseResizeObserverOptions {
    /// The box that is used to determine the dimensions of the target. Defaults to `ContentBox`.
    #[builder(into)]
    pub box_: Option<web_sys::ResizeObserverBoxOptions>,
}

impl From<UseResizeObserverOptions> for web_sys::ResizeObserverOptions {
    fn from(val: UseResizeObserverOptions) -> Self {
        let mut options = web_sys::ResizeObserverOptions::new();
        options.box_(
            val.box_
                .unwrap_or(web_sys::ResizeObserverBoxOptions::ContentBox),
        );
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
