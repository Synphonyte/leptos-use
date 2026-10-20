use crate::{
    IntoElementMaybeSignal, Position, UseMouseCoordType, UseMouseEventExtractor, UseWindow,
    use_window,
};
use default_struct_builder::DefaultBuilder;
use std::convert::Infallible;
use std::marker::PhantomData;

/// Options for [`use_mouse_in_element_with_options`](crate::use_mouse_in_element_with_options).
#[derive(DefaultBuilder)]
pub struct UseMouseInElementOptions<El, M, Ex>
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

    /// If `true` updates the `element_x` and `element_y` signals even if the
    /// mouse is outside of the element. If `false` it doesn't update them when outside.
    /// Defaults to `true`.
    pub(crate) handle_outside: bool,

    #[builder(skip)]
    _marker: PhantomData<M>,
}

impl<M> Default for UseMouseInElementOptions<UseWindow, M, Infallible>
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
            handle_outside: true,
            _marker: PhantomData,
        }
    }
}
