use crate::utils::use_derive_signal;
use leptos::*;

use_derive_signal!(
    /// Reactive `ToString::to_string()`.
    ///
    /// ## Usage
    ///
    /// ```
    /// # use leptos::*;
    /// # use leptos_use::use_to_string;
    /// #
    /// # #[component]
    /// # fn Demo(cx: Scope) -> impl IntoView {
    /// let (number, set_number) = create_signal(cx, 3.14_f64);
    /// let str = use_to_string::<_, f64>(cx, number);
    /// #
    /// # view! { cx, }
    /// # }
    /// ```
    use_to_string<T, T: ToString + 'static> -> String
    |value| value.to_string()
);
