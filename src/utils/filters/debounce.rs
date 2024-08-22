#![cfg_attr(feature = "ssr", allow(unused_variables, unused_imports))]

use cfg_if::cfg_if;
use default_struct_builder::DefaultBuilder;
use leptos::leptos_dom::helpers::TimeoutHandle;
use leptos::prelude::*;
use std::sync::{Arc, Mutex};
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
) -> impl Fn(Arc<dyn Fn() -> R>) -> Arc<Mutex<Option<R>>> + Clone
where
    R: 'static,
{
    let timer = Arc::new(Mutex::new(None::<TimeoutHandle>));
    let max_timer = Arc::new(Mutex::new(None::<TimeoutHandle>));
    let last_return_value: Arc<Mutex<Option<R>>> = Arc::new(Mutex::new(None));

    let clear_timeout = move |timer: &Arc<Mutex<Option<TimeoutHandle>>>| {
        let mut timer = timer.lock().unwrap();
        if let Some(handle) = *timer {
            handle.clear();
            *timer = None;
        }
    };

    on_cleanup({
        let timer = Arc::clone(&timer);

        move || {
            clear_timeout(&timer);
        }
    });

    let ms = ms.into();
    let max_wait_signal = options.max_wait;

    move |_invoke: Arc<dyn Fn() -> R>| {
        let duration = ms.get_untracked();
        let max_duration = max_wait_signal.get_untracked();

        let last_return_val = Arc::clone(&last_return_value);
        let invoke = move || {
            #[cfg(debug_assertions)]
            let zone = leptos::prelude::diagnostics::SpecialNonReactiveZone::enter();

            let return_value = _invoke();

            #[cfg(debug_assertions)]
            drop(zone);

            let mut val_mut = last_return_val.lock().unwrap();
            *val_mut = Some(return_value);
        };

        clear_timeout(&timer);

        if duration <= 0.0 || max_duration.is_some_and(|d| d <= 0.0) {
            clear_timeout(&max_timer);

            invoke();
            return Arc::clone(&last_return_value);
        }

        cfg_if! { if #[cfg(not(feature = "ssr"))] {
            // Create the max_timer. Clears the regular timer on invoke
            if let Some(max_duration) = max_duration {
                let mut max_timer = max_timer.lock().unwrap();

                if max_timer.is_none() {
                    let timer = Arc::clone(&timer);
                    let invok = invoke.clone();
                    *max_timer = set_timeout_with_handle(
                        move || {
                            clear_timeout(&timer);
                            invok();
                        },
                        Duration::from_millis(max_duration as u64),
                    )
                    .ok();
                }
            }

            let max_timer = Arc::clone(&max_timer);

            // Create the regular timer. Clears the max timer on invoke
            *timer.lock().unwrap() = set_timeout_with_handle(
                move || {
                    clear_timeout(&max_timer);
                    invoke();
                },
                Duration::from_millis(duration as u64),
            )
            .ok();
        }}

        Arc::clone(&last_return_value)
    }
}
