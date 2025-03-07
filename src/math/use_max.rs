use crate::math::shared::use_partial_cmp;
use leptos::prelude::*;
use leptos::reactive::wrappers::read::Signal;
use std::cmp::Ordering;

use_partial_cmp!(
    /// Reactive `max()`.
    ///
    /// Works with any container that implements `IntoIterator` (`Vec`, `HashSet`, ...)
    /// with any elements that implement `PartialOrd` and `Clone` (floats, ints, strings, ...).
    ///
    /// If the container is empty or only contains non comparable values like `NaN`, it returns `None`.
    /// Otherwise it returns the `Some(<largest value>)` in the container.
    ///
    /// ## Usage
    ///
    /// ```
    /// # use leptos::prelude::*;
    /// # use leptos_use::math::use_max;
    /// #
    /// # #[component]
    /// # fn Demo() -> impl IntoView {
    /// let (values, set_values) = signal(vec![1.0, 2.0, 3.0, f32::NAN, 4.0, 5.0]);
    /// let result = use_max::<Vec<f32>, _, _>(values); // Some(5.0)
    /// #
    /// # assert_eq!(result.get(), Some(5.0));
    /// # view! { }
    /// # }
    /// ```
    // #[doc(cfg(feature = "math"))]
    use_max,
    Ordering::Less
);
