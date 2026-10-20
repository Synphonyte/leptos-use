use leptos::prelude::*;

/// Return type of [`watch_pausable`](crate::watch_pausable)
pub struct WatchPausableReturn<StopFn, PauseFn, ResumeFn>
where
    StopFn: Fn() + Clone + Send + Sync,
    PauseFn: Fn() + Clone + Send + Sync,
    ResumeFn: Fn() + Clone + Send + Sync,
{
    /// Stops the watcher
    pub stop: StopFn,

    /// Pauses the watcher
    pub pause: PauseFn,

    /// Resumes the watcher
    pub resume: ResumeFn,

    /// Whether the watcher is active (not paused). This doesn't reflect if the watcher has been stopped
    pub is_active: Signal<bool>,
}
