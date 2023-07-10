use crate::use_event_listener;
use leptos::ev::{blur, focus};
use leptos::*;

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
/// # use leptos::*;
/// # use leptos_use::use_window_focus;
/// #
/// # #[component]
/// # fn Demo(cx: Scope) -> impl IntoView {
/// let focused = use_window_focus(cx);
/// #
/// # view! { cx, }
/// # }
/// ```
pub fn use_window_focus(cx: Scope) -> Signal<bool> {
    let (focused, set_focused) = create_signal(cx, document().has_focus().unwrap_or_default());

    let _ = use_event_listener(cx, window(), blur, move |_| set_focused.set(false));
    let _ = use_event_listener(cx, window(), focus, move |_| set_focused.set(true));

    focused.into()
}
