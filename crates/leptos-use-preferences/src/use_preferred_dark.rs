use crate::UsePreferredDarkOptions;
use leptos::prelude::*;

/// Reactive [dark theme preference](https://developer.mozilla.org/en-US/docs/Web/CSS/@media/prefers-color-scheme).
///
/// ## Usage
///
/// ```
/// # use leptos::prelude::*;
/// # use leptos_use_preferences::use_preferred_dark;
/// #
/// # #[component]
/// # fn Demo() -> impl IntoView {
/// #
/// let is_dark_preferred = use_preferred_dark();
/// #
/// #    view! { }
/// # }
/// ```
///
/// ## Server-Side Rendering
///
/// > Make sure you follow the [instructions in Server-Side Rendering](https://leptos-use.rs/server_side_rendering.html).
///
/// On the server this will try to read the
/// [`Sec-CH-Prefers-Color-Scheme` header](https://developer.mozilla.org/en-US/docs/Web/HTTP/Headers/Sec-CH-Prefers-Color-Scheme)
/// to determine the color mode. If the header is not present it will return `ColorMode::Light`.
/// Please have a look at the linked documentation above for that header to see browser support
/// as well as potential server requirements.
///
/// > If you're using `axum` you have to enable the `"axum"` feature in your Cargo.toml.
/// > In case it's `actix-web` enable the feature `"actix"`.
///
/// ### Bring your own header
///
/// In case you're neither using Axum nor Actix or the default implementation is not to your liking,
/// you can provide your own way of reading the color scheme header value using the option
/// [`crate::UsePreferredDarkOptions::ssr_color_header_getter`].
///
/// ## See also
///
/// * [`fn@crate::use_media_query`]
/// * [`fn@crate::use_preferred_contrast`]
/// * [`fn@crate::use_prefers_reduced_motion`]
pub fn use_preferred_dark() -> Signal<bool> {
    use_preferred_dark_with_options(Default::default())
}

/// Version of [`fn@crate::use_preferred_dark`] that accepts a `UsePreferredDarkOptions`.
pub fn use_preferred_dark_with_options(options: UsePreferredDarkOptions) -> Signal<bool> {
    #[cfg(not(feature = "ssr"))]
    {
        let _ = options;

        crate::use_media_query("(prefers-color-scheme: dark)")
    }

    #[cfg(feature = "ssr")]
    {
        let color_header = (options.ssr_color_header_getter)();
        Signal::derive(move || color_header == Some("dark".to_string()))
    }
}
