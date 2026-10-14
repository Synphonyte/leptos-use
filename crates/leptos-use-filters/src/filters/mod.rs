mod debounce;
mod throttle;

pub use debounce::*;
pub use throttle::*;

use crate::sendwrap_fn;
use std::sync::{Arc, Mutex};

pub fn create_filter_wrapper<F, R>(
    filter: ArcFilterFn!(R),
    func: F,
) -> impl Fn() -> Arc<Mutex<Option<R>>> + Clone + Send + Sync
where
    F: Fn() -> R + Clone + 'static,
    R: 'static,
{
    sendwrap_fn!(move || Arc::clone(&filter)(Arc::new(func.clone())))
}

pub fn create_filter_wrapper_with_arg<F, Arg, R>(
    filter: ArcFilterFn!(R),
    func: F,
) -> impl Fn(Arg) -> Arc<Mutex<Option<R>>> + Clone + Send + Sync
where
    F: Fn(Arg) -> R + Clone + 'static,
    R: 'static,
    Arg: Clone + 'static,
{
    sendwrap_fn!(move |arg: Arg| {
        let func = func.clone();
        Arc::clone(&filter)(Arc::new(move || func(arg.clone())))
    })
}
