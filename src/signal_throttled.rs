use crate::utils::{signal_filtered, signal_filtered_local};
use crate::{ThrottleOptions, use_throttle_fn_with_options};
use leptos::prelude::*;
use leptos::reactive::wrappers::read::Signal;

signal_filtered!(
    /// Throttle changing of a `Signal` value.
    ///
    /// Use `*_local` variants for values that are not `Send + Sync`.
    ///
    /// ## Demo
    ///
    /// [Link to Demo](https://github.com/Synphonyte/leptos-use/tree/main/examples/signal_throttled)
    ///
    /// ## Usage
    ///
    /// ```
    /// # use leptos::prelude::*;
    /// # use leptos_use::{signal_throttled, signal_throttled_local};
    /// # use std::cell::RefCell;
    /// #
    /// # #[component]
    /// # fn Demo() -> impl IntoView {
    /// let (input, set_input) = signal("");
    /// let throttled: Signal<&'static str> = signal_throttled(input, 1000.0);
    ///
    /// let (input_local, set_input_local) = signal_local(RefCell::new(0));
    /// let throttled_local: Signal<RefCell<i32>, _> = signal_throttled_local(input_local, 1000.0);
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
    /// # use leptos::prelude::*;
    /// # use leptos_use::{signal_throttled_with_options, signal_throttled_local_with_options, ThrottleOptions};
    /// # use std::cell::RefCell;
    /// #
    /// # #[component]
    /// # fn Demo() -> impl IntoView {
    /// let (input, set_input) = signal("");
    /// let throttled: Signal<&'static str> = signal_throttled_with_options(
    ///     input,
    ///     1000.0,
    ///     ThrottleOptions::default().leading(false).trailing(true)
    /// );
    ///
    /// let (input_local, set_input_local) = signal_local(RefCell::new(0));
    /// let throttled_local: Signal<RefCell<i32>, _> = signal_throttled_local_with_options(
    ///     input_local,
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
    /// > Make sure you follow the [instructions in Server-Side Rendering](https://leptos-use.rs/server_side_rendering.html).
    ///
    /// Internally this uses `setTimeout` which is not supported on the server. So usually
    /// a throttled signal on the server will simply be ignored.
    throttle
    /// [`signal_throttled`]
    /// [`ThrottleOptions`]
);

signal_filtered_local!(
    /// Throttle changing of a `Signal` value that is not `Send + Sync`.
    ///
    /// See ['signal_throttled`] for docs.
    throttle
    /// [`signal_throttled_local`]
    /// [`ThrottledOptions`]
);
