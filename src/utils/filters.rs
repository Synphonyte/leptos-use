use js_sys::Date;
use leptos::leptos_dom::helpers::TimeoutHandle;
use leptos::{set_timeout_with_handle, MaybeSignal, SignalGetUntracked};
use std::cell::{Cell, RefCell};
use std::cmp::max;
use std::rc::Rc;
use std::time::Duration;

pub fn create_filter_wrapper<F, R, Filter>(
    mut filter: Filter,
    func: F,
) -> impl FnMut() -> Rc<RefCell<Option<R>>>
where
    F: FnMut() -> R + Clone + 'static,
    R: 'static,
    Filter: FnMut(Box<dyn FnOnce() -> R>) -> Rc<RefCell<Option<R>>>,
{
    move || {
        let wrapped_func = Box::new(func.clone());
        filter(wrapped_func)
    }
}

pub fn create_filter_wrapper_with_arg<F, Arg, R, Filter>(
    mut filter: Filter,
    func: F,
) -> impl FnMut(Arg) -> Rc<RefCell<Option<R>>>
where
    F: FnMut(Arg) -> R + Clone + 'static,
    R: 'static,
    Arg: 'static,
    Filter: FnMut(Box<dyn FnOnce() -> R>) -> Rc<RefCell<Option<R>>>,
{
    move |arg: Arg| {
        let mut func = func.clone();
        let wrapped_func = Box::new(move || func(arg));
        filter(wrapped_func)
    }
}

#[derive(Copy, Clone)]
pub struct ThrottleOptions {
    pub trailing: bool,
    pub leading: bool,
}

impl Default for ThrottleOptions {
    fn default() -> Self {
        Self {
            trailing: true,
            leading: true,
        }
    }
}

pub fn throttle_filter<R>(
    ms: impl Into<MaybeSignal<f64>>,
    options: ThrottleOptions,
) -> impl FnMut(Box<dyn FnOnce() -> R>) -> Rc<RefCell<Option<R>>>
where
    R: 'static,
{
    let last_exec = Rc::new(Cell::new(0_f64));
    let timer = Rc::new(Cell::new(None::<TimeoutHandle>));
    let is_leading = Rc::new(Cell::new(true));
    let last_value: Rc<RefCell<Option<R>>> = Rc::new(RefCell::new(None));

    let t = Rc::clone(&timer);
    let clear = move || {
        if let Some(handle) = t.get() {
            handle.clear();
            t.set(None);
        }
    };

    let ms = ms.into();

    move |mut _invoke: Box<dyn FnOnce() -> R>| {
        let duration = ms.get_untracked();
        let elapsed = Date::now() - last_exec.get();

        let last_val = Rc::clone(&last_value);
        let invoke = move || {
            let return_value = _invoke();

            let mut val_mut = last_val.borrow_mut();
            *val_mut = Some(return_value);
        };

        let clear = clear.clone();
        clear();

        if duration <= 0.0 {
            last_exec.set(Date::now());
            invoke();
            return Rc::clone(&last_value);
        }

        if elapsed > duration && (options.leading || !is_leading.get()) {
            last_exec.set(Date::now());
            invoke();
        } else if options.trailing {
            let last_exec = Rc::clone(&last_exec);
            let is_leading = Rc::clone(&is_leading);
            timer.set(
                set_timeout_with_handle(
                    move || {
                        last_exec.set(Date::now());
                        is_leading.set(true);
                        invoke();
                        clear();
                    },
                    Duration::from_millis(max(0, (duration - elapsed) as u64)),
                )
                .ok(),
            );
        }

        if !options.leading && timer.get().is_none() {
            let is_leading = Rc::clone(&is_leading);
            timer.set(
                set_timeout_with_handle(
                    move || {
                        is_leading.set(true);
                    },
                    Duration::from_millis(duration as u64),
                )
                .ok(),
            );
        }

        is_leading.set(false);

        Rc::clone(&last_value)
    }
}
