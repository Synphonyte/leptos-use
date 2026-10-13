use crate::{UseLocalesOptions, use_locales_with_options};
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
/// Matching is done by using the [`unic_langid::LanguageIdentifier::matches`](https://docs.rs/unic-langid/latest/unic_langid/struct.LanguageIdentifier.html#method.matches) method.
///
/// ## Demo
///
/// [Link to Demo](https://github.com/Synphonyte/leptos-use/tree/main/examples/use_locale)
///
/// ## Usage
///
/// ```
/// # use leptos::*;
/// # use leptos_use_locale::use_locale;
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
/// > Make sure you follow the [instructions in Server-Side Rendering](https://leptos-use.rs/server_side_rendering.html).
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
            for client_locale in client_locales {
                let Ok(client_locale) = client_locale.parse::<LanguageIdentifier>() else {
                    warn!("Received an invalid LanguageIdentifier");
                    continue;
                };

                if let Some(s) = supported.iter().find(|s| **s == client_locale) {
                    return s.clone();
                }

                if let Some(s) = supported
                    .iter()
                    .find(|s| client_locale.matches(*s, true, true))
                {
                    return s.clone();
                }
            }

            // Checked it's not empty above.
            supported.first().expect(EMPTY_ERR_MSG).clone()
        })
    })
}

pub type UseLocaleOptions = UseLocalesOptions;
