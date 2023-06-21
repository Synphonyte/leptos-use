use crate::core::ElementMaybeSignal;
use crate::use_event_listener_with_options;
use default_struct_builder::DefaultBuilder;
use leptos::ev::{mouseenter, mouseleave};
use leptos::leptos_dom::helpers::TimeoutHandle;
use leptos::*;
use std::time::Duration;
use web_sys::AddEventListenerOptions;

/// Reactive element's hover state.
///
/// ## Demo
///
/// [Link to Demo](https://github.com/Synphonyte/leptos-use/tree/main/examples/use_element_hover)
///
/// ## Usage
///
/// ```
/// # use leptos::*;
/// use leptos_use::use_element_hover;
/// #
/// # #[component]
/// # fn Demo(cx: Scope) -> impl IntoView {
/// let el = create_node_ref(cx);
/// let is_hovered = use_element_hover(cx, el);
///
/// view! { cx,
///     <button node_ref=el>{ move || format!("{:?}", is_hovered.get()) }</button>
/// }
/// # }
/// ```

pub fn use_element_hover<El, T>(cx: Scope, el: El) -> Signal<bool>
where
    El: Clone,
    (Scope, El): Into<ElementMaybeSignal<T, web_sys::EventTarget>>,
    T: Into<web_sys::EventTarget> + Clone + 'static,
{
    use_element_hover_with_options(cx, el, UseElementHoverOptions::default())
}

/// Version of [`use_element_hover`] that takes a `UseElementHoverOptions`. See [`use_element_hover`] for how to use.

pub fn use_element_hover_with_options<El, T>(
    cx: Scope,
    el: El,
    options: UseElementHoverOptions,
) -> Signal<bool>
where
    El: Clone,
    (Scope, El): Into<ElementMaybeSignal<T, web_sys::EventTarget>>,
    T: Into<web_sys::EventTarget> + Clone + 'static,
{
    let UseElementHoverOptions {
        delay_enter,
        delay_leave,
    } = options;

    let (is_hovered, set_hovered) = create_signal(cx, false);

    let mut timer: Option<TimeoutHandle> = None;

    let mut toggle = move |entering: bool| {
        let delay = if entering { delay_enter } else { delay_leave };

        if let Some(handle) = timer.take() {
            handle.clear();
        }

        if delay > 0 {
            timer = set_timeout_with_handle(
                move || set_hovered.set(entering),
                Duration::from_millis(delay),
            )
            .ok();
        } else {
            set_hovered.set(entering);
        }
    };

    let mut listener_options = AddEventListenerOptions::new();
    listener_options.passive(true);

    let _ = use_event_listener_with_options(
        cx,
        el.clone(),
        mouseenter,
        move |_| toggle(true),
        listener_options.clone(),
    );

    let _ = use_event_listener_with_options(
        cx,
        el,
        mouseleave,
        move |_| toggle(false),
        listener_options,
    );

    is_hovered.into()
}

/// Options for [`use_element_hover_with_options`].
#[derive(DefaultBuilder, Default)]
pub struct UseElementHoverOptions {
    /// The time in ms the mouse has to be hovered over the element before the signal is changed to `true`. Defaults to `0`.
    delay_enter: u64,

    /// The time in ms after the mouse has left the element before the signal is changed to `false`. Defaults to `0`.
    delay_leave: u64,
}
