mod debounce;
mod throttle;

pub use debounce::*;
pub use throttle::*;

use crate::utils::CloneableFnWithReturn;
use std::cell::RefCell;
use std::rc::Rc;

pub fn create_filter_wrapper<F, Filter>(mut filter: Filter, func: F) -> impl FnMut()
where
    F: FnOnce() + Clone + 'static,
    Filter: FnMut(Box<dyn CloneableFnWithReturn<()>>) -> Rc<RefCell<Option<()>>>,
{
    move || {
        filter(Box::new(func.clone()));
    }
}

pub fn create_filter_wrapper_with_arg<F, Arg, Filter>(
    mut filter: Filter,
    func: F,
) -> impl FnMut(Arg)
where
    F: FnOnce(Arg) + Clone + 'static,
    Arg: Clone + 'static,
    Filter: FnMut(Box<dyn CloneableFnWithReturn<()>>) -> Rc<RefCell<Option<()>>>,
{
    move |arg: Arg| {
        let mut func = func.clone();
        filter(Box::new(move || func(arg)));
    }
}

pub fn create_filter_wrapper_with_return<F, R, Filter>(
    mut filter: Filter,
    func: F,
) -> impl FnMut() -> Rc<RefCell<Option<R>>>
where
    F: FnOnce() -> R + Clone + 'static,
    R: 'static,
    Filter: FnMut(Box<dyn CloneableFnWithReturn<R>>) -> Rc<RefCell<Option<R>>>,
{
    move || filter(Box::new(func.clone()))
}

pub fn create_filter_wrapper_with_return_and_arg<F, Arg, R, Filter>(
    mut filter: Filter,
    func: F,
) -> impl FnMut(Arg) -> Rc<RefCell<Option<R>>>
where
    F: FnOnce(Arg) -> R + Clone + 'static,
    R: 'static,
    Arg: Clone + 'static,
    Filter: FnMut(Box<dyn CloneableFnWithReturn<R>>) -> Rc<RefCell<Option<R>>>,
{
    move |arg: Arg| {
        let mut func = func.clone();
        filter(Box::new(move || func(arg)))
    }
}
