use crate::core::{ElementMaybeSignal, Position};
use crate::{
    use_mouse_with_options, use_window, UseMouseCoordType, UseMouseEventExtractor, UseMouseOptions,
    UseMouseReturn, UseMouseSourceType, UseWindow,
};
use cfg_if::cfg_if;
use default_struct_builder::DefaultBuilder;
use leptos::prelude::*;
use std::convert::Infallible;
use std::marker::PhantomData;

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
/// # use leptos_use::{use_mouse_in_element, UseMouseInElementReturn};
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
/// ## Server-Side Rendering
///
/// On the server this returns simple Signals with the `initial_value` for `x` and `y`,
/// no-op for `stop`, `is_outside = true` and `0.0` for the rest of the signals.
pub fn use_mouse_in_element<El, T>(target: El) -> UseMouseInElementReturn<impl Fn() + Clone>
where
    El: Into<ElementMaybeSignal<T, web_sys::Element>> + Clone,
    T: Into<web_sys::Element> + Clone + 'static,
{
    use_mouse_in_element_with_options(target, Default::default())
}

/// Version of [`use_mouse_in_element`] that takes a `UseMouseInElementOptions`. See [`use_mouse_in_element`] for how to use.
pub fn use_mouse_in_element_with_options<El, T, OptEl, OptT, OptEx>(
    target: El,
    options: UseMouseInElementOptions<OptEl, OptT, OptEx>,
) -> UseMouseInElementReturn<impl Fn() + Clone>
where
    El: Into<ElementMaybeSignal<T, web_sys::Element>> + Clone,
    T: Into<web_sys::Element> + Clone + 'static,
    OptEl: Into<ElementMaybeSignal<OptT, web_sys::EventTarget>> + Clone,
    OptT: Into<web_sys::EventTarget> + Clone + 'static,
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

    cfg_if! { if #[cfg(feature = "ssr")] {
        let stop = || ();

        let _ = handle_outside;

        let _ = set_element_x;
        let _ = set_element_y;
        let _ = set_element_position_x;
        let _ = set_element_position_y;
        let _ = set_element_width;
        let _ = set_element_height;
        let _ = set_outside;
        let _ = target;
    } else {
        use crate::use_event_listener;
        use leptos::ev::mouseleave;

        let target = target.into();
        let window = window();

        let effect = Effect::watch(
            move || (target.get(), x.get(), y.get()),
            move |(el, x, y), _, _| {
                if let Some(el) = el {
                    let el: web_sys::Element = el.clone().into();
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
                            || el_x <= 0.0
                            || el_y <= 0.0
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

        let stop = move || effect.stop();

        let _ = use_event_listener(document(), mouseleave, move |_| set_outside.set(true));
    }}

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

/// Options for [`use_mouse_in_element_with_options`].
#[derive(DefaultBuilder)]
pub struct UseMouseInElementOptions<El, T, Ex>
where
    El: Clone + Into<ElementMaybeSignal<T, web_sys::EventTarget>>,
    T: Into<web_sys::EventTarget> + Clone + 'static,
    Ex: UseMouseEventExtractor + Clone,
{
    /// How to extract the x, y coordinates from mouse events or touches
    coord_type: UseMouseCoordType<Ex>,

    /// Listen events on `target` element. Defaults to `window`
    target: El,

    /// Listen to `touchmove` events. Defaults to `true`.
    touch: bool,

    /// Reset to initial value when `touchend` event fired. Defaults to `false`
    reset_on_touch_ends: bool,

    /// Initial values. Defaults to `{x: 0.0, y: 0.0}`.
    initial_value: Position,

    /// If `true` updates the `element_x` and `element_y` signals even if the
    /// mouse is outside of the element. If `false` it doesn't update them when outside.
    /// Defaults to `true`.
    handle_outside: bool,

    #[builder(skip)]
    _marker: PhantomData<T>,
}

impl Default for UseMouseInElementOptions<UseWindow, web_sys::Window, Infallible> {
    fn default() -> Self {
        Self {
            coord_type: UseMouseCoordType::default(),
            target: use_window(),
            touch: true,
            reset_on_touch_ends: false,
            initial_value: Position { x: 0.0, y: 0.0 },
            handle_outside: true,
            _marker: PhantomData,
        }
    }
}

/// Return type of [`use_mouse_in_element`].
pub struct UseMouseInElementReturn<F>
where
    F: Fn() + Clone,
{
    /// X coordinate of the mouse pointer / touch
    pub x: Signal<f64>,

    /// Y coordinate of the mouse pointer / touch
    pub y: Signal<f64>,

    /// Identifies the source of the reported coordinates
    pub source_type: Signal<UseMouseSourceType>,

    /// X coordinate of the pointer relative to the left edge of the element
    pub element_x: Signal<f64>,

    /// Y coordinate of the pointer relative to the top edge of the element
    pub element_y: Signal<f64>,

    /// X coordinate of the element relative to the left edge of the document
    pub element_position_x: Signal<f64>,

    /// Y coordinate of the element relative to the top edge of the document
    pub element_position_y: Signal<f64>,

    /// Width of the element
    pub element_width: Signal<f64>,

    /// Height of the element
    pub element_height: Signal<f64>,

    /// `true` if the mouse is outside of the element
    pub is_outside: Signal<bool>,

    /// Stop watching
    pub stop: F,
}
