use crate::core::IntoElementMaybeSignal;
use crate::{use_event_listener_with_options, UseEventListenerOptions};
use default_struct_builder::DefaultBuilder;
use leptos::ev::{mouseenter, mouseleave};
use leptos::leptos_dom::helpers::TimeoutHandle;
use leptos::prelude::*;
use leptos::reactive::wrappers::read::Signal;

/// Reactive element's hover state.
///
/// ## Demo
///
/// [Link to Demo](https://github.com/Synphonyte/leptos-use/tree/main/examples/use_element_hover)
///
/// ## Usage
///
/// ```
/// # use leptos::prelude::*;
/// # use leptos::html::Button;
/// # use leptos_use::use_element_hover;
/// #
/// # #[component]
/// # fn Demo() -> impl IntoView {
/// let el = NodeRef::<Button>::new();
/// let is_hovered = use_element_hover(el);
///
/// view! {
///     <button node_ref=el>{ move || format!("{:?}", is_hovered.get()) }</button>
/// }
/// # }
/// ```
///
/// ## Server-Side Rendering
///
/// On the server this returns a `Signal` that always contains the value `false`.
pub fn use_element_hover<El, M>(el: El) -> Signal<bool>
where
    El: IntoElementMaybeSignal<web_sys::EventTarget, M>,
{
    use_element_hover_with_options(el, UseElementHoverOptions::default())
}

/// Version of [`use_element_hover`] that takes a `UseElementHoverOptions`. See [`use_element_hover`] for how to use.

#[cfg_attr(feature = "ssr", allow(unused_variables, unused_mut))]
pub fn use_element_hover_with_options<El, M>(
    el: El,
    options: UseElementHoverOptions,
) -> Signal<bool>
where
    El: IntoElementMaybeSignal<web_sys::EventTarget, M>,
{
    let UseElementHoverOptions {
        delay_enter,
        delay_leave,
    } = options;

    let (is_hovered, set_hovered) = signal(false);

    let timer = StoredValue::new(None::<TimeoutHandle>);

    let toggle = move |entering: bool| {
        #[cfg(not(feature = "ssr"))]
        {
            let delay = if entering { delay_enter } else { delay_leave };

            timer.update_value(|timer| {
                if let Some(handle) = timer.take() {
                    handle.clear();
                }
            });

            if delay > 0 {
                timer.set_value(
                    set_timeout_with_handle(
                        move || set_hovered.set(entering),
                        std::time::Duration::from_millis(delay),
                    )
                    .ok(),
                );
            } else {
                set_hovered.set(entering);
            }
        }
    };

    let listener_options = UseEventListenerOptions::default().passive(true);

    let el = el.into_element_maybe_signal();

    let _ = use_event_listener_with_options(
        el.clone(),
        mouseenter,
        move |_| toggle(true),
        listener_options,
    );

    let _ =
        use_event_listener_with_options(el, mouseleave, move |_| toggle(false), listener_options);

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
