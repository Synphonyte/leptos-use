use leptos::prelude::*;
use leptos::reactive_graph::wrappers::read::Signal;

/// Reactive `NOT` condition.
///
/// ## Demo
///
/// [Link to Demo](https://github.com/Synphonyte/leptos-use/tree/main/examples/use_not)
///
/// ## Usage
///
/// ```
/// # use leptos::prelude::*;
/// # use leptos_use::math::use_not;
/// #
/// # #[component]
/// # fn Demo() -> impl IntoView {
/// let (a, set_a) = signal(true);
///
/// let not_a = use_not(a);
/// #
/// # view! { }
/// # }
/// ```
///
/// See also
///
/// - [`use_and`]
/// - [`use_or`]
///
// #[doc(cfg(feature = "math"))]
pub fn use_not<S>(a: S) -> Signal<bool>
where
    S: Into<MaybeSignal<bool>>,
{
    let a = a.into();
    Signal::derive(move || !a.get())
}
