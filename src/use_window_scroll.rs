#![cfg_attr(feature = "ssr", allow(unused_variables, unused_imports))]

use crate::{use_event_listener_with_options, use_window, UseEventListenerOptions};
use cfg_if::cfg_if;
use leptos::ev::scroll;
use leptos::prelude::*;

/// Reactive window scroll.
///
/// ## Demo
///
/// [Link to Demo](https://github.com/Synphonyte/leptos-use/tree/main/examples/use_window_scroll)
///
/// ## Usage
///
/// ```
/// # use leptos::prelude::*;
/// # use leptos_use::use_window_scroll;
/// #
/// # #[component]
/// # fn Demo() -> impl IntoView {
/// let (x, y) = use_window_scroll();
/// #
/// # view! { }
/// # }
/// ```
///
/// ## Server-Side Rendering
///
/// > Make sure you follow the [instructions in Server-Side Rendering](https://leptos-use.rs/server_side_rendering.html).
///
/// On the server this returns `Signal`s that are always `0.0`.
pub fn use_window_scroll() -> (Signal<f64>, Signal<f64>) {
    cfg_if! { if #[cfg(feature = "ssr")] {
        let initial_x = 0.0;
        let initial_y = 0.0;
    } else {
        let initial_x = window().scroll_x().unwrap_or_default();
        let initial_y = window().scroll_y().unwrap_or_default();
    }}
    let (x, set_x) = signal(initial_x);
    let (y, set_y) = signal(initial_y);

    let _ = use_event_listener_with_options(
        use_window(),
        scroll,
        move |_| {
            set_x.set(window().scroll_x().unwrap_or_default());
            set_y.set(window().scroll_y().unwrap_or_default());
        },
        UseEventListenerOptions::default()
            .capture(false)
            .passive(true),
    );

    (x.into(), y.into())
}
