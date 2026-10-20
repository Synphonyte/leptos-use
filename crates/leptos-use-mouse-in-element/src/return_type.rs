use crate::UseMouseSourceType;
use leptos::prelude::*;

/// Return type of [`use_mouse_in_element`](crate::use_mouse_in_element).
pub struct UseMouseInElementReturn<F>
where
    F: Fn() + Clone + Send + Sync,
{
    /// X coordinate of the mouse pointer / touch
    pub x: Signal<f64>,

    /// Y coordinate of the mouse pointer / touch
    pub y: Signal<f64>,

    /// Identifies the source of the reported coordinates
    pub source_type: Signal<UseMouseSourceType>,

    /// X coordinate of the pointer relative to the left edge of the element
    pub element_x: Signal<f64>,

    /// Y coordinate of the pointer relative to the top edge of the element
    pub element_y: Signal<f64>,

    /// X coordinate of the element relative to the left edge of the document
    pub element_position_x: Signal<f64>,

    /// Y coordinate of the element relative to the top edge of the document
    pub element_position_y: Signal<f64>,

    /// Width of the element
    pub element_width: Signal<f64>,

    /// Height of the element
    pub element_height: Signal<f64>,

    /// `true` if the mouse is outside of the element
    pub is_outside: Signal<bool>,

    /// Stop watching
    pub stop: F,
}
