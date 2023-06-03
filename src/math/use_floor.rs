use leptos::*;
use num::Float;

/// Reactive `floor()`.
///
/// ## Demo
///
/// [Link to Demo](https://github.com/Synphonyte/leptos-use/tree/main/examples/use_floor)
///
/// ### Usage
///
/// ```
/// # use leptos::*;
/// # use leptos_use::math::use_floor;
/// #
/// # #[component]
/// # fn Demo(cx: Scope) -> impl IntoView {
/// let (value, set_value) = create_signal(cx, 45.95);
/// let result: Signal<f64> = use_floor(cx, value); // 45
/// # view! { cx, }
/// # }
/// ```
pub fn use_floor<S, N>(cx: Scope, x: S) -> Signal<N>
where
    S: Into<MaybeSignal<N>>,
    N: Float,
{
    let x = x.into();
    Signal::derive(cx, move || x.get().floor())
}
