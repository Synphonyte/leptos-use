use crate::utils::{CloneableFnWithReturn, FilterOptions};
use default_struct_builder::DefaultBuilder;
use leptos::prelude::*;
use std::cell::RefCell;
use std::rc::Rc;

pub struct PausableWrapperReturn<PauseFn, ResumeFn, WrappedFn, Arg, R>
where
    PauseFn: Fn() + Clone,
    ResumeFn: Fn() + Clone,
    WrappedFn: Fn(Arg) -> Option<R> + Clone,
    R: 'static,
{
    pub pause: PauseFn,
    pub resume: ResumeFn,
    pub wrapped_fn: WrappedFn,

    _marker_arg: std::marker::PhantomData<Arg>,
    _marker_r: std::marker::PhantomData<R>,
}

pub fn pausable_wrapper<F, Arg, R>(
    function: F,
) -> PausableWrapperReturn<
    impl Fn() + Clone,
    impl Fn() + Clone,
    impl Fn(Arg) -> Option<R> + Clone,
    Arg,
    R,
>
where
    R: 'static,
    F: Fn(Arg) -> R + Clone,
{
    let (active, set_active) = signal(true);

    let pause = move || {
        set_active(false);
    };

    let resume = move || {
        set_active(true);
    };

    let wrapped_fn = move |arg: Arg| {
        if active.get_untracked() {
            Some(function(arg))
        } else {
            None
        }
    };

    PausableWrapperReturn {
        pause,
        resume,
        wrapped_fn,

        _marker_arg: std::marker::PhantomData,
        _marker_r: std::marker::PhantomData,
    }
}
