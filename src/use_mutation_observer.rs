use crate::core::IntoElementsMaybeSignal;
use cfg_if::cfg_if;
use default_struct_builder::DefaultBuilder;
use leptos::reactive::wrappers::read::Signal;
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
/// # use leptos_use::{use_mutation_observer_with_options, UseMutationObserverOptions};
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

                    if is_supported.get() && !targets.is_empty() {
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
                false,
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

/// Options for [`use_mutation_observer_with_options`].
#[derive(DefaultBuilder, Clone, Default)]
pub struct UseMutationObserverOptions {
    /// Set to `true` to extend monitoring to the entire subtree of nodes rooted at `target`.
    /// All of the other properties are then extended to all of the nodes in the subtree
    /// instead of applying solely to the `target` node. The default value is `false`.
    subtree: bool,

    /// Set to `true` to monitor the target node (and, if `subtree` is `true`, its descendants)
    /// for the addition of new child nodes or removal of existing child nodes.
    /// The default value is `false`.
    child_list: bool,

    /// Set to `true` to watch for changes to the value of attributes on the node or nodes being
    /// monitored. The default value is `true` if either of `attribute_filter` or
    /// `attribute_old_value` is specified, otherwise the default value is `false`.
    attributes: bool,

    /// An array of specific attribute names to be monitored. If this property isn't included,
    /// changes to all attributes cause mutation notifications.
    #[builder(into)]
    attribute_filter: Option<Vec<String>>,

    /// Set to `true` to record the previous value of any attribute that changes when monitoring
    /// the node or nodes for attribute changes; See
    /// [Monitoring attribute values](https://developer.mozilla.org/en-US/docs/Web/API/MutationObserver/observe#monitoring_attribute_values)
    /// for an example of watching for attribute changes and recording values.
    /// The default value is `false`.
    attribute_old_value: bool,

    /// Set to `true` to monitor the specified target node
    /// (and, if `subtree` is `true`, its descendants)
    /// for changes to the character data contained within the node or nodes.
    /// The default value is `true` if `character_data_old_value` is specified,
    /// otherwise the default value is `false`.
    #[builder(into)]
    character_data: Option<bool>,

    /// Set to `true` to record the previous value of a node's text whenever the text changes on
    /// nodes being monitored. The default value is `false`.
    character_data_old_value: bool,
}

impl From<UseMutationObserverOptions> for web_sys::MutationObserverInit {
    fn from(val: UseMutationObserverOptions) -> Self {
        let UseMutationObserverOptions {
            subtree,
            child_list,
            attributes,
            attribute_filter,
            attribute_old_value,
            character_data,
            character_data_old_value,
        } = val;

        let init = Self::new();

        init.set_subtree(subtree);
        init.set_child_list(child_list);
        init.set_attributes(attributes);
        init.set_attribute_old_value(attribute_old_value);
        init.set_character_data_old_value(character_data_old_value);

        if let Some(attribute_filter) = attribute_filter {
            let array = js_sys::Array::from_iter(attribute_filter.into_iter().map(JsValue::from));
            init.set_attribute_filter(array.unchecked_ref());
        }
        if let Some(character_data) = character_data {
            init.set_character_data(character_data);
        }

        init
    }
}

/// The return value of [`use_mutation_observer`].
pub struct UseMutationObserverReturn<F: Fn() + Clone + Send + Sync> {
    /// Whether the browser supports the MutationObserver API
    pub is_supported: Signal<bool>,
    /// A function to stop and detach the MutationObserver
    pub stop: F,
}
