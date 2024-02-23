use crate::math::shared::use_binary_logic;
use leptos::*;
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
    /// # use leptos::*;
    /// # use leptos_use::math::use_or;
    /// #
    /// # #[component]
    /// # fn Demo() -> impl IntoView {
    /// let (a, set_a) = create_signal(true);
    /// let (b, set_b) = create_signal(false);
    ///
    /// let a_or_b = use_or(a, b);
    /// #
    /// # view! { }
    /// # }
    /// ```
    // #[doc(cfg(feature = "math"))]
    or
    ||
);
