use crate::core::MaybeRwSignal;
use leptos::prelude::*;

/// A boolean switcher with utility functions.
///
/// ## Demo
///
/// [Link to Demo](https://github.com/Synphonyte/leptos-use/tree/main/examples/use_toggle)
///
/// ## Usage
///
/// ```
/// # use leptos::*;
/// # use leptos_use::{use_toggle, UseToggleReturn};
/// #
/// # #[component]
/// # fn Demo() -> impl IntoView {
/// let UseToggleReturn { toggle, value, set_value } = use_toggle(true);
/// #
/// # view! { }
/// # }
/// ```
///
/// ## See also
///
/// * [`fn@crate::use_cycle_list`]
// #[doc(cfg(feature = "use_toggle"))]
pub fn use_toggle(
    initial_value: impl Into<MaybeRwSignal<bool>>,
) -> UseToggleReturn<impl Fn() + Clone + Send + Sync + 'static> {
    let initial_value = initial_value.into();
    let (value, set_value) = initial_value.into_signal();

    let toggle = move || {
        set_value.update(|v| *v = !*v);
    };

    UseToggleReturn {
        toggle,
        value,
        set_value,
    }
}

/// Return type of [`fn@crate::use_toggle`].
// #[doc(cfg(feature = "use_toggle"))]
pub struct UseToggleReturn<F>
where
    F: Fn() + Clone + Send + Sync + 'static,
{
    /// Toggles the value between `true` and `false`.
    pub toggle: F,
    /// The current value as signal.
    pub value: Signal<bool>,
    /// Sets the current value to the given value.
    pub set_value: WriteSignal<bool>,
}
