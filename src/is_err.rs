use leptos::*;

crate::use_derive_signal!(
    /// Reactive `Result::is_err()`.
    ///
    /// ## Usage
    ///
    /// ```
    /// # use leptos::*;
    /// # use leptos_use::is_err;
    /// #
    /// # #[component]
    /// # fn Demo() -> impl IntoView {
    /// let (example, set_example) = create_signal(
    ///     if js_sys::Math::random() < 0.5 { Ok("Example") } else { Err(()) }
    /// );
    ///
    /// let is_error = is_err(example);
    /// #
    /// # view! { }
    /// # }
    /// ```
    is_err<Result<T, ()>, T: 'static> -> bool
    |value| value.is_err()
);
