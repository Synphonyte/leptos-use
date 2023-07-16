use crate::utils::use_derive_signal;
use leptos::*;

use_derive_signal!(
    /// Reactive `Result::is_err()`.
    ///
    /// ## Usage
    ///
    /// ```
    /// # use leptos::*;
    /// # use leptos_use::is_err;
    /// #
    /// # #[component]
    /// # fn Demo(cx: Scope) -> impl IntoView {
    /// let (example, set_example) = create_signal(
    ///     cx,
    ///     if js_sys::Math::random() < 0.5 { Ok("Example") } else { Err(()) }
    /// );
    ///
    /// let is_error = is_err(cx, example);
    /// #
    /// # view! { cx, }
    /// # }
    /// ```
    is_err<Result<T, ()>, T: 'static> -> bool
    |value| value.is_err()
);
