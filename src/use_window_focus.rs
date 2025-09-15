#![cfg_attr(feature = "ssr", allow(unused_variables, unused_imports))]

use crate::use_event_listener;
use cfg_if::cfg_if;
use leptos::ev::{blur, focus};
use leptos::prelude::*;

/// Reactively track window focus
/// with `window.onfocus` and `window.onblur` events.
///
/// ## Demo
///
/// [Link to Demo](https://github.com/Synphonyte/leptos-use/tree/main/examples/use_window_focus)
///
/// ## Usage
///
/// ```
/// # use leptos::prelude::*;
/// # use leptos_use::use_window_focus;
/// #
/// # #[component]
/// # fn Demo() -> impl IntoView {
/// let focused = use_window_focus();
/// #
/// # view! { }
/// # }
/// ```
///
/// ## Server-Side Rendering
///
/// > Make sure you follow the [instructions in Server-Side Rendering](https://leptos-use.rs/server_side_rendering.html).
///
/// On the server this returns a `Signal` that is always `true`.
pub fn use_window_focus() -> Signal<bool> {
    cfg_if! { if #[cfg(feature = "ssr")] {
        let initial_focus = true;
    } else {
        let initial_focus = document().has_focus().unwrap_or_default();
    }}

    let (focused, set_focused) = signal(initial_focus);

    cfg_if! { if #[cfg(not(feature = "ssr"))] {
        let _ = use_event_listener(window(), blur, move |_| set_focused.set(false));
        let _ = use_event_listener(window(), focus, move |_| set_focused.set(true));
    }}

    focused.into()
}
