use leptos::prelude::*;

/// Return type of [`fn@crate::use_window_size`].
pub struct UseWindowSizeReturn {
    /// The width of the window.
    pub width: Signal<f64>,
    /// The height of the window.
    pub height: Signal<f64>,
}
