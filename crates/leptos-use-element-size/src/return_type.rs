use leptos::reactive::wrappers::read::Signal;

/// The return value of [`use_element_size`](crate::use_element_size).
pub struct UseElementSizeReturn {
    /// The width of the element.
    pub width: Signal<f64>,
    /// The height of the element.
    pub height: Signal<f64>,
}
