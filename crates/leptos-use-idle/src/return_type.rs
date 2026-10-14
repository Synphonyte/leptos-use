use leptos::reactive::wrappers::read::Signal;

/// Return type of [`use_idle`](crate::use_idle).
pub struct UseIdleReturn<F>
where
    F: Fn() + Clone + Send + Sync,
{
    /// Wether the use has been inactive for at least `timeout` milliseconds.
    pub idle: Signal<bool>,

    /// Timestamp of last user activity.
    pub last_active: Signal<f64>,

    /// Reset function. Sets the idle state to `false`.
    pub reset: F,
}
