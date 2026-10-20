use crate::get_header;
use default_struct_builder::DefaultBuilder;
use std::sync::Arc;

/// Options for [`fn@crate::use_preferred_dark_with_options`].
#[derive(DefaultBuilder)]
pub struct UsePreferredDarkOptions {
    /// Getter function to return the string value of the
    /// [`Sec-CH-Prefers-Color-Scheme`](https://developer.mozilla.org/en-US/docs/Web/HTTP/Headers/Sec-CH-Prefers-Color-Scheme)
    /// header.
    /// When you use one of the features `"axum"` or `"actix"` there's a valid default
    /// implementation provided.
    #[allow(dead_code)]
    #[doc(hidden)]
    pub ssr_color_header_getter: Arc<dyn Fn() -> Option<String> + Send + Sync>,
}

impl Default for UsePreferredDarkOptions {
    fn default() -> Self {
        Self {
            ssr_color_header_getter: Arc::new(move || {
                get_header!(
                    HeaderName::from_static("sec-ch-prefers-color-scheme"),
                    use_preferred_dark,
                    ssr_color_header_getter
                )
            }),
        }
    }
}

/// Options for [`fn@crate::use_prefers_reduced_motion_with_options`].
#[derive(DefaultBuilder)]
pub struct UsePrefersReducedMotionOptions {
    /// Getter function to return the string value of the
    /// [`Sec-CH-Prefers-Reduced-Motion`](https://developer.mozilla.org/en-US/docs/Web/HTTP/Headers/Sec-CH-Prefers-Reduced-Motion)
    /// header.
    /// When you use one of the features `"axum"` or `"actix"` there's a valid default
    /// implementation provided.
    #[allow(dead_code)]
    pub(crate) ssr_motion_header_getter: Arc<dyn Fn() -> Option<String> + Send + Sync>,
}

impl Default for UsePrefersReducedMotionOptions {
    fn default() -> Self {
        Self {
            ssr_motion_header_getter: Arc::new(move || {
                get_header!(
                    HeaderName::from_static("sec-ch-prefers-reduced-motion"),
                    use_prefers_reduced_motion,
                    ssr_motion_header_getter
                )
            }),
        }
    }
}
