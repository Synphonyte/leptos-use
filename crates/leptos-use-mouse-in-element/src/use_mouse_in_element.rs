use crate::{
    IntoElementMaybeSignal, UseMouseEventExtractor, UseMouseInElementOptions,
    UseMouseInElementReturn, UseMouseOptions, UseMouseReturn, use_mouse_with_options,
};
use leptos::prelude::*;

/// Reactive mouse position related to an element.
///
/// ## Demo
///
/// [Link to Demo](https://github.com/Synphonyte/leptos-use/tree/main/examples/use_mouse_in_element)
///
/// ## Usage
///
/// ```
/// # use leptos::prelude::*;
/// # use leptos::html::Div;
/// # use leptos_use_mouse_in_element::{use_mouse_in_element, UseMouseInElementReturn};
/// #
/// # #[component]
/// # fn Demo() -> impl IntoView {
/// let target = NodeRef::<Div>::new();
/// let UseMouseInElementReturn { x, y, is_outside, .. } = use_mouse_in_element(target);
///
/// view! {
///     <div node_ref=target>
///         <h1>Hello world</h1>
///     </div>
/// }
/// # }
/// ```
///
/// ## SendWrapped Return
///
/// The returned closure `stop` is a sendwrapped function. It can
/// only be called from the same thread that called `use_mouse_in_element`.
///
/// ## Server-Side Rendering
///
/// > Make sure you follow the [instructions in Server-Side Rendering](https://leptos-use.rs/server_side_rendering.html).
///
/// On the server this returns simple Signals with the `initial_value` for `x` and `y`,
/// no-op for `stop`, `is_outside = true` and `0.0` for the rest of the signals.
pub fn use_mouse_in_element<El, M>(
    target: El,
) -> UseMouseInElementReturn<impl Fn() + Clone + Send + Sync>
where
    El: IntoElementMaybeSignal<web_sys::Element, M>,
{
    use_mouse_in_element_with_options(target, Default::default())
}

/// Version of [`use_mouse_in_element`] that takes a `UseMouseInElementOptions`. See [`use_mouse_in_element`] for how to use.
pub fn use_mouse_in_element_with_options<El, M, OptEl, OptM, OptEx>(
    target: El,
    options: UseMouseInElementOptions<OptEl, OptM, OptEx>,
) -> UseMouseInElementReturn<impl Fn() + Clone + Send + Sync>
where
    El: IntoElementMaybeSignal<web_sys::Element, M>,
    OptEl: IntoElementMaybeSignal<web_sys::EventTarget, OptM>,
    OptEx: UseMouseEventExtractor + Clone + 'static,
{
    let UseMouseInElementOptions {
        coord_type,
        target: use_mouse_target,
        touch,
        reset_on_touch_ends,
        initial_value,
        handle_outside,
        ..
    } = options;

    let UseMouseReturn {
        x, y, source_type, ..
    } = use_mouse_with_options(
        UseMouseOptions::default()
            .coord_type(coord_type)
            .target(use_mouse_target)
            .touch(touch)
            .reset_on_touch_ends(reset_on_touch_ends)
            .initial_value(initial_value),
    );

    let (element_x, set_element_x) = signal(0.0);
    let (element_y, set_element_y) = signal(0.0);
    let (element_position_x, set_element_position_x) = signal(0.0);
    let (element_position_y, set_element_position_y) = signal(0.0);
    let (element_width, set_element_width) = signal(0.0);
    let (element_height, set_element_height) = signal(0.0);
    let (is_outside, set_outside) = signal(true);

    let stop;

    #[cfg(feature = "ssr")]
    {
        stop = || ();

        let _ = handle_outside;

        let _ = set_element_x;
        let _ = set_element_y;
        let _ = set_element_position_x;
        let _ = set_element_position_y;
        let _ = set_element_width;
        let _ = set_element_height;
        let _ = set_outside;
        let _ = target;
    }

    #[cfg(not(feature = "ssr"))]
    {
        use crate::{UseEventListenerOptions, sendwrap_fn, use_event_listener_with_options};
        use leptos::ev::mouseleave;

        let target = target.into_element_maybe_signal();
        let window = window();

        let effect = Effect::watch(
            move || (target.get(), x.get(), y.get()),
            move |(el, x, y), _, _| {
                if let Some(el) = el {
                    let el = el.clone();
                    let rect = el.get_bounding_client_rect();
                    let left = rect.left();
                    let top = rect.top();
                    let width = rect.width();
                    let height = rect.height();

                    set_element_position_x.set(left + window.page_x_offset().unwrap_or_default());
                    set_element_position_y.set(top + window.page_y_offset().unwrap_or_default());

                    set_element_height.set(height);
                    set_element_width.set(width);

                    let el_x = *x - element_position_x.get_untracked();
                    let el_y = *y - element_position_y.get_untracked();

                    set_outside.set(
                        width == 0.0
                            || height == 0.0
                            || el_x < 0.0
                            || el_y < 0.0
                            || el_x > width
                            || el_y > height,
                    );

                    if handle_outside || !is_outside.get_untracked() {
                        set_element_x.set(el_x);
                        set_element_y.set(el_y);
                    }
                }
            },
            false,
        );

        stop = sendwrap_fn!(move || effect.stop());

        let _ = use_event_listener_with_options(
            document(),
            mouseleave,
            move |_| set_outside.set(true),
            UseEventListenerOptions::default().passive(true),
        );
    }

    UseMouseInElementReturn {
        x,
        y,
        source_type,
        element_x: element_x.into(),
        element_y: element_y.into(),
        element_position_x: element_position_x.into(),
        element_position_y: element_position_y.into(),
        element_width: element_width.into(),
        element_height: element_height.into(),
        is_outside: is_outside.into(),
        stop,
    }
}
