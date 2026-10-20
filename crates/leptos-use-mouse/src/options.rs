use crate::{IntoElementMaybeSignal, Position, UseMouseEventExtractor, UseWindow, use_window};
use default_struct_builder::DefaultBuilder;
use std::convert::Infallible;
use std::marker::PhantomData;

#[derive(DefaultBuilder)]
/// Options for [`use_mouse_with_options`](crate::use_mouse_with_options).
pub struct UseMouseOptions<El, M, Ex>
where
    El: IntoElementMaybeSignal<web_sys::EventTarget, M>,
    Ex: UseMouseEventExtractor + Clone,
{
    /// How to extract the x, y coordinates from mouse events or touches
    pub(crate) coord_type: UseMouseCoordType<Ex>,

    /// Listen events on `target` element. Defaults to `window`
    pub(crate) target: El,

    /// Listen to `touchmove` events. Defaults to `true`.
    pub(crate) touch: bool,

    /// Reset to initial value when `touchend` event fired. Defaults to `false`
    pub(crate) reset_on_touch_ends: bool,

    /// Initial values. Defaults to `{x: 0.0, y: 0.0}`.
    pub(crate) initial_value: Position,

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

impl<E: UseMouseEventExtractor + Clone> UseMouseEventExtractor for UseMouseCoordType<E> {
    #[allow(clippy::unnecessary_cast)]
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
            UseMouseCoordType::Screen => Some((touch.screen_x() as f64, touch.screen_y() as f64)),
            UseMouseCoordType::Movement => None,
            UseMouseCoordType::Custom(extractor) => extractor.extract_touch_coords(touch),
        }
    }
}
