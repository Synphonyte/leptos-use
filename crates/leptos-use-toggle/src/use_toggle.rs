use crate::{MaybeRwSignal, UseToggleReturn};
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
/// # use leptos_use_toggle::{use_toggle, UseToggleReturn};
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
