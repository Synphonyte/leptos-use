use crate::{IntoElementsMaybeSignal, UseMutationObserverOptions, UseMutationObserverReturn};
use cfg_if::cfg_if;
#[cfg(feature = "ssr")]
use leptos::reactive::wrappers::read::Signal;
#[cfg(not(feature = "ssr"))]
use wasm_bindgen::prelude::*;

cfg_if! { if #[cfg(not(feature = "ssr"))] {
    use crate::{sendwrap_fn, use_supported};
    use leptos::prelude::*;
    use std::cell::RefCell;
    use std::rc::Rc;
}}

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
/// # use leptos::prelude::*;
/// # use leptos::html::Pre;
/// # use leptos_use_element::{use_mutation_observer_with_options, UseMutationObserverOptions};
/// #
/// # #[component]
/// # fn Demo() -> impl IntoView {
/// let el = NodeRef::<Pre>::new();
/// let (text, set_text) = signal("".to_string());
///
/// use_mutation_observer_with_options(
///     el,
///     move |mutations, _| {
///         if let Some(mutation) = mutations.first() {
///             set_text.update(|text| *text = format!("{text}\n{:?}", mutation.attribute_name()));
///         }
///     },
///     UseMutationObserverOptions::default().attributes(true),
/// );
///
/// view! {
///     <pre node_ref=el>{ text }</pre>
/// }
/// # }
/// ```
///
/// ## SendWrapped Return
///
/// The returned closure `stop` is a sendwrapped function. It can
/// only be called from the same thread that called `use_mouse_in_element`.
///
/// ## Server-Side Rendering
///
/// > Make sure you follow the [instructions in Server-Side Rendering](https://leptos-use.rs/server_side_rendering.html).
///
/// On the server this amounts to a no-op.
pub fn use_mutation_observer<El, M, F>(
    target: El,
    callback: F,
) -> UseMutationObserverReturn<impl Fn() + Clone + Send + Sync>
where
    El: IntoElementsMaybeSignal<web_sys::Element, M>,
    F: FnMut(Vec<web_sys::MutationRecord>, web_sys::MutationObserver) + 'static,
{
    use_mutation_observer_with_options(target, callback, UseMutationObserverOptions::default())
}

/// Version of [`use_mutation_observer`] that takes a `UseMutationObserverOptions`. See [`use_mutation_observer`] for how to use.
#[cfg_attr(feature = "ssr", allow(unused_variables, unused_mut))]
pub fn use_mutation_observer_with_options<El, M, F>(
    target: El,
    mut callback: F,
    options: UseMutationObserverOptions,
) -> UseMutationObserverReturn<impl Fn() + Clone + Send + Sync>
where
    El: IntoElementsMaybeSignal<web_sys::Element, M>,
    F: FnMut(Vec<web_sys::MutationRecord>, web_sys::MutationObserver) + 'static,
{
    #[cfg(feature = "ssr")]
    {
        UseMutationObserverReturn {
            is_supported: Signal::derive(|| true),
            stop: || {},
        }
    }

    #[cfg(not(feature = "ssr"))]
    {
        use crate::js;
        use send_wrapper::SendWrapper;

        let closure_js = Closure::<dyn FnMut(js_sys::Array, web_sys::MutationObserver)>::new(
            move |entries: js_sys::Array, observer| {
                #[cfg(debug_assertions)]
                let _z = leptos::reactive::diagnostics::SpecialNonReactiveZone::enter();

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

        let is_supported = use_supported(|| js!("MutationObserver" in &window()));

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

        let targets = target.into_elements_maybe_signal();

        let stop_watch = {
            let cleanup = cleanup.clone();

            let stop = Effect::watch(
                move || targets.get(),
                move |targets, _, _| {
                    cleanup();

                    if is_supported.get_untracked() && !targets.is_empty() {
                        let obs =
                            web_sys::MutationObserver::new(closure_js.as_ref().unchecked_ref())
                                .expect("failed to create MutationObserver");

                        for target in targets.iter().flatten() {
                            let target = target.clone();
                            let _ = obs.observe_with_options(&target, &options.clone().into());
                        }

                        observer.replace(Some(obs));
                    }
                },
                true,
            );

            move || stop.stop()
        };

        let stop = sendwrap_fn!(move || {
            cleanup();
            stop_watch();
        });

        on_cleanup({
            let stop = SendWrapper::new(stop.clone());
            #[allow(clippy::redundant_closure)]
            move || stop()
        });

        UseMutationObserverReturn { is_supported, stop }
    }
}
