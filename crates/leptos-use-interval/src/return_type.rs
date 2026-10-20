use default_struct_builder::DefaultBuilder;
use leptos::reactive::wrappers::read::Signal;

/// Return type of [`use_interval`](crate::use_interval).
#[derive(DefaultBuilder)]
pub struct UseIntervalReturn<PauseFn, ResumeFn, ResetFn>
where
    PauseFn: Fn() + Clone + Send + Sync,
    ResumeFn: Fn() + Clone + Send + Sync,
    ResetFn: Fn() + Clone + Send + Sync,
{
    /// Counter signal that increases by one every interval.
    pub counter: Signal<u64>,

    /// Reset the counter to zero
    pub reset: ResetFn,

    /// A Signal that indicates whether the counter is active. `false` when paused.
    pub is_active: Signal<bool>,

    /// Temporarily pause the counter
    pub pause: PauseFn,

    /// Resume the counter
    pub resume: ResumeFn,
}
