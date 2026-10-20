use leptos::prelude::*;
use std::marker::PhantomData;

/// Return type of [`use_timeout_fn`](crate::use_timeout_fn).
pub struct UseTimeoutFnReturn<StartFn, StartArg, StopFn>
where
    StartFn: Fn(StartArg) + Clone + Send + Sync,
    StopFn: Fn() + Clone + Send + Sync,
{
    /// Whether the timeout is pending. When the `callback` is called this is set to `false`.
    pub is_pending: Signal<bool>,

    /// Start the timeout. The `callback` will be called after `delay` milliseconds.
    /// If a previous timer is still pending, it is cancelled and replaced.
    pub start: StartFn,

    /// Stop the timeout. If the timeout was still pending the `callback` is not called.
    pub stop: StopFn,

    pub(crate) _marker: PhantomData<StartArg>,
}
