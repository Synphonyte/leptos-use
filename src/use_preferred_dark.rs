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
/// # fn Demo(cx: Scope) -> impl IntoView {
/// #
/// let is_dark_preferred = use_preferred_dark(cx);
/// #
/// #    view! { cx, }
/// # }
/// ```
///
/// ## See also
///
/// * [`use_media_query`]
/// * [`use_preferred_contrast`]
pub fn use_preferred_dark(cx: Scope) -> Signal<bool> {
    use_media_query(cx, "(prefers-color-scheme: dark)")
}
