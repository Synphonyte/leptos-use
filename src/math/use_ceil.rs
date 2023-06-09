use crate::math::shared::use_simple_math;
use leptos::*;
use num::Float;
use paste::paste;

use_simple_math!(
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
    ceil
);
