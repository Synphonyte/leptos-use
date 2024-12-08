use crate::utils::{signal_filtered, signal_filtered_local};
use crate::{use_debounce_fn_with_options, DebounceOptions};
use leptos::prelude::*;
use leptos::reactive::wrappers::read::Signal;
use paste::paste;

signal_filtered!(
    /// Debounce changing of a `Signal` value.
    ///
    /// Use `*_local` variants for values that are not `Send + Sync`.
    ///
    /// ## Demo
    ///
    /// [Link to Demo](https://github.com/Synphonyte/leptos-use/tree/main/examples/signal_debounced)
    ///
    /// ## Usage
    ///
    /// ```
    /// # use leptos::prelude::*;
    /// # use leptos_use::{signal_debounced, signal_debounced_local};
    /// # use std::cell::RefCell;
    /// #
    /// # #[component]
    /// # fn Demo() -> impl IntoView {
    /// let (input, set_input) = signal("");
    /// let debounced: Signal<&'static str> = signal_debounced(input, 1000.0);
    ///
    /// let (input_local, set_input_local) = signal_local(RefCell::new(0));
    /// let debounced_local: Signal<RefCell<i32>, _> = signal_debounced_local(input_local, 1000.0);
    /// #
    /// # view! { }
    /// # }
    /// ```
    ///
    /// ### Options
    ///
    /// The usual debounce option `max_wait` is available.
    ///
    /// ```
    /// # use leptos::prelude::*;
    /// # use leptos_use::{signal_debounced_with_options, signal_debounced_local_with_options, DebounceOptions};
    /// # use std::cell::RefCell;
    /// #
    /// # #[component]
    /// # fn Demo() -> impl IntoView {
    /// let (input, set_input) = signal("");
    /// let debounced: Signal<&'static str> = signal_debounced_with_options(
    ///     input,
    ///     1000.0,
    ///     DebounceOptions::default().max_wait(Some(500.0))
    /// );
    ///
    /// let (input_local, set_input_local) = signal_local(RefCell::new(0));
    /// let debounced_local: Signal<RefCell<i32>, _> = signal_debounced_local_with_options(
    ///     input_local,
    ///     1000.0,
    ///     DebounceOptions::default().max_wait(Some(500.0))
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
    debounce
    /// [`signal_debounced`]
    /// [`DebounceOptions`]
);

signal_filtered_local!(
    /// Debounce changing of a `Signal` value that is not `Send + Sync`.
    ///
    /// See ['signal_debounced`] for docs.
    debounce
    /// [`signal_debounced_local`]
    /// [`DebounceOptions`]
);
