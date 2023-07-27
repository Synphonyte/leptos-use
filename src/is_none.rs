use crate::utils::use_derive_signal;
use leptos::*;

use_derive_signal!(
    /// Reactive `Option::is_none()`.
    ///
    /// ## Usage
    ///
    /// ```
    /// # use leptos::*;
    /// # use leptos_use::is_none;
    /// #
    /// # #[component]
    /// # fn Demo() -> impl IntoView {
    /// let (example, set_example) = create_signal(
    ///     if js_sys::Math::random() < 0.5 { Some("Example") } else { None }
    /// );
    ///
    /// let is_empty = is_none(example);
    /// #
    /// # view! { }
    /// # }
    /// ```
    is_none<Option<T>, T: 'static> -> bool
    |value| value.is_none()
);
