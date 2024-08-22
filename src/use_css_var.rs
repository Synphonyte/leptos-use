#![cfg_attr(feature = "ssr", allow(unused_variables, unused_imports))]

use crate::core::ElementMaybeSignal;
use crate::{
    use_mutation_observer_with_options, watch_with_options, UseMutationObserverOptions,
    WatchOptions,
};
use cfg_if::cfg_if;
use default_struct_builder::DefaultBuilder;
use leptos::prelude::*;
use std::marker::PhantomData;
use std::time::Duration;
use wasm_bindgen::JsCast;

/// Manipulate CSS variables.
///
/// ## Demo
///
/// [Link to Demo](https://github.com/Synphonyte/leptos-use/tree/main/examples/use_css_var)
///
/// ## Usage
///
/// ```
/// # use leptos::prelude::*;
/// # use leptos_use::use_css_var;
/// #
/// # #[component]
/// # fn Demo() -> impl IntoView {
/// let (color, set_color) = use_css_var("--color");
///
/// set_color.set("red".to_string());
/// #
/// # view! { }
/// # }
/// ```
///
/// The variable name itself can be a `Signal`.
///
/// ```
/// # use leptos::prelude::*;
/// # use leptos_use::use_css_var;
/// #
/// # #[component]
/// # fn Demo() -> impl IntoView {
/// let (key, set_key) = signal("--color".to_string());
/// let (color, set_color) = use_css_var(key);
/// #
/// # view! { }
/// # }
/// ```
///
/// You can specify the element that the variable is applied to as well as an initial value in case
/// the variable is not set yet. The option to listen for changes to the variable is also available.
///
/// ```
/// # use leptos::prelude::*;
/// # use leptos::html::Div;
/// # use leptos_use::{use_css_var_with_options, UseCssVarOptions};
/// #
/// # #[component]
/// # fn Demo() -> impl IntoView {
/// let el = NodeRef::<Div>::new();
///
/// let (color, set_color) = use_css_var_with_options(
///     "--color",
///     UseCssVarOptions::default()
///         .target(el)
///         .initial_value("#eee")
///         .observe(true),
/// );
///
/// view! {
///     <div node_ref=el>"..."</div>
/// }
/// # }
/// ```
///
/// ## Server-Side Rendering
///
/// On the server this simply returns `signal(options.initial_value)`.
pub fn use_css_var(
    prop: impl Into<MaybeSignal<String>>,
) -> (ReadSignal<String>, WriteSignal<String>) {
    use_css_var_with_options(prop, UseCssVarOptions::default())
}

/// Version of [`use_css_var`] that takes a `UseCssVarOptions`. See [`use_css_var`] for how to use.
pub fn use_css_var_with_options<P, El, T>(
    prop: P,
    options: UseCssVarOptions<El, T>,
) -> (ReadSignal<String>, WriteSignal<String>)
where
    P: Into<MaybeSignal<String>>,
    El: Clone,
    El: Into<ElementMaybeSignal<T, web_sys::Element>>,
    T: Into<web_sys::Element> + Clone + 'static,
{
    let UseCssVarOptions {
        target,
        initial_value,
        observe,
        ..
    } = options;

    let (variable, set_variable) = signal(initial_value.clone());

    cfg_if! { if #[cfg(not(feature = "ssr"))] {
        let el_signal = target.into();
        let prop = prop.into();

        let update_css_var = {
            let prop = prop.clone();
            let el_signal = el_signal.clone();

            move || {
                let key = prop.get_untracked();

                if let Some(el) = el_signal.get_untracked() {
                    if let Ok(Some(style)) = window().get_computed_style(&el.into()) {
                        if let Ok(value) = style.get_property_value(&key) {
                            set_variable.update(|var| *var = value.trim().to_string());
                            return;
                        }
                    }

                    let initial_value = initial_value.clone();
                    set_variable.update(|var| *var = initial_value);
                }
            }
        };

        if observe {
            let update_css_var = update_css_var.clone();
            let el_signal = el_signal.clone();

            use_mutation_observer_with_options::<ElementMaybeSignal<T, web_sys::Element>, T, _>(
                                el_signal,
                move |_, _| update_css_var(),
                UseMutationObserverOptions::default()
                    .attribute_filter(vec!["style".to_string()]),
            );
        }

        // To get around style attributes on node_refs that are not applied after the first render
        set_timeout(update_css_var.clone(), Duration::ZERO);

        {
            let el_signal = el_signal.clone();
            let prop = prop.clone();

            let _ = watch_with_options(
                                move || (el_signal.get(), prop.get()),
                move |_, _, _| update_css_var(),
                WatchOptions::default().immediate(true),
            );
        }

        let _ = watch(
                        move || variable.get(),
            move |val, _, _| {
                if let Some(el) = el_signal.get() {
                    let el = el.into().unchecked_into::<web_sys::HtmlElement>();
                    let style = el.style();
                    let _ = style.set_property(&prop.get_untracked(), val);
                }
            },
            false,
        );
    }}

    (variable, set_variable)
}

/// Options for [`use_css_var_with_options`].
#[derive(DefaultBuilder)]
pub struct UseCssVarOptions<El, T>
where
    El: Clone,
    El: Into<ElementMaybeSignal<T, web_sys::Element>>,
    T: Into<web_sys::Element> + Clone + 'static,
{
    /// The target element to read the variable from and set the variable on.
    /// Defaults to the `document.documentElement`.
    target: El,

    /// The initial value of the variable before it is read. Also the default value
    /// if the variable isn't defined on the target. Defaults to "".
    #[builder(into)]
    initial_value: String,

    /// If `true` use a `MutationObserver` to monitor variable changes. Defaults to `false`.
    observe: bool,

    #[builder(skip)]
    _marker: PhantomData<T>,
}

cfg_if! { if #[cfg(feature = "ssr")] {
    impl Default for UseCssVarOptions<Option<web_sys::Element>, web_sys::Element> {
        fn default() -> Self {
            Self {
                target: None,
                initial_value: "".into(),
                observe: false,
                _marker: PhantomData,
            }
        }
    }
} else {
    impl Default for UseCssVarOptions<web_sys::Element, web_sys::Element> {
        fn default() -> Self {
            Self {
                target: document().document_element().expect("No document element"),
                initial_value: "".into(),
                observe: false,
                _marker: PhantomData,
            }
        }
    }
}}
