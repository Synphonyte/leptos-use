use crate::Position;
use leptos::prelude::*;
use leptos::reactive::wrappers::read::Signal;

/// Return type of [`use_draggable`](crate::use_draggable).
pub struct UseDraggableReturn {
    /// X coordinate of the element
    pub x: Signal<f64>,
    /// Y coordinate of the element
    pub y: Signal<f64>,
    /// Position of the element
    pub position: Signal<Position>,
    /// Set the position of the element manually
    pub set_position: WriteSignal<Position>,
    /// Whether the element is being dragged
    pub is_dragging: Signal<bool>,
    /// Style attribute "left: {x}px; top: {y}px;"
    pub style: Signal<String>,
}
