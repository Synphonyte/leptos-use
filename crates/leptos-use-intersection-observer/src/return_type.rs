use leptos::reactive::wrappers::read::Signal;

/// The return value of [`use_intersection_observer`](crate::use_intersection_observer).
pub struct UseIntersectionObserverReturn<StopFn, PauseFn, ResumeFn>
where
    StopFn: Fn() + Clone + Send + Sync,
    PauseFn: Fn() + Clone + Send + Sync,
    ResumeFn: Fn() + Clone + Send + Sync,
{
    /// Pauses the `IntersectionObserver` observations. Will cause `is_active = false`.
    pub pause: PauseFn,
    /// Resumes the `IntersectionObserver` observations. Will cause `is_active = true`.
    pub resume: ResumeFn,
    /// Stops the `IntersectionObserver` observations altogether.
    pub stop: StopFn,
    /// A signal which is `true` when the `IntersectionObserver` is active, and `false` when paused or stopped.
    pub is_active: Signal<bool>,
}
