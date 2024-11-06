use leptos::prelude::*;

crate::use_derive_signal!(
    /// Reactive `Option::is_some()`.
    ///
    /// ## Usage
    ///
    /// ```
    /// # use leptos::prelude::*;
    /// # use leptos_use::is_some;
    /// #
    /// # #[component]
    /// # fn Demo() -> impl IntoView {
    /// let (example, set_example) = signal(
    ///     if js_sys::Math::random() < 0.5 { Some("Example") } else { None }
    /// );
    ///
    /// let not_empty = is_some::<ReadSignal<Option<&str>>, &str>(example);
    /// #
    /// # view! { }
    /// # }
    /// ```
    is_some<Option<T>, T: 'static> -> bool
    |value| value.is_some()
);
