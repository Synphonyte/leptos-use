use crate::utils::get_header;
use default_struct_builder::DefaultBuilder;
use leptos::prelude::*;
use std::sync::Arc;

/// Reactive locales.
///
/// If called on the client-side this function returns the value of
/// [`navigator.languages`](https://developer.mozilla.org/en-US/docs/Web/API/Navigator/languages)
/// and listens for changes to that property.
///
/// See "Server-Side Rendering" below.
///
/// ## Demo
///
/// [Link to Demo](https://github.com/Synphonyte/leptos-use/tree/main/examples/use_locales)
///
/// ## Usage
///
/// ```
/// # use leptos::*;
/// # use leptos_use::use_locales;
/// #
/// # #[component]
/// # fn Demo() -> impl IntoView {
/// let locales = use_locales();
/// #
/// # view! { }
/// # }
/// ```
///
/// ## Server-Side Rendering
///
/// On the server this returns the parsed value of the `accept-language` header.
///
/// > If you're using `axum` you have to enable the `"axum"` feature in your Cargo.toml.
/// > In case it's `actix-web` enable the feature `"actix"`, for `spin` enable `"spin"`.
///
/// ### Bring your own header
///
/// In case you're neither using Axum, Actix nor Spin, or the default implementation is not to your liking,
/// you can provide your own way of reading the language header value using the option
/// [`crate::UseLocalesOptions::ssr_lang_header_getter`].
pub fn use_locales() -> Signal<Vec<String>> {
    use_locales_with_options(UseLocalesOptions::default())
}

/// Version of [`fn@crate::use_locales`] that takes a `UseLocalesOptions`. See [`fn@crate::use_locales`] for how to use.
pub fn use_locales_with_options(options: UseLocalesOptions) -> Signal<Vec<String>> {
    #[cfg(not(feature = "ssr"))]
    {
        let _ = options;

        let read_navigator_languages = || {
            let window = crate::use_window();
            let navigator = window.navigator();
            navigator
                .map(|navigator| navigator.languages().to_vec())
                .unwrap_or_default()
                .into_iter()
                .filter_map(|x| x.as_string())
                .collect::<Vec<String>>()
        };

        let (locales, set_locales) = signal(read_navigator_languages());

        let _ =
            crate::use_event_listener(crate::use_window(), leptos::ev::languagechange, move |_| {
                set_locales.update(|locales| *locales = read_navigator_languages());
            });

        locales.into()
    }

    #[cfg(feature = "ssr")]
    {
        let UseLocalesOptions {
            ssr_lang_header_getter,
        } = options;

        let accept_language = ssr_lang_header_getter().unwrap_or_default();

        let locales = accept_language
            .split(',')
            .map(|locale| {
                locale
                    .split_once(';')
                    .map(|x| x.0)
                    .unwrap_or(locale)
                    .to_owned()
            })
            .collect::<Vec<_>>();

        Signal::derive(move || locales.clone())
    }
}

/// Options for [`fn@crate::use_locales_with_options`].
#[derive(DefaultBuilder)]
pub struct UseLocalesOptions {
    /// Getter function to return the string value of the accept languange header.
    /// When you use one of the features `"axum"`, `"actix"` or `"spin"` there's a valid default implementation provided.
    #[allow(dead_code)]
    ssr_lang_header_getter: Arc<dyn Fn() -> Option<String>>,
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
