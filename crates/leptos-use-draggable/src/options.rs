use crate::{IntoElementMaybeSignal, MaybeRwSignal, PointerType, Position, UseWindow, use_window};
use default_struct_builder::DefaultBuilder;
use leptos::reactive::wrappers::read::Signal;
use std::marker::PhantomData;
use std::sync::Arc;
use wasm_bindgen::JsCast;
use web_sys::PointerEvent;

/// Options for [`use_draggable_with_options`](crate::use_draggable_with_options).
#[derive(DefaultBuilder)]
pub struct UseDraggableOptions<DragEl, DragM, HandleEl, HandleM>
where
    DragEl: IntoElementMaybeSignal<web_sys::EventTarget, DragM>,
    HandleEl: IntoElementMaybeSignal<web_sys::EventTarget, HandleM>,
{
    /// Only start the dragging when click on the element directly. Defaults to `false`.
    #[builder(into)]
    pub(crate) exact: Signal<bool>,

    /// Prevent events defaults. Defaults to `false`.
    #[builder(into)]
    pub(crate) prevent_default: Signal<bool>,

    /// Prevent events propagation. Defaults to `false`.
    #[builder(into)]
    pub(crate) stop_propagation: Signal<bool>,

    /// Element to attach `pointermove` and `pointerup` events to. Defaults to `window`.
    pub(crate) dragging_element: DragEl,

    /// Handle that triggers the drag event. Defaults to `target`.
    pub(crate) handle: Option<HandleEl>,

    /// Pointer types that listen to. Defaults to `[Mouse, Touch, Pen]`.
    pub(crate) pointer_types: Vec<PointerType>,

    /// Initial position of the element. Defaults to `{ x: 0, y: 0 }`.
    #[builder(into)]
    pub(crate) initial_value: MaybeRwSignal<Position>,

    /// Computes the initial offset of the target element for drag positioning.
    /// Defaults to using its bounding client rectangle's left and top values.
    pub(crate) target_offset: Arc<dyn Fn(web_sys::EventTarget) -> (f64, f64)>,

    /// Callback when the dragging starts. Return `false` to prevent dragging.
    pub(crate) on_start: Arc<dyn Fn(UseDraggableCallbackArgs) -> bool + Send + Sync>,

    /// Callback during dragging.
    pub(crate) on_move: Arc<dyn Fn(UseDraggableCallbackArgs) + Send + Sync>,

    /// Callback when dragging end.
    pub(crate) on_end: Arc<dyn Fn(UseDraggableCallbackArgs) + Send + Sync>,

    #[builder(skip)]
    _marker1: PhantomData<DragM>,
    #[builder(skip)]
    _marker2: PhantomData<HandleM>,
}

impl<DragM, HandleM> Default
    for UseDraggableOptions<UseWindow, DragM, Option<web_sys::EventTarget>, HandleM>
where
    UseWindow: IntoElementMaybeSignal<web_sys::EventTarget, DragM>,
    Option<web_sys::EventTarget>: IntoElementMaybeSignal<web_sys::EventTarget, HandleM>,
{
    fn default() -> Self {
        Self {
            exact: Signal::default(),
            prevent_default: Signal::default(),
            stop_propagation: Signal::default(),
            dragging_element: use_window(),
            handle: None,
            pointer_types: vec![PointerType::Mouse, PointerType::Touch, PointerType::Pen],
            initial_value: MaybeRwSignal::default(),
            target_offset: Arc::new(|target: web_sys::EventTarget| {
                let target: web_sys::Element = target.unchecked_into();
                let rect = target.get_bounding_client_rect();
                (rect.left(), rect.top())
            }),
            on_start: Arc::new(|_| true),
            on_move: Arc::new(|_| {}),
            on_end: Arc::new(|_| {}),
            _marker1: PhantomData,
            _marker2: PhantomData,
        }
    }
}

/// Argument for the `on_...` handler functions of [`UseDraggableOptions`].
pub struct UseDraggableCallbackArgs {
    /// Position of the `target` element
    pub position: Position,
    /// Original `PointerEvent` from the event listener
    pub event: PointerEvent,
}
