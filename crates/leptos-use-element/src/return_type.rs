use leptos::reactive::wrappers::read::Signal;

/// The return value of [`use_mutation_observer`](crate::use_mutation_observer).
pub struct UseMutationObserverReturn<F: Fn() + Clone + Send + Sync> {
    /// Whether the browser supports the MutationObserver API
    pub is_supported: Signal<bool>,
    /// A function to stop and detach the MutationObserver
    pub stop: F,
}
