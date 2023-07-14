#![cfg_attr(feature = "ssr", allow(unused_variables, unused_imports))]

use crate::use_event_listener;
use cfg_if::cfg_if;
use leptos::ev::visibilitychange;
use leptos::*;

/// Reactively track `document.visibilityState`
///
/// ## Demo
///
/// [Link to Demo](https://github.com/Synphonyte/leptos-use/tree/main/examples/use_document_visibility)
///
/// ## Usage
///
/// ```
/// # use leptos::*;
/// # use leptos_use::use_document_visibility;
/// #
/// # #[component]
/// # fn Demo(cx: Scope) -> impl IntoView {
/// let visibility = use_document_visibility(cx);
/// #
/// # view! { cx, }
/// # }
/// ```
///
/// ## Server-Side Rendering
///
/// On the server this returns a `Signal` that always contains the value `web_sys::VisibilityState::Hidden`.
pub fn use_document_visibility(cx: Scope) -> Signal<web_sys::VisibilityState> {
    cfg_if! { if #[cfg(feature = "ssr")] {
        let inital_visibility = web_sys::VisibilityState::Hidden;
    } else {
        let inital_visibility = document().visibility_state();
    }}

    let (visibility, set_visibility) = create_signal(cx, inital_visibility);

    cfg_if! { if #[cfg(not(feature = "ssr"))] {
        let _ = use_event_listener(cx, document(), visibilitychange, move |_| {
            set_visibility.set(document().visibility_state());
        });
    }}

    visibility.into()
}
