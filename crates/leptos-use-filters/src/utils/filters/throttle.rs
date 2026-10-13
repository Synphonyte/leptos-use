#![cfg_attr(feature = "ssr", allow(unused_variables, unused_imports))]

use crate::core::now;
use cfg_if::cfg_if;
use default_struct_builder::DefaultBuilder;
use leptos::leptos_dom::helpers::TimeoutHandle;
use leptos::prelude::*;
use std::sync::{Arc, Mutex, atomic::AtomicBool};
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
    ms: impl Into<Signal<f64>>,
    options: ThrottleOptions,
) -> impl Fn(Arc<dyn Fn() -> R>) -> Arc<Mutex<Option<R>>> + Clone
where
    R: 'static,
{
    let last_exec = Arc::new(Mutex::new(None::<f64>));
    let timer = Arc::new(Mutex::new(None::<TimeoutHandle>));
    let is_leading = Arc::new(AtomicBool::new(true));
    let last_return_value: Arc<Mutex<Option<R>>> = Arc::new(Mutex::new(None));

    let t = Arc::clone(&timer);
    let clear = move || {
        let mut t = t.lock().unwrap();
        if let Some(handle) = *t {
            handle.clear();
            *t = None;
        }
    };

    on_cleanup(clear.clone());

    let ms = ms.into();

    move |mut _invoke: Arc<dyn Fn() -> R>| {
        let duration = ms.get_untracked();
        // `None` means the filter has never executed yet.
        let elapsed = last_exec.lock().unwrap().map(|last| now() - last);

        let last_return_val = Arc::clone(&last_return_value);
        let invoke = move || {
            #[cfg(debug_assertions)]
            let zone = leptos::reactive::diagnostics::SpecialNonReactiveZone::enter();

            let return_value = _invoke();

            #[cfg(debug_assertions)]
            drop(zone);

            let mut val_mut = last_return_val.lock().unwrap();
            *val_mut = Some(return_value);
        };

        let clear = clear.clone();
        clear();

        if duration <= 0.0 {
            *last_exec.lock().unwrap() = Some(now());
            invoke();
            return Arc::clone(&last_return_value);
        }

        if elapsed.is_none_or(|elapsed| elapsed > duration)
            && (options.leading || !is_leading.load(std::sync::atomic::Ordering::Relaxed))
        {
            *last_exec.lock().unwrap() = Some(now());
            invoke();
        } else if options.trailing {
            cfg_if! { if #[cfg(not(feature = "ssr"))] {
                let remaining = elapsed.map_or(duration, |elapsed| (duration - elapsed).max(0.0));

                let last_exec = Arc::clone(&last_exec);
                let is_leading = Arc::clone(&is_leading);
                *timer.lock().unwrap() =
                    set_timeout_with_handle(
                        move || {
                            *last_exec.lock().unwrap() = Some(now());
                            is_leading.store(true, std::sync::atomic::Ordering::Relaxed);
                            invoke();
                            clear();
                        },
                        Duration::from_millis((remaining as u64).min(i32::MAX as u64)),
                    )
                    .ok();
            }}
        }

        cfg_if! { if #[cfg(not(feature = "ssr"))] {
            let mut timer = timer.lock().unwrap();

            if !options.leading && timer.is_none() {
                let is_leading = Arc::clone(&is_leading);
                *timer = set_timeout_with_handle(
                        move || {
                            is_leading.store(true, std::sync::atomic::Ordering::Relaxed);
                        },
                        Duration::from_millis((duration as u64).min(i32::MAX as u64)),
                    )
                    .ok();
            }
        }}

        is_leading.store(false, std::sync::atomic::Ordering::Relaxed);

        Arc::clone(&last_return_value)
    }
}
