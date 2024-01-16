#![cfg_attr(feature = "ssr", allow(unused_variables, unused_imports))]

use cfg_if::cfg_if;
use default_struct_builder::DefaultBuilder;
use leptos::leptos_dom::helpers::TimeoutHandle;
use leptos::{on_cleanup, set_timeout_with_handle, MaybeSignal, SignalGetUntracked};
use std::cell::{Cell, RefCell};
use std::rc::Rc;
use std::time::Duration;

#[derive(Copy, Clone, DefaultBuilder, Default)]
pub struct DebounceOptions {
    /// The maximum time allowed to be delayed before it's invoked.
    /// In milliseconds.
    #[builder(into)]
    pub max_wait: MaybeSignal<Option<f64>>,
}

pub fn debounce_filter<R>(
    ms: impl Into<MaybeSignal<f64>>,
    options: DebounceOptions,
) -> impl Fn(Rc<dyn Fn() -> R>) -> Rc<RefCell<Option<R>>> + Clone
where
    R: 'static,
{
    let timer = Rc::new(Cell::new(None::<TimeoutHandle>));
    let max_timer = Rc::new(Cell::new(None::<TimeoutHandle>));
    let last_return_value: Rc<RefCell<Option<R>>> = Rc::new(RefCell::new(None));

    let clear_timeout = move |timer: &Rc<Cell<Option<TimeoutHandle>>>| {
        if let Some(handle) = timer.get() {
            handle.clear();
            timer.set(None);
        }
    };

    on_cleanup({
        let timer = Rc::clone(&timer);

        move || {
            clear_timeout(&timer);
        }
    });

    let ms = ms.into();
    let max_wait_signal = options.max_wait;

    move |_invoke: Rc<dyn Fn() -> R>| {
        let duration = ms.get_untracked();
        let max_duration = max_wait_signal.get_untracked();

        let last_return_val = Rc::clone(&last_return_value);
        let invoke = move || {
            let return_value = _invoke();

            let mut val_mut = last_return_val.borrow_mut();
            *val_mut = Some(return_value);
        };

        clear_timeout(&timer);

        if duration <= 0.0 || max_duration.is_some_and(|d| d <= 0.0) {
            clear_timeout(&max_timer);

            invoke();
            return Rc::clone(&last_return_value);
        }

        cfg_if! { if #[cfg(not(feature = "ssr"))] {
            // Create the max_timer. Clears the regular timer on invoke
            if let Some(max_duration) = max_duration {
                if max_timer.get().is_none() {
                    let timer = Rc::clone(&timer);
                    let invok = invoke.clone();
                    max_timer.set(
                        set_timeout_with_handle(
                            move || {
                                clear_timeout(&timer);
                                invok();
                            },
                            Duration::from_millis(max_duration as u64),
                        )
                        .ok(),
                    );
                }
            }

            let max_timer = Rc::clone(&max_timer);

            // Create the regular timer. Clears the max timer on invoke
            timer.set(
                set_timeout_with_handle(
                    move || {
                        clear_timeout(&max_timer);
                        invoke();
                    },
                    Duration::from_millis(duration as u64),
                )
                .ok(),
            );
        }}

        Rc::clone(&last_return_value)
    }
}
