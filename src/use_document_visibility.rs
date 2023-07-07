use crate::use_event_listener;
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
pub fn use_document_visibility(cx: Scope) -> Signal<web_sys::VisibilityState> {
    let (visibility, set_visibility) = create_signal(cx, document().visibility_state());

    let _ = use_event_listener(cx, document(), visibilitychange, move |_| {
        set_visibility.set(document().visibility_state());
    });

    visibility.into()
}
