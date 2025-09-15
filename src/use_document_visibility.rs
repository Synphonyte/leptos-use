#![cfg_attr(feature = "ssr", allow(unused_variables, unused_imports))]

use crate::use_event_listener;
use cfg_if::cfg_if;
use leptos::ev::visibilitychange;
use leptos::prelude::*;
use leptos::reactive::wrappers::read::Signal;

/// Reactively track `document.visibilityState`
///
/// ## Demo
///
/// [Link to Demo](https://github.com/Synphonyte/leptos-use/tree/main/examples/use_document_visibility)
///
/// ## Usage
///
/// ```
/// # use leptos::prelude::*;
/// # use leptos_use::use_document_visibility;
/// #
/// # #[component]
/// # fn Demo() -> impl IntoView {
/// let visibility = use_document_visibility();
/// #
/// # view! { }
/// # }
/// ```
///
/// ## Server-Side Rendering
///
/// > Make sure you follow the [instructions in Server-Side Rendering](https://leptos-use.rs/server_side_rendering.html).
///
/// On the server this returns a `Signal` that always contains the value `web_sys::VisibilityState::Hidden`.
pub fn use_document_visibility() -> Signal<web_sys::VisibilityState> {
    cfg_if! { if #[cfg(feature = "ssr")] {
        let inital_visibility = web_sys::VisibilityState::Hidden;
    } else {
        let inital_visibility = document().visibility_state();
    }}

    let (visibility, set_visibility) = signal(inital_visibility);

    cfg_if! { if #[cfg(not(feature = "ssr"))] {
        let _ = use_event_listener(document(), visibilitychange, move |_| {
            set_visibility.set(document().visibility_state());
        });
    }}

    visibility.into()
}
