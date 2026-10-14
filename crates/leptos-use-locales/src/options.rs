use crate::get_header;
use default_struct_builder::DefaultBuilder;
use std::sync::Arc;

/// Options for [`fn@crate::use_locales_with_options`].
#[derive(DefaultBuilder)]
pub struct UseLocalesOptions {
    /// Getter function to return the string value of the accept languange header.
    /// When you use one of the features `"axum"` or `"actix"` there's a valid default implementation provided.
    #[allow(dead_code)]
    pub(crate) ssr_lang_header_getter: Arc<dyn Fn() -> Option<String>>,
}

impl Default for UseLocalesOptions {
    fn default() -> Self {
        Self {
            ssr_lang_header_getter: Arc::new(move || {
                get_header!(ACCEPT_LANGUAGE, use_locales, ssr_lang_header_getter)
            }),
        }
    }
}
