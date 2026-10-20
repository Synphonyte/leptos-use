use crate::{
    Pausable, TimestampInterval, UseIntervalFnOptions, UseRafFnOptions, UseTimestampOptions,
    UseTimestampReturn, now, use_interval_fn_with_options, use_raf_fn_with_options,
};
use leptos::prelude::*;
use leptos::reactive::wrappers::read::Signal;
use std::rc::Rc;
use std::sync::Arc;

/// Reactive current timestamp.
///
/// ## Demo
///
/// [Link to Demo](https://github.com/Synphonyte/leptos-use/tree/main/examples/use_timestamp)
///
/// ## Usage
///
/// ```
/// # use leptos::prelude::*;
/// # use leptos_use_timestamp::use_timestamp;
/// #
/// # #[component]
/// # fn Demo() -> impl IntoView {
/// let timestamp = use_timestamp();
/// #
/// # view! { }
/// # }
/// ```
///
/// With controls:
///
/// ```
/// # use leptos::prelude::*;
/// # use leptos_use_timestamp::{use_timestamp_with_controls, UseTimestampReturn};
/// #
/// # #[component]
/// # fn Demo() -> impl IntoView {
/// let UseTimestampReturn {
///     timestamp,
///     is_active,
///     pause,
///     resume,
/// } = use_timestamp_with_controls();
/// #
/// # view! { }
/// # }
/// ```
///
/// ## SendWrapped Return
///
/// The returned closures `pause` and `resume` of the `..._with_controls` versions are
/// sendwrapped functions. They can only be called from the same thread that called
/// `use_timestamp_with_controls`.
///
/// ## Server-Side Rendering
///
/// > Make sure you follow the [instructions in Server-Side Rendering](https://leptos-use.rs/server_side_rendering.html).
///
/// On the server this function will return a signal with the milliseconds since the Unix epoch.
/// But the signal will never update (as there's no `request_animation_frame` on the server).
pub fn use_timestamp() -> Signal<f64> {
    use_timestamp_with_controls().timestamp
}

/// Version of [`use_timestamp`] that takes a `UseTimestampOptions`. See [`use_timestamp`] for how to use.
pub fn use_timestamp_with_options(options: UseTimestampOptions) -> Signal<f64> {
    use_timestamp_with_controls_and_options(options).timestamp
}

/// Version of [`use_timestamp`] that returns controls. See [`use_timestamp`] for how to use.
pub fn use_timestamp_with_controls() -> UseTimestampReturn {
    use_timestamp_with_controls_and_options(UseTimestampOptions::default())
}

/// Version of [`use_timestamp`] that takes a `UseTimestampOptions` and returns controls. See [`use_timestamp`] for how to use.
pub fn use_timestamp_with_controls_and_options(options: UseTimestampOptions) -> UseTimestampReturn {
    let UseTimestampOptions {
        offset,
        immediate,
        interval,
        callback,
    } = options;

    let (ts, set_ts) = signal(now() + offset);

    let update = move || {
        set_ts.set(now() + offset);
    };

    let cb = {
        let callback = Rc::clone(&callback);

        move || {
            update();

            #[cfg(debug_assertions)]
            let _z = leptos::reactive::diagnostics::SpecialNonReactiveZone::enter();

            callback(ts.get_untracked());
        }
    };

    match interval {
        TimestampInterval::RequestAnimationFrame => {
            let Pausable {
                pause,
                resume,
                is_active,
            } = use_raf_fn_with_options(
                move |_| cb(),
                UseRafFnOptions::default().immediate(immediate),
            );

            UseTimestampReturn {
                timestamp: ts.into(),
                is_active,
                pause: Arc::new(pause),
                resume: Arc::new(resume),
            }
        }

        TimestampInterval::Interval(interval) => {
            let Pausable {
                pause,
                resume,
                is_active,
            } = use_interval_fn_with_options(
                cb,
                interval,
                UseIntervalFnOptions::default().immediate(immediate),
            );

            UseTimestampReturn {
                timestamp: ts.into(),
                is_active,
                pause: Arc::new(pause),
                resume: Arc::new(resume),
            }
        }
    }
}
