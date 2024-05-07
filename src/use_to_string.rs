use crate::utils::use_derive_signal;
use leptos::prelude::*;

use_derive_signal!(
    /// Reactive `ToString::to_string()`.
    ///
    /// ## Usage
    ///
    /// ```
    /// # use leptos::prelude::*;
    /// # use leptos_use::use_to_string;
    /// #
    /// # #[component]
    /// # fn Demo() -> impl IntoView {
    /// let (number, set_number) = signal(3.14_f64);
    /// let str = use_to_string::<_, f64>(number);
    /// #
    /// # view! { }
    /// # }
    /// ```
    use_to_string<T, T: ToString + 'static> -> String
    |value| value.to_string()
);
