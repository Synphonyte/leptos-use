use crate::utils::use_derive_signal;
use leptos::*;

use_derive_signal!(
    /// Reactive `Option::is_some()`.
    ///
    /// ## Usage
    ///
    /// ```
    /// # use leptos::*;
    /// # use leptos_use::is_some;
    /// #
    /// # #[component]
    /// # fn Demo(cx: Scope) -> impl IntoView {
    /// let (example, set_example) = create_signal(
    ///     cx,
    ///     if js_sys::Math::random() < 0.5 { Some("Example") } else { None }
    /// );
    ///
    /// let not_empty = is_some(cx, example);
    /// #
    /// # view! { cx, }
    /// # }
    /// ```
    is_some<Option<T>, T: 'static> -> bool
    |value| value.is_some()
);
