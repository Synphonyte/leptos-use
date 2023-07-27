use crate::utils::signal_filtered;
use crate::{use_throttle_fn_with_options, ThrottleOptions};
use leptos::*;
use paste::paste;

signal_filtered!(
    /// Throttle changing of a `Signal` value.
    ///
    /// ## Demo
    ///
    /// [Link to Demo](https://github.com/Synphonyte/leptos-use/tree/main/examples/signal_throttled)
    ///
    /// ## Usage
    ///
    /// ```
    /// # use leptos::*;
    /// # use leptos_use::signal_throttled;
    /// #
    /// # #[component]
    /// # fn Demo() -> impl IntoView {
    /// let (input, set_input) = create_signal("");
    /// let throttled = signal_throttled(input, 1000.0);
    /// #
    /// # view! { }
    /// # }
    /// ```
    ///
    /// ### Options
    ///
    /// The usual throttle options `leading` and `trailing` are available.
    ///
    /// ```
    /// # use leptos::*;
    /// # use leptos_use::{signal_throttled_with_options, ThrottleOptions};
    /// #
    /// # #[component]
    /// # fn Demo() -> impl IntoView {
    /// let (input, set_input) = create_signal("");
    /// let throttled = signal_throttled_with_options(
    ///     input,
    ///     1000.0,
    ///     ThrottleOptions::default().leading(false).trailing(true)
    /// );
    /// #
    /// # view! { }
    /// # }
    /// ```
    ///
    /// ## Recommended Reading
    ///
    /// - [**Debounce vs Throttle**: Definitive Visual Guide](https://redd.one/blog/debounce-vs-throttle)
    /// - [Debouncing and Throttling Explained Through Examples](https://css-tricks.com/debouncing-throttling-explained-examples/)
    ///
    /// ## Server-Side Rendering
    ///
    /// Internally this uses `setTimeout` which is not supported on the server. So usually
    /// a throttled signal on the server will simply be ignored.
    throttle
    /// [`signal_throttled`]
    /// [`ThrottleOptions`]
);
