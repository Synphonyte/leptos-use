use leptos::reactive::wrappers::read::Signal;

/// The return value of [`use_resize_observer`](crate::use_resize_observer).
pub struct UseResizeObserverReturn<F: Fn() + Clone + Send + Sync> {
    /// Whether the browser supports the ResizeObserver API
    pub is_supported: Signal<bool>,
    /// A function to stop and detach the ResizeObserver
    pub stop: F,
}
