use leptos::reactive::wrappers::read::Signal;

/// Return type of [`use_element_bounding`](crate::use_element_bounding).
pub struct UseElementBoundingReturn<F>
where
    F: Fn() + Clone + Send + Sync,
{
    /// Reactive version of [`BoudingClientRect.height`](https://developer.mozilla.org/en-US/docs/Web/API/DOMRectReadOnly/height)
    pub height: Signal<f64>,
    /// Reactive version of [`BoudingClientRect.width`](https://developer.mozilla.org/en-US/docs/Web/API/DOMRectReadOnly/width)
    pub width: Signal<f64>,
    /// Reactive version of [`BoudingClientRect.left`](https://developer.mozilla.org/en-US/docs/Web/API/DOMRectReadOnly/left)
    pub left: Signal<f64>,
    /// Reactive version of [`BoudingClientRect.right`](https://developer.mozilla.org/en-US/docs/Web/API/DOMRectReadOnly/right)
    pub right: Signal<f64>,
    /// Reactive version of [`BoudingClientRect.top`](https://developer.mozilla.org/en-US/docs/Web/API/DOMRectReadOnly/top)
    pub top: Signal<f64>,
    /// Reactive version of [`BoudingClientRect.bottom`](https://developer.mozilla.org/en-US/docs/Web/API/DOMRectReadOnly/bottom)
    pub bottom: Signal<f64>,
    /// Reactive version of [`BoudingClientRect.x`](https://developer.mozilla.org/en-US/docs/Web/API/DOMRectReadOnly/x)
    pub x: Signal<f64>,
    /// Reactive version of [`BoudingClientRect.y`](https://developer.mozilla.org/en-US/docs/Web/API/DOMRectReadOnly/y)
    pub y: Signal<f64>,
    /// Function to re-evaluate `get_bounding_client_rect()` and update the signals.
    pub update: F,
}
