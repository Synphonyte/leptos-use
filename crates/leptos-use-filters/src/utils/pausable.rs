use leptos::reactive::wrappers::read::Signal;

/// Pausable effect
#[derive(Clone)]
pub struct Pausable<PauseFn, ResumeFn>
where
    PauseFn: Fn() + Clone + Send + Sync,
    ResumeFn: Fn() + Clone + Send + Sync,
{
    /// A Signal that indicates whether a pausable instance is active. `false` when paused.
    pub is_active: Signal<bool>,

    /// Temporarily pause the effect from executing
    pub pause: PauseFn,

    /// Resume the effect
    pub resume: ResumeFn,
}
