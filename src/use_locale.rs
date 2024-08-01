use crate::{use_locales_with_options, UseLocalesOptions};
use leptos::*;

/// Reactive locale matching.
///
/// Returns the first matching locale given by [`fn@crate::use_locales`] that is also found in
/// the `supported` list. In case there is no match, then the first locale in `supported` will be
/// returned. If `supported` is empty, the empty string is returned.
///
/// Matching is done by checking if an accepted locale from `use_locales` starts with a supported
/// locale. If a match is found the locale from the `supported` list is returned.
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
/// let locale = use_locale(["en", "de", "fr"]);
/// #
/// # view! { }
/// # }
/// ```
///
/// ## Server-Side Rendering
///
/// See [`fn@crate::use_locales`]
pub fn use_locale<S>(supported: S) -> Signal<String>
where
    S: IntoIterator,
    S::Item: Into<String> + Clone + 'static,
{
    use_locale_with_options(supported, UseLocaleOptions::default())
}

/// Version of [`fn@crate::use_locale`] that takes a `UseLocaleOptions`. See [`fn@crate::use_locale`] for how to use.
pub fn use_locale_with_options<S>(supported: S, options: UseLocaleOptions) -> Signal<String>
where
    S: IntoIterator,
    S::Item: Into<String> + Clone + 'static,
{
    let locales = use_locales_with_options(options);

    let supported = supported.into_iter().collect::<Vec<_>>();

    Signal::derive(move || {
        let supported = supported.clone();

        locales.with(|locales| {
            let mut first_supported = None;

            for s in supported {
                let s = s.into();

                if first_supported.is_none() {
                    first_supported = Some(s.clone());
                }

                for locale in locales {
                    if locale.starts_with(&s) {
                        return s;
                    }
                }
            }

            first_supported.unwrap_or_else(|| "".to_string())
        })
    })
}

pub type UseLocaleOptions = UseLocalesOptions;
