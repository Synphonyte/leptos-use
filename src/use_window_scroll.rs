use crate::use_event_listener_with_options;
use leptos::ev::scroll;
use leptos::*;
use web_sys::AddEventListenerOptions;

/// Reactive window scroll.
///
/// ## Demo
///
/// [Link to Demo](https://github.com/Synphonyte/leptos-use/tree/main/examples/use_window_scroll)
///
/// ## Usage
///
/// ```
/// # use leptos::*;
/// # use leptos_use::use_window_scroll;
/// #
/// # #[component]
/// # fn Demo(cx: Scope) -> impl IntoView {
/// let (x, y) = use_window_scroll(cx);
/// #
/// # view! { cx, }
/// # }
/// ```
pub fn use_window_scroll(cx: Scope) -> (Signal<f64>, Signal<f64>) {
    let (x, set_x) = create_signal(cx, window().scroll_x().unwrap_or_default());
    let (y, set_y) = create_signal(cx, window().scroll_y().unwrap_or_default());

    let mut options = AddEventListenerOptions::new();
    options.capture(false);
    options.passive(true);

    let _ = use_event_listener_with_options(
        cx,
        window(),
        scroll,
        move |_| {
            set_x.set(window().scroll_x().unwrap_or_default());
            set_y.set(window().scroll_y().unwrap_or_default());
        },
        options,
    );

    (x.into(), y.into())
}
