use crate::{ElementsMaybeSignal, IntoElementsMaybeSignal};
use default_struct_builder::DefaultBuilder;

/// Options for [`on_click_outside_with_options`](crate::on_click_outside_with_options).
#[derive(Clone, DefaultBuilder)]
#[cfg_attr(feature = "ssr", allow(dead_code))]
pub struct OnClickOutsideOptions {
    /// List of elementss that should not trigger the callback. Defaults to `[]`.
    #[builder(skip)]
    pub(crate) ignore: ElementsMaybeSignal<web_sys::EventTarget>,

    /// Use capturing phase for internal event listener. Defaults to `true`.
    pub(crate) capture: bool,

    /// Run callback if focus moves to an iframe. Defaults to `false`.
    pub(crate) detect_iframes: bool,
}

impl Default for OnClickOutsideOptions {
    fn default() -> Self {
        Self {
            ignore: Vec::<web_sys::EventTarget>::new().into_elements_maybe_signal(),
            capture: true,
            detect_iframes: false,
        }
    }
}

impl OnClickOutsideOptions {
    /// List of elements that should not trigger the callback. Defaults to `[]`.
    #[cfg_attr(feature = "ssr", allow(dead_code))]
    pub fn ignore<M>(self, ignore: impl IntoElementsMaybeSignal<web_sys::EventTarget, M>) -> Self {
        Self {
            ignore: ignore.into_elements_maybe_signal(),
            ..self
        }
    }
}
