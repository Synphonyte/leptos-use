use crate::{use_throttle_fn_with_options, ThrottleOptions};
use leptos::*;

/// Throttle changing of a `Signal` value.
///
/// ## Demo
///
/// [Link to Demo](https://github.com/Synphonyte/leptos-use/tree/main/examples/signal_throttled)
///
/// ## Usage
///
/// ```
/// # use leptos::*;
/// # use leptos_use::signal_throttled;
/// #
/// # #[component]
/// # fn Demo(cx: Scope) -> impl IntoView {
/// let (input, set_input) = create_signal(cx, "");
/// let throttled = signal_throttled(cx, input, 1000.0);
/// #
/// # view! { cx, }
/// # }
/// ```
/// 
/// ### Options
///
/// The usual throttle options `leading` and `trailing` are available. 
///
/// ```
/// # use leptos::*;
/// # use leptos_use::{signal_throttled_with_options, ThrottleOptions};
/// #
/// # #[component]
/// # fn Demo(cx: Scope) -> impl IntoView {
/// let (input, set_input) = create_signal(cx, "");
/// let throttled = signal_throttled_with_options(cx, input, 1000.0, ThrottleOptions::default().leading(false).trailing(true));
/// #
/// # view! { cx, }
/// # }
/// ```
pub fn signal_throttled<S, T>(
    cx: Scope,
    value: S,
    ms: impl Into<MaybeSignal<f64>> + 'static,
) -> Signal<T>
where
    S: Into<Signal<T>>,
    T: Clone + 'static,
{
    signal_throttled_with_options(cx, value, ms, ThrottleOptions::default())
}

/// Version of [`signal_throttled`] that takes a `SignalThrottledOptions`. See [`signal_throttled`] for how to use.
pub fn signal_throttled_with_options<S, T>(
    cx: Scope,
    value: S,
    ms: impl Into<MaybeSignal<f64>> + 'static,
    options: ThrottleOptions,
) -> Signal<T>
where
    S: Into<Signal<T>>,
    T: Clone + 'static,
{
    let value = value.into();
    let ms = ms.into();

    if ms.get_untracked() <= 0.0 {
        return value;
    }

    let (throttled, set_throttled) = create_signal(cx, value.get_untracked());

    let update = use_throttle_fn_with_options(
        move || set_throttled.set(value.get_untracked()),
        ms,
        options,
    );

    let _ = watch(cx, move || value.get(), move |_, _, _| update(), false);

    throttled.into()
}
