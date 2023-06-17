use crate::use_event_listener_with_options;
use leptos::ev::{blur, focus};
use leptos::html::{AnyElement, ToHtmlElement};
use leptos::*;
use web_sys::AddEventListenerOptions;

///
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
///     log!("focus changed to {:?}", active_element());
/// });
/// #
/// # view! { cx, }
/// # }
/// ```

pub fn use_active_element(cx: Scope) -> Signal<Option<HtmlElement<AnyElement>>> {
    let get_active_element = move || {
        document()
            .active_element()
            .map(|el| el.to_leptos_element(cx))
    };

    let (active_element, set_active_element) = create_signal(cx, get_active_element());

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

    active_element.into()
}
