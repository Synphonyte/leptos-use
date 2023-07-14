#![cfg_attr(feature = "ssr", allow(unused_variables, unused_imports))]

use crate::use_event_listener_with_options;
use cfg_if::cfg_if;
use leptos::ev::{blur, focus};
use leptos::html::{AnyElement, ToHtmlElement};
use leptos::*;
use web_sys::AddEventListenerOptions;

/// Reactive `document.activeElement`
///
/// ## Demo
///
/// [Link to Demo](https://github.com/Synphonyte/leptos-use/tree/main/examples/use_active_element)
///
/// ## Usage
///
/// ```
/// # use leptos::*;
/// use leptos_use::use_active_element;
/// #
/// # #[component]
/// # fn Demo(cx: Scope) -> impl IntoView {
/// let active_element = use_active_element(cx);
///
/// create_effect(cx, move |_| {
///     log!("focus changed to {:?}", active_element.get());
/// });
/// #
/// # view! { cx, }
/// # }
/// ```
///
/// ## Server-Side Rendering
///
/// On the server this returns a `Signal` that always contains the value `None`.
pub fn use_active_element(cx: Scope) -> Signal<Option<HtmlElement<AnyElement>>> {
    cfg_if! { if #[cfg(feature = "ssr")] {
        let get_active_element = || { None };
    } else {
        let get_active_element = move || {
            document()
                .active_element()
                .map(|el| el.to_leptos_element(cx))
        };
    }}

    let (active_element, set_active_element) = create_signal(cx, get_active_element());

    cfg_if! { if #[cfg(not(feature = "ssr"))] {
        let mut listener_options = AddEventListenerOptions::new();
        listener_options.capture(true);

        let _ = use_event_listener_with_options(
            cx,
            window(),
            blur,
            move |event| {
                if event.related_target().is_some() {
                    return;
                }

                set_active_element.update(|el| *el = get_active_element());
            },
            listener_options.clone(),
        );

        let _ = use_event_listener_with_options(
            cx,
            window(),
            focus,
            move |_| {
                set_active_element.update(|el| *el = get_active_element());
            },
            listener_options,
        );
    }}

    active_element.into()
}
