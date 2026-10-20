use crate::IntoElementMaybeSignal;
use default_struct_builder::DefaultBuilder;
use std::marker::PhantomData;

/// Options for [`use_element_visibility_with_options`](crate::use_element_visibility_with_options).
#[derive(DefaultBuilder)]
pub struct UseElementVisibilityOptions<El, M>
where
    El: IntoElementMaybeSignal<web_sys::Element, M>,
{
    /// A `web_sys::Element` or `web_sys::Document` object which is an ancestor of the intended `target`,
    /// whose bounding rectangle will be considered the viewport.
    /// Any part of the target not visible in the visible area of the `root` is not considered visible.
    /// Defaults to `None` (which means the root `document` will be used).
    /// Please note that setting this to a `Some(document)` may not be supported by all browsers.
    /// See [Browser Compatibility](https://developer.mozilla.org/en-US/docs/Web/API/IntersectionObserver/IntersectionObserver#browser_compatibility)
    #[cfg_attr(feature = "ssr", allow(dead_code))]
    pub(crate) viewport: Option<El>,

    #[builder(skip)]
    _marker: PhantomData<M>,
}

impl<M> Default for UseElementVisibilityOptions<web_sys::Element, M>
where
    web_sys::Element: IntoElementMaybeSignal<web_sys::Element, M>,
{
    fn default() -> Self {
        Self {
            viewport: None,
            _marker: PhantomData,
        }
    }
}
