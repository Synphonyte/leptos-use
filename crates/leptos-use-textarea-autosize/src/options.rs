use crate::{ElementMaybeSignal, IntoElementMaybeSignal, MaybeRwSignal};
use default_struct_builder::DefaultBuilder;
use std::sync::Arc;

/// Options for [`fn@crate::use_textarea_autosize_with_options`].
#[derive(DefaultBuilder)]
#[cfg_attr(feature = "ssr", allow(dead_code))]
pub struct UseTextareaAutosizeOptions {
    /// Textarea content
    #[builder(into)]
    pub(crate) content: MaybeRwSignal<String>,

    /// Watch sources that should trigger a textarea resize
    pub(crate) watch: Arc<dyn Fn() + Send + Sync>,

    /// Function called when the textarea size changes
    pub(crate) on_resize: Arc<dyn Fn() + Send + Sync>,

    /// Specify style target to apply the height based on textarea content.
    /// If not provided it will use textarea it self.
    #[builder(skip)]
    pub(crate) style_target: ElementMaybeSignal<web_sys::Element>,

    /// Specify the style property that will be used to manipulate height.
    /// Should be `"height"` or `"min-height"`. Default value is `"height"`.
    #[builder(into)]
    pub(crate) style_prop: String,
}

impl Default for UseTextareaAutosizeOptions {
    fn default() -> Self {
        Self {
            content: MaybeRwSignal::default(),
            watch: Arc::new(|| ()),
            on_resize: Arc::new(|| ()),
            style_target: Default::default(),
            style_prop: "height".to_string(),
        }
    }
}

impl UseTextareaAutosizeOptions {
    /// List of elementss that should not trigger the callback. Defaults to `[]`.
    #[cfg_attr(feature = "ssr", allow(dead_code))]
    pub fn style_target<M>(
        self,
        style_target: impl IntoElementMaybeSignal<web_sys::Element, M>,
    ) -> Self {
        Self {
            style_target: style_target.into_element_maybe_signal(),
            ..self
        }
    }
}
