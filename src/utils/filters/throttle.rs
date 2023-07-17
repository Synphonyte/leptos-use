#![cfg_attr(feature = "ssr", allow(unused_variables, unused_imports))]

use crate::utils::CloneableFnWithReturn;
use cfg_if::cfg_if;
use default_struct_builder::DefaultBuilder;
use js_sys::Date;
use leptos::leptos_dom::helpers::TimeoutHandle;
use leptos::{set_timeout_with_handle, MaybeSignal, SignalGetUntracked};
use std::cell::{Cell, RefCell};
use std::cmp::max;
use std::rc::Rc;
use std::time::Duration;

#[derive(Copy, Clone, DefaultBuilder)]
pub struct ThrottleOptions {
    /// Invoke on the trailing edge of the timeout. Defaults to `true`.
    pub trailing: bool,
    /// Invoke on the leading edge of the timeout (=immediately). Defaults to `true`.
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
) -> impl Fn(Box<dyn CloneableFnWithReturn<R>>) -> Rc<RefCell<Option<R>>> + Clone
where
    R: 'static,
{
    let last_exec = Rc::new(Cell::new(0_f64));
    let timer = Rc::new(Cell::new(None::<TimeoutHandle>));
    let is_leading = Rc::new(Cell::new(true));
    let last_return_value: Rc<RefCell<Option<R>>> = Rc::new(RefCell::new(None));

    let t = Rc::clone(&timer);
    let clear = move || {
        if let Some(handle) = t.get() {
            handle.clear();
            t.set(None);
        }
    };

    let ms = ms.into();

    move |mut _invoke: Box<dyn CloneableFnWithReturn<R>>| {
        let duration = ms.get_untracked();
        let elapsed = Date::now() - last_exec.get();

        let last_return_val = Rc::clone(&last_return_value);
        let invoke = move || {
            let return_value = _invoke();

            let mut val_mut = last_return_val.borrow_mut();
            *val_mut = Some(return_value);
        };

        let clear = clear.clone();
        clear();

        if duration <= 0.0 {
            last_exec.set(Date::now());
            invoke();
            return Rc::clone(&last_return_value);
        }

        if elapsed > duration && (options.leading || !is_leading.get()) {
            last_exec.set(Date::now());
            invoke();
        } else if options.trailing {
            cfg_if! { if #[cfg(not(feature = "ssr"))] {
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
            }}
        }

        cfg_if! { if #[cfg(not(feature = "ssr"))] {
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
        }}

        is_leading.set(false);

        Rc::clone(&last_return_value)
    }
}
