use crate::use_media_query;
use leptos::*;

/// Reactive [dark theme preference](https://developer.mozilla.org/en-US/docs/Web/CSS/@media/prefers-color-scheme).
///
/// ## Usage
///
/// ```
/// # use leptos::*;
/// # use leptos_use::use_preferred_dark;
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
/// On the server this functions returns a Signal that is always `false`.
///
/// ## See also
///
/// * [`fn@crate::use_media_query`]
/// * [`fn@crate::use_preferred_contrast`]
pub fn use_preferred_dark() -> Signal<bool> {
    use_media_query("(prefers-color-scheme: dark)")
}
