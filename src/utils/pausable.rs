use std::cell::Cell;
use leptos::{ReadSignal, Signal};

/// Pausable effect
pub struct Pausable<PauseFn, ResumeFn>
where
    PauseFn: Fn() + Clone,
    ResumeFn: Fn() + Clone,
{
    /// A Signal that indicates whether a pausable instance is active. `false` when paused.
    pub is_active: Signal<bool>,

    /// Temporarily pause the effect from executing
    pub pause: PauseFn,

    /// Resume the effect
    pub resume: ResumeFn,
}
