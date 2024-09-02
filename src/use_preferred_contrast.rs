use crate::use_media_query;
use leptos::prelude::*;
use leptos::reactive_graph::wrappers::read::Signal;
use std::fmt::Display;

/// Reactive [prefers-contrast](https://developer.mozilla.org/en-US/docs/Web/CSS/@media/prefers-contrast) media query.
///
/// ## Usage
///
/// ```
/// # use leptos::prelude::*;
/// # use leptos_use::use_preferred_contrast;
/// #
/// # #[component]
/// # fn Demo() -> impl IntoView {
/// #
/// let preferred_contrast = use_preferred_contrast();
/// #
/// #    view! { }
/// # }
/// ```
///
/// ## Server-Side Rendering
///
/// On the server this returns a `Signal` that always contains the value `PreferredContrast::NoPreference`.
///
/// ## See also
///
/// * [`fn@crate::use_media_query`]
/// * [`fn@crate::use_preferred_dark`]
/// * [`fn@crate::use_prefers_reduced_motion`]
pub fn use_preferred_contrast() -> Signal<PreferredContrast> {
    let is_more = use_media_query("(prefers-contrast: more)");
    let is_less = use_media_query("(prefers-contrast: less)");
    let is_custom = use_media_query("(prefers-contrast: custom)");

    Signal::derive(move || {
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
