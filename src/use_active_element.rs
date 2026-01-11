#![cfg_attr(feature = "ssr", allow(unused_variables, unused_imports))]

use crate::core::{OptionLocalRwSignal, OptionLocalSignal};
use crate::{UseEventListenerOptions, use_document, use_event_listener_with_options, use_window};
use leptos::ev::{blur, focus};
use leptos::prelude::*;

/// Reactive `document.activeElement`
///
/// ## Demo
///
/// [Link to Demo](https://github.com/Synphonyte/leptos-use/tree/main/examples/use_active_element)
///
/// ## Usage
///
/// ```
/// # use leptos::prelude::*;
/// # use leptos::logging::log;
/// # use leptos_use::use_active_element;
/// #
/// # #[component]
/// # fn Demo() -> impl IntoView {
/// let active_element = use_active_element();
///
/// Effect::new(move || {
///     log!("focus changed to {:?}", active_element.get());
/// });
/// #
/// # view! { }
/// # }
/// ```
///
/// ## Server-Side Rendering
///
/// > Make sure you follow the [instructions in Server-Side Rendering](https://leptos-use.rs/server_side_rendering.html).
///
/// On the server this returns a `Signal` that always contains the value `None`.
pub fn use_active_element() -> OptionLocalSignal<web_sys::Element> {
    let get_active_element = move || use_document().active_element();

    let active_element = OptionLocalRwSignal::new();
    if let Some(cur_active_element) = get_active_element() {
        active_element.set(Some(cur_active_element));
    }

    let listener_options = UseEventListenerOptions::default().capture(true);

    let _ = use_event_listener_with_options(
        use_window(),
        blur,
        move |event| {
            if event.related_target().is_some() {
                return;
            }

            active_element.update(|el| *el = get_active_element());
        },
        listener_options,
    );

    let _ = use_event_listener_with_options(
        use_window(),
        focus,
        move |_| {
            active_element.update(|el| *el = get_active_element());
        },
        listener_options,
    );

    active_element.read_only()
}
