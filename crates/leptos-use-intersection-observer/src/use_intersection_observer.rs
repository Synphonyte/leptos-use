use crate::{
    IntoElementMaybeSignal, IntoElementsMaybeSignal, UseIntersectionObserverOptions,
    UseIntersectionObserverReturn, sendwrap_fn,
};
use cfg_if::cfg_if;
use leptos::prelude::*;

cfg_if! { if #[cfg(not(feature = "ssr"))] {
    use crate::{watch_with_options, WatchOptions};
    // use std::cell::RefCell;
    // use std::rc::Rc;
    use std::sync::{Arc, Mutex};
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
/// # use leptos_use_intersection_observer::use_intersection_observer;
/// #
/// # #[component]
/// # fn Demo() -> impl IntoView {
/// let el = NodeRef::<Div>::new();
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
/// ## SendWrapped Return
///
/// The returned closures `pause`, `resume` and `stop` are sendwrapped functions. They can
/// only be called from the same thread that called `use_intersection_observer`.
///
/// ## Server-Side Rendering
///
/// > Make sure you follow the [instructions in Server-Side Rendering](https://leptos-use.rs/server_side_rendering.html).
///
/// On the server this amounts to a no-op.
///
/// ## See also
///
/// * [`fn@crate::use_element_visibility`]
pub fn use_intersection_observer<Els, M, F, RootM>(
    target: Els,
    callback: F,
) -> UseIntersectionObserverReturn<
    impl Fn() + Clone + Send + Sync,
    impl Fn() + Clone + Send + Sync,
    impl Fn() + Clone + Send + Sync,
>
where
    Els: IntoElementsMaybeSignal<web_sys::Element, M>,
    F: FnMut(Vec<web_sys::IntersectionObserverEntry>, web_sys::IntersectionObserver) + 'static,
    web_sys::Element: IntoElementMaybeSignal<web_sys::Element, RootM>,
{
    use_intersection_observer_with_options::<Els, M, web_sys::Element, RootM, F>(
        target,
        callback,
        UseIntersectionObserverOptions::default(),
    )
}

/// Version of [`use_intersection_observer`] that takes a [`UseIntersectionObserverOptions`]. See [`use_intersection_observer`] for how to use.
#[cfg_attr(feature = "ssr", allow(unused_variables, unused_mut))]
pub fn use_intersection_observer_with_options<Els, M, RootEl, RootM, F>(
    target: Els,
    mut callback: F,
    options: UseIntersectionObserverOptions<RootEl, RootM>,
) -> UseIntersectionObserverReturn<
    impl Fn() + Clone + Send + Sync,
    impl Fn() + Clone + Send + Sync,
    impl Fn() + Clone + Send + Sync,
>
where
    Els: IntoElementsMaybeSignal<web_sys::Element, M>,
    RootEl: IntoElementMaybeSignal<web_sys::Element, RootM>,
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

    let pause;
    let cleanup;
    let stop;

    #[cfg(feature = "ssr")]
    {
        pause = || {};
        cleanup = || {};
        stop = || {};
    }

    #[cfg(not(feature = "ssr"))]
    {
        use send_wrapper::SendWrapper;

        let closure_js = Closure::<dyn FnMut(js_sys::Array, web_sys::IntersectionObserver)>::new(
            move |entries: js_sys::Array, observer| {
                #[cfg(debug_assertions)]
                let _z = leptos::reactive::diagnostics::SpecialNonReactiveZone::enter();

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

        let observer: Arc<Mutex<Option<SendWrapper<web_sys::IntersectionObserver>>>> =
            Arc::new(Mutex::new(None));

        cleanup = {
            let observer = Arc::clone(&observer);

            move || {
                if let Some(o) = observer.lock().unwrap().take() {
                    o.disconnect();
                }
            }
        };

        let targets = target.into_elements_maybe_signal();
        let root = root.map(|root| root.into_element_maybe_signal());

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

                    let options = web_sys::IntersectionObserverInit::new();
                    options.set_root_margin(&root_margin);
                    options.set_threshold(
                        &thresholds
                            .iter()
                            .copied()
                            .map(JsValue::from)
                            .collect::<js_sys::Array>(),
                    );

                    if let Some(Some(root)) = root {
                        let root = root.clone();
                        options.set_root(Some(&root));
                    }

                    let obs = web_sys::IntersectionObserver::new_with_options(
                        closure_js.clone().as_ref().unchecked_ref(),
                        &options,
                    )
                    .expect("failed to create IntersectionObserver");

                    for target in targets.iter().flatten() {
                        let target = target.clone();
                        obs.observe(&target);
                    }

                    *observer.lock().unwrap() = Some(SendWrapper::new(obs));
                },
                WatchOptions::default().immediate(immediate),
            )
        };

        stop = {
            let cleanup = cleanup.clone();

            sendwrap_fn!(move || {
                cleanup();
                stop_watch();
            })
        };

        on_cleanup(stop.clone());

        pause = {
            let cleanup = cleanup.clone();

            sendwrap_fn!(move || {
                cleanup();
                set_active.set(false);
            })
        };
    }

    UseIntersectionObserverReturn {
        is_active: is_active.into(),
        pause,
        resume: sendwrap_fn!(move || {
            cleanup();
            set_active.set(true);
        }),
        stop,
    }
}
