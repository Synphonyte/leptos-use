use crate::core::ElementMaybeSignal;
use crate::{use_mutation_observer_with_options, watch, watch_with_options, WatchOptions};
use default_struct_builder::DefaultBuilder;
use leptos::*;
use std::marker::PhantomData;
use std::time::Duration;
use wasm_bindgen::{JsCast, JsValue};

/// Manipulate CSS variables.
///
/// ## Demo
///
/// [Link to Demo](https://github.com/Synphonyte/leptos-use/tree/main/examples/use_css_var)
///
/// ## Usage
///
/// ```
/// # use leptos::*;
/// # use leptos_use::use_css_var;
/// #
/// # #[component]
/// # fn Demo(cx: Scope) -> impl IntoView {
/// let (color, set_color) = use_css_var(cx, "--color");
///
/// set_color.set("red".to_string());
/// #
/// # view! { cx, }
/// # }
/// ```
///
/// The variable name itself can be a `Signal`.
///
/// ```
/// # use leptos::*;
/// # use leptos_use::use_css_var;
/// #
/// # #[component]
/// # fn Demo(cx: Scope) -> impl IntoView {
/// let (key, set_key) = create_signal(cx, "--color".to_string());
/// let (color, set_color) = use_css_var(cx, key);
/// #
/// # view! { cx, }
/// # }
/// ```
///
/// You can specify the element that the variable is applied to as well as an initial value in case
/// the variable is not set yet. The option to listen for changes to the variable is also available.
///
/// ```
/// # use leptos::*;
/// # use leptos_use::{use_css_var_with_options, UseCssVarOptions};
/// #
/// # #[component]
/// # fn Demo(cx: Scope) -> impl IntoView {
/// let el = create_node_ref(cx);
///
/// let (color, set_color) = use_css_var_with_options(
///     cx,
///     "--color",
///     UseCssVarOptions::default()
///         .target(el)
///         .initial_value("#eee")
///         .observe(true),
/// );
///
/// view! { cx,
///     <div node_ref=el>"..."</div>
/// }
/// # }
/// ```

pub fn use_css_var(
    cx: Scope,
    prop: impl Into<MaybeSignal<String>>,
) -> (ReadSignal<String>, WriteSignal<String>) {
    use_css_var_with_options(cx, prop, UseCssVarOptions::default())
}

/// Version of [`use_css_var`] that takes a `UseCssVarOptions`. See [`use_css_var`] for how to use.
pub fn use_css_var_with_options<P, El, T>(
    cx: Scope,
    prop: P,
    options: UseCssVarOptions<El, T>,
) -> (ReadSignal<String>, WriteSignal<String>)
where
    P: Into<MaybeSignal<String>>,
    El: Clone,
    (Scope, El): Into<ElementMaybeSignal<T, web_sys::Element>>,
    T: Into<web_sys::Element> + Clone + 'static,
{
    let UseCssVarOptions {
        target,
        initial_value,
        observe,
        ..
    } = options;

    let (variable, set_variable) = create_signal(cx, initial_value.clone());

    let el_signal = (cx, target).into();
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
        let mut init = web_sys::MutationObserverInit::new();
        let update_css_var = update_css_var.clone();
        let el_signal = el_signal.clone();

        init.attribute_filter(&js_sys::Array::from_iter(
            vec![JsValue::from_str("style")].into_iter(),
        ));
        use_mutation_observer_with_options::<ElementMaybeSignal<T, web_sys::Element>, T, _>(
            cx,
            el_signal,
            move |_, _| update_css_var(),
            init,
        );
    }

    // To get around style attributes on node_refs that are not applied after the first render
    set_timeout(update_css_var.clone(), Duration::ZERO);

    {
        let el_signal = el_signal.clone();
        let prop = prop.clone();

        let _ = watch_with_options(
            cx,
            move || (el_signal.get(), prop.get()),
            move |_, _, _| update_css_var(),
            WatchOptions::default().immediate(true),
        );
    }

    let _ = watch(cx, move || variable.get(), move |val, _, _| {
        if let Some(el) = el_signal.get() {
            let el = el.into().unchecked_into::<web_sys::HtmlElement>();
            let style = el.style();
            let _ = style.set_property(&prop.get_untracked(), val);
        }
    });

    (variable, set_variable)
}

/// Options for [`use_css_var_with_options`].
#[derive(DefaultBuilder)]
pub struct UseCssVarOptions<El, T>
where
    El: Clone,
    (Scope, El): Into<ElementMaybeSignal<T, web_sys::Element>>,
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
