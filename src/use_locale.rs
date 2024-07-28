use crate::utils::get_header;
use crate::{use_locales_with_options, UseLocalesOptions};
use default_struct_builder::DefaultBuilder;
use leptos::*;
use std::rc::Rc;

/// Reactive locale.
///
/// This is basically the same as [`fn@crate::use_locales`] but takes the first entry of the list.
/// If no locale is found then [`crate::UseLocaleOptions::fallback`] is returned which defaults
/// to `"en"`.
///
/// ## Demo
///
/// [Link to Demo](https://github.com/Synphonyte/leptos-use/tree/main/examples/use_locale)
///
/// ## Usage
///
/// ```
/// # use leptos::*;
/// # use leptos_use::use_locale;
/// #
/// # #[component]
/// # fn Demo() -> impl IntoView {
/// let locale = use_locale();
/// #
/// # view! { }
/// # }
/// ```
///
/// ## Server-Side Rendering
///
/// See [`fn@crate::use_locales`]

pub fn use_locale() -> Signal<String> {
    use_locale_with_options(UseLocaleOptions::default())
}

/// Version of [`fn@crate::use_locale`] that takes a `UseLocaleOptions`. See [`fn@crate::use_locale`] for how to use.
pub fn use_locale_with_options(options: UseLocaleOptions) -> Signal<String> {
    let UseLocaleOptions {
        fallback,
        ssr_lang_header_getter,
    } = options;

    let locales = use_locales_with_options(UseLocalesOptions {
        ssr_lang_header_getter,
    });

    Signal::derive(move || locales.get().first().cloned().unwrap_or(fallback.clone()))
}

/// Options for [`fn@crate::use_locale_with_options`].
#[derive(DefaultBuilder)]
pub struct UseLocaleOptions {
    /// Fallback value in case no locale is found. Defaults to `"en"`.
    fallback: String,

    /// Getter function to return the string value of the accept languange header.
    /// When you use one of the features `"axum"`, `"actix"` or `"spin"` there's a valid default implementation provided.
    #[allow(dead_code)]
    ssr_lang_header_getter: Rc<dyn Fn() -> Option<String>>,
}

impl Default for UseLocaleOptions {
    fn default() -> Self {
        Self {
            fallback: "en".to_string(),
            ssr_lang_header_getter: Rc::new(move || {
                get_header!(ACCEPT_LANGUAGE, use_locale, ssr_lang_header_getter)
            }),
        }
    }
}
