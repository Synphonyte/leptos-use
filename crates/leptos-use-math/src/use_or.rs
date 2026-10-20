use crate::shared::use_binary_logic;
use leptos::prelude::*;
use leptos::reactive::wrappers::read::Signal;
use paste::paste;

use_binary_logic!(
    /// Reactive `OR` condition.
    ///
    /// ## Demo
    ///
    /// [Link to Demo](https://github.com/Synphonyte/leptos-use/tree/main/examples/use_or)
    ///
    /// ## Usage
    ///
    /// ```
    /// # use leptos::prelude::*;
    /// # use leptos_use_math::use_or;
    /// #
    /// # #[component]
    /// # fn Demo() -> impl IntoView {
    /// let (a, set_a) = signal(true);
    /// let (b, set_b) = signal(false);
    ///
    /// let a_or_b = use_or(a, b);
    /// #
    /// # view! { }
    /// # }
    /// ```
    or
    ||
);
