use crate::IntoElementMaybeSignal;
use default_struct_builder::DefaultBuilder;
#[cfg(not(feature = "ssr"))]
use leptos::prelude::*;
use std::marker::PhantomData;

/// Options for [`use_css_var_with_options`](crate::use_css_var_with_options).
#[derive(DefaultBuilder)]
pub struct UseCssVarOptions<El, M>
where
    El: IntoElementMaybeSignal<web_sys::Element, M>,
{
    /// The target element to read the variable from and set the variable on.
    /// Defaults to the `document.documentElement`.
    pub(crate) target: El,

    /// The initial value of the variable before it is read. Also the default value
    /// if the variable isn't defined on the target. Defaults to "".
    #[builder(into)]
    pub(crate) initial_value: String,

    /// If `true` use a `MutationObserver` to monitor variable changes. Defaults to `false`.
    pub(crate) observe: bool,

    #[builder(skip)]
    _marker: PhantomData<M>,
}

#[cfg(feature = "ssr")]
impl<M> Default for UseCssVarOptions<Option<web_sys::Element>, M>
where
    Option<web_sys::Element>: IntoElementMaybeSignal<web_sys::Element, M>,
{
    fn default() -> Self {
        Self {
            target: None,
            initial_value: "".into(),
            observe: false,
            _marker: PhantomData,
        }
    }
}

#[cfg(not(feature = "ssr"))]
impl<M> Default for UseCssVarOptions<web_sys::Element, M>
where
    web_sys::Element: IntoElementMaybeSignal<web_sys::Element, M>,
{
    fn default() -> Self {
        Self {
            target: document().document_element().expect("No document element"),
            initial_value: "".into(),
            observe: false,
            _marker: PhantomData,
        }
    }
}
