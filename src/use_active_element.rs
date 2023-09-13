#![cfg_attr(feature = "ssr", allow(unused_variables, unused_imports))]

use crate::{use_event_listener_with_options, UseEventListenerOptions};
use cfg_if::cfg_if;
use leptos::ev::{blur, focus};
use leptos::html::{AnyElement, ToHtmlElement};
use leptos::*;

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
/// # use leptos::logging::log;
/// use leptos_use::use_active_element;
/// #
/// # #[component]
/// # fn Demo() -> impl IntoView {
/// let active_element = use_active_element();
///
/// create_effect(move |_| {
///     log!("focus changed to {:?}", active_element.get());
/// });
/// #
/// # view! { }
/// # }
/// ```
///
/// ## Server-Side Rendering
///
/// On the server this returns a `Signal` that always contains the value `None`.
pub fn use_active_element() -> Signal<Option<HtmlElement<AnyElement>>> {
    cfg_if! { if #[cfg(feature = "ssr")] {
        let get_active_element = || { None };
    } else {
        let get_active_element = move || {
            document()
                .active_element()
                .map(|el| el.to_leptos_element())
        };
    }}

    let (active_element, set_active_element) = create_signal(get_active_element());

    cfg_if! { if #[cfg(not(feature = "ssr"))] {
        let listener_options = UseEventListenerOptions::default()
            .capture(true);

        let _ = use_event_listener_with_options(
                        window(),
            blur,
            move |event| {
                if event.related_target().is_some() {
                    return;
                }

                set_active_element.update(|el| *el = get_active_element());
            },
            listener_options,
        );

        let _ = use_event_listener_with_options(
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
