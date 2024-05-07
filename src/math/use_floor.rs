use crate::math::shared::use_simple_math;
use leptos::prelude::wrappers::read::Signal;
use leptos::prelude::*;
use num::Float;
use paste::paste;

use_simple_math!(
    /// Reactive `floor()`.
    ///
    /// ## Demo
    ///
    /// [Link to Demo](https://github.com/Synphonyte/leptos-use/tree/main/examples/use_floor)
    ///
    /// ## Usage
    ///
    /// ```
    /// # use leptos::prelude::*;
    /// # use leptos_use::math::use_floor;
    /// #
    /// # #[component]
    /// # fn Demo() -> impl IntoView {
    /// let (value, set_value) = signal(45.95);
    /// let result: Signal<f64> = use_floor(value); // 45
    /// #
    /// # assert_eq!(result.get(), 45.0);
    /// # view! { }
    /// # }
    /// ```
    // #[doc(cfg(feature = "math"))]
    floor
);
