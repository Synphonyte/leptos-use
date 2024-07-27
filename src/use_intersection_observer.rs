use crate::core::{ElementMaybeSignal, ElementsMaybeSignal};
use cfg_if::cfg_if;
use default_struct_builder::DefaultBuilder;
use leptos::prelude::wrappers::read::Signal;
use leptos::prelude::*;
use std::marker::PhantomData;

cfg_if! { if #[cfg(not(feature = "ssr"))] {
    use crate::{watch_with_options, WatchOptions};
    use std::cell::RefCell;
    use std::rc::Rc;
    use wasm_bindgen::prelude::*;
}}

/// Reactive [IntersectionObserver](https://developer.mozilla.org/en-US/docs/Web/API/IntersectionObserver).
///
/// Detects that a target element's visibility inside the viewport.
///
/// ## Demo
///
/// [Link to Demo](https://github.com/Synphonyte/leptos-use/tree/main/examples/use_intersection_observer)
///
/// ## Usage
///
/// ```
/// # use leptos::prelude::*;
/// # use leptos::html::Div;
/// # use leptos_use::use_intersection_observer;
/// #
/// # #[component]
/// # fn Demo() -> impl IntoView {
/// let el = create_node_ref::<Div>();
/// let (is_visible, set_visible) = signal(false);
///
/// use_intersection_observer(
///     el,
///     move |entries, _| {
///         set_visible.set(entries[0].is_intersecting());
///     },
/// );
///
/// view! {
///     <div node_ref=el>
///         <h1>"Hello World"</h1>
///     </div>
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
/// * [`fn@crate::use_element_visibility`]
pub fn use_intersection_observer<El, T, F>(
    target: El,
    callback: F,
) -> UseIntersectionObserverReturn<impl Fn() + Clone, impl Fn() + Clone, impl Fn() + Clone>
where
    El: Into<ElementsMaybeSignal<T, web_sys::Element>>,
    T: Into<web_sys::Element> + Clone + 'static,
    F: FnMut(Vec<web_sys::IntersectionObserverEntry>, web_sys::IntersectionObserver) + 'static,
{
    use_intersection_observer_with_options::<El, T, web_sys::Element, web_sys::Element, F>(
        target,
        callback,
        UseIntersectionObserverOptions::default(),
    )
}

/// Version of [`use_intersection_observer`] that takes a [`UseIntersectionObserverOptions`]. See [`use_intersection_observer`] for how to use.
#[cfg_attr(feature = "ssr", allow(unused_variables, unused_mut))]
pub fn use_intersection_observer_with_options<El, T, RootEl, RootT, F>(
    target: El,
    mut callback: F,
    options: UseIntersectionObserverOptions<RootEl, RootT>,
) -> UseIntersectionObserverReturn<impl Fn() + Clone, impl Fn() + Clone, impl Fn() + Clone>
where
    El: Into<ElementsMaybeSignal<T, web_sys::Element>>,
    T: Into<web_sys::Element> + Clone + 'static,
    RootEl: Into<ElementMaybeSignal<RootT, web_sys::Element>>,
    RootT: Into<web_sys::Element> + Clone + 'static,
    F: FnMut(Vec<web_sys::IntersectionObserverEntry>, web_sys::IntersectionObserver) + 'static,
{
    let UseIntersectionObserverOptions {
        immediate,
        root,
        root_margin,
        thresholds,
        ..
    } = options;

    let (is_active, set_active) = signal(immediate);

    cfg_if! { if #[cfg(feature = "ssr")] {
        let pause = || {};
        let cleanup = || {};
        let stop = || {};
    } else {
        let closure_js = Closure::<dyn FnMut(js_sys::Array, web_sys::IntersectionObserver)>::new(
            move |entries: js_sys::Array, observer| {
                #[cfg(debug_assertions)]
                let _z = leptos::prelude::diagnostics::SpecialNonReactiveZone::enter();

                callback(
                    entries
                        .to_vec()
                        .into_iter()
                        .map(|v| v.unchecked_into::<web_sys::IntersectionObserverEntry>())
                        .collect(),
                    observer,
                );
            },
        )
        .into_js_value();

        let observer: Rc<RefCell<Option<web_sys::IntersectionObserver>>> =
            Rc::new(RefCell::new(None));

        let cleanup = {
            let obsserver = Rc::clone(&observer);

            move || {
                if let Some(o) = obsserver.take() {
                    o.disconnect();
                }
            }
        };

        let targets = target.into();
        let root = root.map(|root| (root).into());

        let stop_watch = {
            let cleanup = cleanup.clone();

            watch_with_options(
                move || {
                    (
                        targets.get(),
                        root.as_ref().map(|root| root.get()),
                        is_active.get(),
                    )
                },
                move |values, _, _| {
                    let (targets, root, is_active) = values;

                    cleanup();

                    if !is_active {
                        return;
                    }

                    let mut options = web_sys::IntersectionObserverInit::new();
                    options.root_margin(&root_margin).threshold(
                        &thresholds
                            .iter()
                            .copied()
                            .map(JsValue::from)
                            .collect::<js_sys::Array>(),
                    );

                    if let Some(Some(root)) = root {
                        let root: web_sys::Element = root.clone().into();
                        options.root(Some(&root));
                    }

                    let obs = web_sys::IntersectionObserver::new_with_options(
                        closure_js.clone().as_ref().unchecked_ref(),
                        &options,
                    )
                    .expect("failed to create IntersectionObserver");

                    for target in targets.iter().flatten() {
                        let target: web_sys::Element = target.clone().into();
                        obs.observe(&target);
                    }

                    observer.replace(Some(obs));
                },
                WatchOptions::default().immediate(immediate),
            )
        };

        let stop = {
            let cleanup = cleanup.clone();

            move || {
                cleanup();
                stop_watch();
            }
        };

        on_cleanup(stop.clone());

        let pause = {
            let cleanup = cleanup.clone();

            move || {
                cleanup();
                set_active.set(false);
            }
        };
    }}

    UseIntersectionObserverReturn {
        is_active: is_active.into(),
        pause,
        resume: move || {
            cleanup();
            set_active.set(true);
        },
        stop,
    }
}

/// Options for [`use_intersection_observer_with_options`].
#[derive(DefaultBuilder)]
pub struct UseIntersectionObserverOptions<El, T>
where
    El: Into<ElementMaybeSignal<T, web_sys::Element>>,
    T: Into<web_sys::Element> + Clone + 'static,
{
    /// If `true`, the `IntersectionObserver` will be attached immediately. Otherwise it
    /// will only be attached after the returned `resume` closure is called. That is
    /// `use_intersections_observer` will be started "paused".
    immediate: bool,

    /// A `web_sys::Element` or `web_sys::Document` object which is an ancestor of the intended `target`,
    /// whose bounding rectangle will be considered the viewport.
    /// Any part of the target not visible in the visible area of the `root` is not considered visible.
    /// Defaults to `None` (which means the root `document` will be used).
    /// Please note that setting this to a `Some(document)` may not be supported by all browsers.
    /// See [Browser Compatibility](https://developer.mozilla.org/en-US/docs/Web/API/IntersectionObserver/IntersectionObserver#browser_compatibility)
    root: Option<El>,

    /// A string which specifies a set of offsets to add to the root's [bounding box](https://developer.mozilla.org/en-US/docs/Glossary/Bounding_box)
    /// when calculating intersections, effectively shrinking or growing the root for calculation purposes. The syntax is approximately the same as that for the CSS
    /// [`margin`](https://developer.mozilla.org/en-US/docs/Web/CSS/margin) property; see
    /// [The intersection root and root margin](https://developer.mozilla.org/en-US/docs/Web/API/Intersection_Observer_API#the_intersection_root_and_root_margin)
    /// for more information on how the margin works and the syntax. The default is `"0px"`.
    #[builder(into)]
    root_margin: String,

    // TODO : validate that each number is between 0 and 1 ?
    /// A `Vec` of numbers between 0.0 and 1.0, specifying a ratio of intersection area to total
    /// bounding box area for the observed target. A value of 0.0 means that even a single
    /// visible pixel counts as the target being visible. 1.0 means that the entire target
    /// element is visible. See [Thresholds](https://developer.mozilla.org/en-US/docs/Web/API/Intersection_Observer_API#thresholds)
    /// for a more in-depth description of how thresholds are used.
    /// The default is a single threshold of `[0.0]`.
    thresholds: Vec<f64>,

    #[builder(skip)]
    _marker: PhantomData<T>,
}

impl Default for UseIntersectionObserverOptions<web_sys::Element, web_sys::Element> {
    fn default() -> Self {
        Self {
            immediate: true,
            root: None,
            root_margin: "0px".into(),
            thresholds: vec![0.0],
            _marker: PhantomData,
        }
    }
}

/// The return value of [`use_intersection_observer`].
pub struct UseIntersectionObserverReturn<StopFn, PauseFn, ResumeFn>
where
    StopFn: Fn() + Clone,
    PauseFn: Fn() + Clone,
    ResumeFn: Fn() + Clone,
{
    /// Pauses the `IntersectionObserver` observations. Will cause `is_active = false`.
    pub pause: PauseFn,
    /// Resumes the `IntersectionObserver` observations. Will cause `is_active = true`.
    pub resume: ResumeFn,
    /// Stops the `IntersectionObserver` observations altogether.
    pub stop: StopFn,
    /// A signal which is `true` when the `IntersectionObserver` is active, and `false` when paused or stopped.
    pub is_active: Signal<bool>,
}
