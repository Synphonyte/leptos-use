use crate::{use_locales_with_options, UseLocalesOptions};
use leptos::{logging::warn, prelude::*};
use unic_langid::LanguageIdentifier;

/// Reactive locale matching.
///
/// Returns the first matching locale given by [`fn@crate::use_locales`] that is also found in
/// the `supported` list. In case there is no match, then the first locale in `supported` will be
/// returned.
///
/// > If `supported` is empty, this function will panic!
///
/// Matching is done by using the [`fn@unic_langid::LanguageIdentifier::matches`] method.
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
/// use unic_langid::langid_slice;
/// #
/// # #[component]
/// # fn Demo() -> impl IntoView {
/// let locale = use_locale(langid_slice!["en", "de", "fr"]);
/// #
/// # view! { }
/// # }
/// ```
///
/// ## Server-Side Rendering
///
/// See [`fn@crate::use_locales`]
pub fn use_locale<S>(supported: S) -> Signal<LanguageIdentifier>
where
    S: IntoIterator,
    S::Item: AsRef<LanguageIdentifier>,
{
    use_locale_with_options(supported, UseLocaleOptions::default())
}

/// Version of [`fn@crate::use_locale`] that takes a `UseLocaleOptions`. See [`fn@crate::use_locale`] for how to use.
pub fn use_locale_with_options<S>(
    supported: S,
    options: UseLocaleOptions,
) -> Signal<LanguageIdentifier>
where
    S: IntoIterator,
    S::Item: AsRef<LanguageIdentifier>,
{
    let client_locales = use_locales_with_options(options);

    let supported = supported
        .into_iter()
        .map(|l| l.as_ref().clone())
        .collect::<Vec<_>>();

    const EMPTY_ERR_MSG: &str = "Empty supported list. You have to provide at least one locale in the `supported` parameter";

    assert!(!supported.is_empty(), "{}", EMPTY_ERR_MSG);

    Signal::derive(move || {
        client_locales.with(|client_locales| {
            let mut supported_iter = supported.iter().peekable();

            // Checked it's not empty above.
            let first_supported = *supported_iter.peek().unwrap();

            for s in supported_iter {
                for client_locale in client_locales {
                    let client_locale = client_locale.parse::<LanguageIdentifier>();

                    if let Ok(client_locale) = client_locale {
                        if client_locale.matches(s, true, true) {
                            return (*s).clone();
                        }
                    } else {
                        warn!("Received an invalid LanguageIdentifier")
                    }
                }
            }

            (*first_supported).clone()
        })
    })
}

pub type UseLocaleOptions = UseLocalesOptions;
