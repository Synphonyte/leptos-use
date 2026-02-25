#![cfg_attr(feature = "ssr", allow(unused_variables, unused_imports))]

use crate::core::{IntoElementMaybeSignal, Position};
use crate::{UseEventListenerOptions, UseWindow, use_event_listener_with_options, use_window};
use default_struct_builder::DefaultBuilder;
use leptos::ev::{dragover, mousemove, touchend, touchmove, touchstart};
use leptos::prelude::*;
use std::convert::Infallible;
use std::marker::PhantomData;
use wasm_bindgen::{JsCast, JsValue};

/// Reactive mouse position
///
/// ## Demo
///
/// [Link to Demo](https://github.com/Synphonyte/leptos-use/tree/main/examples/use_mouse)
///
/// ## Basic Usage
///
/// ```
/// # use leptos::prelude::*;
/// # use leptos_use::{use_mouse, UseMouseReturn};
/// #
/// # #[component]
/// # fn Demo() -> impl IntoView {
/// let UseMouseReturn {
///     x, y, source_type, ..
/// } = use_mouse();
/// # view! { }
/// # }
/// ```
///
/// Touch is enabled by default. To only detect mouse changes, set `touch` to `false`.
/// The `dragover` event is used to track mouse position while dragging.
///
/// ```
/// # use leptos::prelude::*;
/// # use leptos_use::{use_mouse_with_options, UseMouseOptions, UseMouseReturn};
/// #
/// # #[component]
/// # fn Demo() -> impl IntoView {
/// let UseMouseReturn {
///     x, y, ..
/// } = use_mouse_with_options(
///     UseMouseOptions::default().touch(false)
/// );
/// # view! { }
/// # }
/// ```
///
/// ## Custom Extractor
///
/// It's also possible to provide a custom extractor to get the position from the events.
///
/// ```
/// # use leptos::prelude::*;
/// # use leptos::html::Div;
/// use web_sys::MouseEvent;
/// use leptos_use::{use_mouse_with_options, UseMouseOptions, UseMouseReturn, UseMouseEventExtractor, UseMouseCoordType};
///
/// #[derive(Clone)]
/// struct MyExtractor;
///
/// impl UseMouseEventExtractor for MyExtractor {
///     fn extract_mouse_coords(&self, event: &MouseEvent) -> Option<(f64, f64)> {
///         Some((event.offset_x() as f64, event.offset_y() as f64))
///     }
///
///     // don't implement fn extract_touch_coords to ignore touch events
/// }
///
/// #[component]
/// fn Demo() -> impl IntoView {
///     let element = NodeRef::<Div>::new();
///
///     let UseMouseReturn {
///         x, y, source_type, ..
///     } = use_mouse_with_options(
///         UseMouseOptions::default()
///             .target(element)
///             .coord_type(UseMouseCoordType::Custom(MyExtractor))
///     );
///     view! { <div node_ref=element></div> }
/// }
/// ```
///
/// ## Server-Side Rendering
///
/// > Make sure you follow the [instructions in Server-Side Rendering](https://leptos-use.rs/server_side_rendering.html).
///
/// On the server this returns simple `Signal`s with the `initial_value`s.
pub fn use_mouse() -> UseMouseReturn {
    use_mouse_with_options(Default::default())
}

/// Variant of [`use_mouse`] that accepts options. Please see [`use_mouse`] for how to use.
pub fn use_mouse_with_options<El, M, Ex>(options: UseMouseOptions<El, M, Ex>) -> UseMouseReturn
where
    El: IntoElementMaybeSignal<web_sys::EventTarget, M>,
    Ex: UseMouseEventExtractor + Clone + 'static,
{
    let (x, set_x) = signal(options.initial_value.x);
    let (y, set_y) = signal(options.initial_value.y);
    let (source_type, set_source_type) = signal(UseMouseSourceType::Unset);

    let mouse_handler = {
        let coord_type = options.coord_type.clone();

        move |event: web_sys::MouseEvent| {
            let result = coord_type.extract_mouse_coords(&event);

            if let Some((x, y)) = result {
                set_x.set(x);
                set_y.set(y);
                set_source_type.set(UseMouseSourceType::Mouse);
            }
        }
    };

    let drag_handler = {
        let mouse_handler = mouse_handler.clone();

        move |event: web_sys::DragEvent| {
            let js_value: &JsValue = event.as_ref();
            mouse_handler(js_value.clone().unchecked_into::<web_sys::MouseEvent>());
        }
    };

    let touch_handler = {
        let coord_type = options.coord_type.clone();

        move |event: web_sys::TouchEvent| {
            let touches = event.touches();
            if touches.length() > 0 {
                let result = coord_type.extract_touch_coords(
                    &touches
                        .get(0)
                        .expect("Just checked that there's at least on touch"),
                );

                if let Some((x, y)) = result {
                    set_x.set(x);
                    set_y.set(y);
                    set_source_type.set(UseMouseSourceType::Touch);
                }
            }
        }
    };

    let initial_value = options.initial_value;
    let reset = move || {
        set_x.set(initial_value.x);
        set_y.set(initial_value.y);
    };

    // TODO : event filters?

    #[cfg(not(feature = "ssr"))]
    {
        let target = options.target.into_element_maybe_signal();
        let event_listener_options = UseEventListenerOptions::default().passive(true);

        let _ = use_event_listener_with_options(
            target,
            mousemove,
            mouse_handler,
            event_listener_options,
        );
        let _ =
            use_event_listener_with_options(target, dragover, drag_handler, event_listener_options);

        if options.touch && !matches!(options.coord_type, UseMouseCoordType::Movement) {
            let _ = use_event_listener_with_options(
                target,
                touchstart,
                touch_handler.clone(),
                event_listener_options,
            );
            let _ = use_event_listener_with_options(
                target,
                touchmove,
                touch_handler,
                event_listener_options,
            );
            if options.reset_on_touch_ends {
                let _ = use_event_listener_with_options(
                    target,
                    touchend,
                    move |_| reset(),
                    event_listener_options,
                );
            }
        }
    }

    UseMouseReturn {
        x: x.into(),
        y: y.into(),
        set_x,
        set_y,
        source_type: source_type.into(),
    }
}

#[derive(DefaultBuilder)]
/// Options for [`use_mouse_with_options`].
pub struct UseMouseOptions<El, M, Ex>
where
    El: IntoElementMaybeSignal<web_sys::EventTarget, M>,
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

    #[builder(skip)]
    _marker: PhantomData<M>,
}

impl<M> Default for UseMouseOptions<UseWindow, M, Infallible>
where
    UseWindow: IntoElementMaybeSignal<web_sys::EventTarget, M>,
{
    fn default() -> Self {
        Self {
            coord_type: UseMouseCoordType::default(),
            target: use_window(),
            touch: true,
            reset_on_touch_ends: false,
            initial_value: Position { x: 0.0, y: 0.0 },
            _marker: PhantomData,
        }
    }
}

/// Defines how to get the coordinates from the event.
#[derive(Clone, Default)]
pub enum UseMouseCoordType<E: UseMouseEventExtractor + Clone> {
    #[default]
    Page,
    Client,
    Screen,
    Movement,
    Custom(E),
}

/// Trait to implement if you want to specify a custom extractor
#[allow(unused_variables)]
pub trait UseMouseEventExtractor {
    /// Return the coordinates from mouse events (`Some(x, y)`) or `None`
    fn extract_mouse_coords(&self, event: &web_sys::MouseEvent) -> Option<(f64, f64)> {
        None
    }

    /// Return the coordinates from touches (`Some(x, y)`) or `None`
    fn extract_touch_coords(&self, touch: &web_sys::Touch) -> Option<(f64, f64)> {
        None
    }
}

impl<E: UseMouseEventExtractor + Clone> UseMouseEventExtractor for UseMouseCoordType<E> {
    fn extract_mouse_coords(&self, event: &web_sys::MouseEvent) -> Option<(f64, f64)> {
        match self {
            UseMouseCoordType::Page => Some((event.page_x() as f64, event.page_y() as f64)),
            UseMouseCoordType::Client => Some((event.client_x() as f64, event.client_y() as f64)),
            UseMouseCoordType::Screen => Some((event.screen_x() as f64, event.screen_y() as f64)),
            UseMouseCoordType::Movement => {
                Some((event.movement_x() as f64, event.movement_y() as f64))
            }
            UseMouseCoordType::Custom(extractor) => extractor.extract_mouse_coords(event),
        }
    }

    fn extract_touch_coords(&self, touch: &web_sys::Touch) -> Option<(f64, f64)> {
        match self {
            UseMouseCoordType::Page => Some((touch.page_x() as f64, touch.page_y() as f64)),
            UseMouseCoordType::Client => Some((touch.client_x() as f64, touch.client_y() as f64)),
            UseMouseCoordType::Screen => Some((touch.screen_x() as f64, touch.client_y() as f64)),
            UseMouseCoordType::Movement => None,
            UseMouseCoordType::Custom(extractor) => extractor.extract_touch_coords(touch),
        }
    }
}

impl UseMouseEventExtractor for Infallible {
    fn extract_mouse_coords(&self, _: &web_sys::MouseEvent) -> Option<(f64, f64)> {
        unreachable!()
    }

    fn extract_touch_coords(&self, _: &web_sys::Touch) -> Option<(f64, f64)> {
        unreachable!()
    }
}

/// Return type of [`use_mouse`].
pub struct UseMouseReturn {
    /// X coordinate of the mouse pointer / touch
    pub x: Signal<f64>,
    /// Y coordinate of the mouse pointer / touch
    pub y: Signal<f64>,
    /// Sets the value of `x`. This does not actually move the mouse cursor.
    pub set_x: WriteSignal<f64>,
    /// Sets the value of `y`. This does not actually move the mouse cursor.
    pub set_y: WriteSignal<f64>,
    /// Identifies the source of the reported coordinates
    pub source_type: Signal<UseMouseSourceType>,
}

/// Identifies the source of the reported coordinates
#[derive(Copy, Clone, Debug, PartialEq, Eq)]
pub enum UseMouseSourceType {
    /// coordinates come from mouse movement
    Mouse,
    /// coordinates come from touch
    Touch,
    /// Initially before any event has been recorded the source type is unset
    Unset,
}
