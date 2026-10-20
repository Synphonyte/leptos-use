use leptos::reactive::wrappers::read::Signal;
use std::sync::Arc;

/// Return type of [`use_timestamp_with_controls`](crate::use_timestamp_with_controls).
pub struct UseTimestampReturn {
    /// The current timestamp
    pub timestamp: Signal<f64>,

    /// A Signal that indicates whether the timestamp updating is active. `false` when paused.
    pub is_active: Signal<bool>,

    /// Temporarily pause the timestamp from updating
    pub pause: Arc<dyn Fn()>,

    /// Resume the timestamp updating
    pub resume: Arc<dyn Fn()>,
}
