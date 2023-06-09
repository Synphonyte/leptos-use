use leptos::*;
use leptos_use::math::shared::use_simple_math;
use num::Float;
use paste::paste;

use_simple_math!(
    /// Reactive `round()`.
    ///
    /// ## Demo
    ///
    /// [Link to Demo](https://github.com/Synphonyte/leptos-use/tree/main/examples/use_round)
    ///
    /// ## Usage
    ///
    /// ```
    /// # use leptos::*;
    /// # use leptos_use::math::use_round;
    /// #
    /// # #[component]
    /// # fn Demo(cx: Scope) -> impl IntoView {
    /// let (value, set_value) = create_signal(cx, 45.95);
    /// let result: Signal<f64> = use_round(cx, value); // 46
    /// #
    /// # assert_eq!(result.get(), 46.0);
    /// # view! { cx, }
    /// # }
    /// ```
    #[doc(cfg(feature = "math"))]
    round
);
