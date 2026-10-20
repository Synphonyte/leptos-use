use leptos::prelude::*;

/// Return type of [`use_mouse`](crate::use_mouse).
pub struct UseMouseReturn {
    /// X coordinate of the mouse pointer / touch
    pub x: Signal<f64>,
    /// Y coordinate of the mouse pointer / touch
    pub y: Signal<f64>,
    /// Sets the value of `x`. This does not actually move the mouse cursor.
    pub set_x: WriteSignal<f64>,
    /// Sets the value of `y`. This does not actually move the mouse cursor.
    pub set_y: WriteSignal<f64>,
    /// Identifies the source of the reported coordinates
    pub source_type: Signal<UseMouseSourceType>,
}

/// Identifies the source of the reported coordinates
#[derive(Copy, Clone, Debug, PartialEq, Eq)]
pub enum UseMouseSourceType {
    /// coordinates come from mouse movement
    Mouse,
    /// coordinates come from touch
    Touch,
    /// Initially before any event has been recorded the source type is unset
    Unset,
}
