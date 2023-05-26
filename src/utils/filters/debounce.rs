use crate::utils::CloneableFnWithReturn;
use leptos::leptos_dom::helpers::TimeoutHandle;
use leptos::{set_timeout_with_handle, MaybeSignal, SignalGetUntracked};
use std::cell::{Cell, RefCell};
use std::rc::Rc;
use std::time::Duration;

#[derive(Default)]
pub struct DebounceOptions {
    /// The maximum time allowed to be delayed before it's invoked.
    /// In milliseconds.
    max_wait: MaybeSignal<Option<f64>>,
}

pub fn debounce_filter(
    ms: impl Into<MaybeSignal<f64>>,
    options: DebounceOptions,
) -> impl FnMut(Box<dyn CloneableFnWithReturn<()>>) -> Rc<RefCell<Option<()>>> {
    let timer = Rc::new(Cell::new(None::<TimeoutHandle>));
    let max_timer = Rc::new(Cell::new(None::<TimeoutHandle>));

    let clear_timeout = move |timer: &Rc<Cell<Option<TimeoutHandle>>>| {
        if let Some(handle) = timer.get() {
            handle.clear();
            timer.set(None);
        }
    };

    let ms = ms.into();

    move |invoke: Box<dyn CloneableFnWithReturn<()>>| {
        let duration = ms.get_untracked();
        let max_duration = options.max_wait.get_untracked();

        // TODO : return value like throttle_filter?

        clear_timeout(&timer);

        if duration <= 0.0 || max_duration.is_some_and(|d| d <= 0.0) {
            clear_timeout(&max_timer);

            invoke();
            return Rc::new(RefCell::new(None));
        }

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

        Rc::new(RefCell::new(None))
    }
}
