use crate::use_media_query;
use leptos::*;
use std::fmt::Display;

/// Reactive [prefers-contrast](https://developer.mozilla.org/en-US/docs/Web/CSS/@media/prefers-contrast) media query.
///
/// ## Usage
///
/// ```
/// # use leptos::*;
/// # use leptos_use::use_preferred_contrast;
/// #
/// # #[component]
/// # fn Demo(cx: Scope) -> impl IntoView {
/// #
/// let preferred_contrast = use_preferred_contrast(cx);
/// #
/// #    view! { cx, }
/// # }
/// ```
///
/// ## See also
///
/// * [`use_media_query`]
/// * [`use_preferred_dark`]
pub fn use_preferred_contrast(cx: Scope) -> Signal<PreferredContrast> {
    let is_more = use_media_query(cx, "(prefers-contrast: more)");
    let is_less = use_media_query(cx, "(prefers-contrast: less)");
    let is_custom = use_media_query(cx, "(prefers-contrast: custom)");

    Signal::derive(cx, move || {
        if is_more.get() {
            PreferredContrast::More
        } else if is_less.get() {
            PreferredContrast::Less
        } else if is_custom.get() {
            PreferredContrast::Custom
        } else {
            PreferredContrast::NoPreference
        }
    })
}

/// Return value for [`use_preferred_contrast`]
#[derive(Debug, Clone, Copy, PartialEq, Eq, Default)]
pub enum PreferredContrast {
    More,
    Less,
    Custom,
    #[default]
    NoPreference,
}

impl Display for PreferredContrast {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            PreferredContrast::More => write!(f, "more"),
            PreferredContrast::Less => write!(f, "less"),
            PreferredContrast::Custom => write!(f, "custom"),
            PreferredContrast::NoPreference => write!(f, "no-preference"),
        }
    }
}
