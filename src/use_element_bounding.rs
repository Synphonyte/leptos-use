use crate::core::ElementMaybeSignal;
use crate::{
    use_event_listener_with_options, use_resize_observer, use_window, UseEventListenerOptions,
};
use default_struct_builder::DefaultBuilder;
use leptos::ev::{resize, scroll};
use leptos::*;

/// Reactive [bounding box](https://developer.mozilla.org/en-US/docs/Web/API/Element/getBoundingClientRect) of an HTML element
///
/// ## Demo
///
/// [Link to Demo](https://github.com/Synphonyte/leptos-use/tree/main/examples/use_element_bounding)
///
/// ## Usage
///
/// ```
/// # use leptos::*;
/// # use leptos::html::Div;
/// # use leptos_use::{use_element_bounding, UseElementBoundingReturn};
/// #
/// # #[component]
/// # fn Demo() -> impl IntoView {
/// let el = create_node_ref::<Div>();
/// let UseElementBoundingReturn {
///     x, y, top,right,bottom,left, width, height, ..
/// } = use_element_bounding(el);
///
/// view! { <div node_ref=el></div> }
/// # }
/// ```
pub fn use_element_bounding<El, T>(target: El) -> UseElementBoundingReturn<impl Fn() + Clone>
where
    El: Into<ElementMaybeSignal<T, web_sys::Element>> + Clone,
    T: Into<web_sys::Element> + Clone + 'static,
{
    use_element_bounding_with_options(target, UseElementBoundingOptions::default())
}

/// Version of [`use_element_bounding`] that takes a `UseElementBoundingOptions`. See [`use_element_bounding`] for how to use.
pub fn use_element_bounding_with_options<El, T>(
    target: El,
    options: UseElementBoundingOptions,
) -> UseElementBoundingReturn<impl Fn() + Clone>
where
    El: Into<ElementMaybeSignal<T, web_sys::Element>> + Clone,
    T: Into<web_sys::Element> + Clone + 'static,
{
    let UseElementBoundingOptions {
        reset,
        window_resize,
        window_scroll,
        immediate,
    } = options;

    let (height, set_height) = create_signal(0.0);
    let (width, set_width) = create_signal(0.0);
    let (left, set_left) = create_signal(0.0);
    let (right, set_right) = create_signal(0.0);
    let (top, set_top) = create_signal(0.0);
    let (bottom, set_bottom) = create_signal(0.0);
    let (x, set_x) = create_signal(0.0);
    let (y, set_y) = create_signal(0.0);

    let target = target.into();

    let update = {
        let target = target.clone();

        move || {
            let el = target.get_untracked();

            if let Some(el) = el {
                let rect = el.into().get_bounding_client_rect();

                set_height.set(rect.height());
                set_width.set(rect.width());
                set_left.set(rect.x());
                set_right.set(rect.x() + rect.width());
                set_top.set(rect.y());
                set_bottom.set(rect.y() + rect.height());
                set_x.set(rect.x());
                set_y.set(rect.y());
            } else if reset {
                set_height.set(0.0);
                set_width.set(0.0);
                set_left.set(0.0);
                set_right.set(0.0);
                set_top.set(0.0);
                set_bottom.set(0.0);
                set_x.set(0.0);
                set_y.set(0.0);
            }
        }
    };

    use_resize_observer(target.clone(), {
        let update = update.clone();

        move |_, _| {
            update();
        }
    });

    let _ = watch(
        move || target.get(),
        {
            let update = update.clone();
            move |_, _, _| {
                update();
            }
        },
        false,
    );

    if window_scroll {
        let _ = use_event_listener_with_options(
            use_window(),
            scroll,
            {
                let update = update.clone();
                move |_| update()
            },
            UseEventListenerOptions::default()
                .capture(true)
                .passive(true),
        );
    }

    if window_resize {
        let _ = use_event_listener_with_options(
            use_window(),
            resize,
            {
                let update = update.clone();
                move |_| update()
            },
            UseEventListenerOptions::default().passive(true),
        );
    }

    if immediate {
        update();
    }

    UseElementBoundingReturn {
        height: height.into(),
        width: width.into(),
        left: left.into(),
        right: right.into(),
        top: top.into(),
        bottom: bottom.into(),
        x: x.into(),
        y: y.into(),
        update,
    }
}

/// Options for [`use_element_bounding_with_options`].
#[derive(DefaultBuilder)]
pub struct UseElementBoundingOptions {
    /// Reset values to 0 on component disposal
    ///
    /// Default: `true`
    pub reset: bool,

    /// Listen to window resize event
    ///
    /// Default: `true`
    pub window_resize: bool,

    /// Listen to window scroll event
    ///
    /// Default: `true`
    pub window_scroll: bool,

    /// Immediately call update
    ///
    /// Default: `true`
    pub immediate: bool,
}

impl Default for UseElementBoundingOptions {
    fn default() -> Self {
        Self {
            reset: true,
            window_resize: true,
            window_scroll: true,
            immediate: true,
        }
    }
}

/// Return type of [`use_element_bounding`].
pub struct UseElementBoundingReturn<F>
where
    F: Fn() + Clone,
{
    /// Reactive version of [`BoudingClientRect.height`](https://developer.mozilla.org/en-US/docs/Web/API/DOMRectReadOnly/height)
    pub height: Signal<f64>,
    /// Reactive version of [`BoudingClientRect.width`](https://developer.mozilla.org/en-US/docs/Web/API/DOMRectReadOnly/width)
    pub width: Signal<f64>,
    /// Reactive version of [`BoudingClientRect.left`](https://developer.mozilla.org/en-US/docs/Web/API/DOMRectReadOnly/left)
    pub left: Signal<f64>,
    /// Reactive version of [`BoudingClientRect.right`](https://developer.mozilla.org/en-US/docs/Web/API/DOMRectReadOnly/right)
    pub right: Signal<f64>,
    /// Reactive version of [`BoudingClientRect.top`](https://developer.mozilla.org/en-US/docs/Web/API/DOMRectReadOnly/top)
    pub top: Signal<f64>,
    /// Reactive version of [`BoudingClientRect.bottom`](https://developer.mozilla.org/en-US/docs/Web/API/DOMRectReadOnly/bottom)
    pub bottom: Signal<f64>,
    /// Reactive version of [`BoudingClientRect.x`](https://developer.mozilla.org/en-US/docs/Web/API/DOMRectReadOnly/x)
    pub x: Signal<f64>,
    /// Reactive version of [`BoudingClientRect.y`](https://developer.mozilla.org/en-US/docs/Web/API/DOMRectReadOnly/y)
    pub y: Signal<f64>,
    /// Function to re-evaluate `get_bounding_client_rect()` and update the signals.
    pub update: F,
}
