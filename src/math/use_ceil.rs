use leptos::*;
use num::Float;

/// Reactive `ceil()`.
///
/// ## Demo
///
/// [Link to Demo](https://github.com/Synphonyte/leptos-use/tree/main/examples/use_ceil)
///
/// ## Usage
///
/// ```
/// # use leptos::*;
/// # use leptos_use::math::use_ceil;
/// #
/// # #[component]
/// # fn Demo(cx: Scope) -> impl IntoView {
/// let (value, set_value) = create_signal(cx, 44.15);
/// let result: Signal<f64> = use_ceil(cx, value); // 45
/// #
/// # assert_eq!(result.get(), 45.0);
/// # view! { cx, }
/// # }
/// ```
#[doc(cfg(feature = "math"))]
pub fn use_ceil<S, N>(cx: Scope, x: S) -> Signal<N>
where
    S: Into<MaybeSignal<N>>,
    N: Float,
{
    let x = x.into();
    Signal::derive(cx, move || x.get().ceil())
}
