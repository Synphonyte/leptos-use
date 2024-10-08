#![cfg_attr(feature = "ssr", allow(unused_variables, unused_imports))]

use crate::core::now;
use cfg_if::cfg_if;
use default_struct_builder::DefaultBuilder;
use leptos::leptos_dom::helpers::TimeoutHandle;
use leptos::prelude::*;
use std::cmp::max;
use std::sync::{atomic::AtomicBool, Arc, Mutex};
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
) -> impl Fn(Arc<dyn Fn() -> R>) -> Arc<Mutex<Option<R>>> + Clone
where
    R: 'static,
{
    let last_exec = Arc::new(Mutex::new(0_f64));
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
        let elapsed = now() - *last_exec.lock().unwrap();

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
            *last_exec.lock().unwrap() = now();
            invoke();
            return Arc::clone(&last_return_value);
        }

        if elapsed > duration
            && (options.leading || !is_leading.load(std::sync::atomic::Ordering::Relaxed))
        {
            *last_exec.lock().unwrap() = now();
            invoke();
        } else if options.trailing {
            cfg_if! { if #[cfg(not(feature = "ssr"))] {
                let last_exec = Arc::clone(&last_exec);
                let is_leading = Arc::clone(&is_leading);
                *timer.lock().unwrap() =
                    set_timeout_with_handle(
                        move || {
                            *last_exec.lock().unwrap() = now();
                            is_leading.store(true, std::sync::atomic::Ordering::Relaxed);
                            invoke();
                            clear();
                        },
                        Duration::from_millis(max(0, (duration - elapsed) as u64)),
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
                        Duration::from_millis(duration as u64),
                    )
                    .ok();
            }
        }}

        is_leading.store(false, std::sync::atomic::Ordering::Relaxed);

        Arc::clone(&last_return_value)
    }
}
