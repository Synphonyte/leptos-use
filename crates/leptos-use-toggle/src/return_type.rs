use leptos::prelude::*;

/// Return type of [`fn@crate::use_toggle`].
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
