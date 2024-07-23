use crate::core::{ElementMaybeSignal, MaybeRwSignal, PointerType, Position};
use crate::{use_event_listener_with_options, use_window, UseEventListenerOptions, UseWindow};
use default_struct_builder::DefaultBuilder;
use leptos::ev::{pointerdown, pointermove, pointerup};
use leptos::prelude::diagnostics::SpecialNonReactiveZone;
use leptos::prelude::wrappers::read::Signal;
use leptos::prelude::*;
use send_wrapper::SendWrapper;
use std::marker::PhantomData;
use std::sync::Arc;
use wasm_bindgen::JsCast;
use web_sys::PointerEvent;

/// Make elements draggable.
///
/// ## Demo
///
/// [Link to Demo](https://github.com/Synphonyte/leptos-use/tree/main/examples/use_draggable)
///
/// ## Usage
///
/// ```
/// # use leptos::prelude::*;
/// # use leptos::html::Div;
/// # use leptos_use::{use_draggable_with_options, UseDraggableOptions, UseDraggableReturn};
/// # use leptos_use::core::Position;
/// #
/// # #[component]
/// # fn Demo() -> impl IntoView {
/// let el = create_node_ref::<Div>();
///
/// // `style` is a helper string "left: {x}px; top: {y}px;"
/// let UseDraggableReturn {
///     x,
///     y,
///     style,
///     ..
/// } = use_draggable_with_options(
///     el,
///     UseDraggableOptions::default().initial_value(Position { x: 40.0, y: 40.0 }),
/// );
///
/// view! {
///     <div node_ref=el style=move || format!("position: fixed; {}", style.get())>
///         Drag me! I am at { x }, { y }
///     </div>
/// }
/// # }
/// ```
pub fn use_draggable<El, T>(target: El) -> UseDraggableReturn
where
    El: Into<ElementMaybeSignal<T, web_sys::EventTarget>>,
    T: Into<web_sys::EventTarget> + Clone + 'static,
{
    use_draggable_with_options::<
        El,
        T,
        UseWindow,
        web_sys::Window,
        web_sys::EventTarget,
        web_sys::EventTarget,
    >(target, UseDraggableOptions::default())
}

/// Version of [`use_draggable`] that takes a `UseDraggableOptions`. See [`use_draggable`] for how to use.
pub fn use_draggable_with_options<El, T, DragEl, DragT, HandleEl, HandleT>(
    target: El,
    options: UseDraggableOptions<DragEl, DragT, HandleEl, HandleT>,
) -> UseDraggableReturn
where
    El: Into<ElementMaybeSignal<T, web_sys::EventTarget>>,
    T: Into<web_sys::EventTarget> + Clone + 'static,
    DragEl: Clone,
    DragEl: Into<ElementMaybeSignal<DragT, web_sys::EventTarget>>,
    DragT: Into<web_sys::EventTarget> + Clone + 'static,
    HandleEl: Into<ElementMaybeSignal<HandleT, web_sys::EventTarget>>,
    HandleT: Into<web_sys::EventTarget> + Clone + 'static,
{
    let UseDraggableOptions {
        exact,
        prevent_default,
        stop_propagation,
        dragging_element,
        handle,
        pointer_types,
        initial_value,
        on_start,
        on_move,
        on_end,
        ..
    } = options;

    let target = target.into();

    let dragging_handle = if let Some(handle) = handle {
        // let handle = Signal::derive(|| SendWrapper::new(handle));
        let handle: ElementMaybeSignal<_, _> = handle.into();
        Signal::derive(move || handle.get().map(|handle| SendWrapper::new(handle.into())))
    } else {
        let target = target.clone();
        Signal::derive(move || target.get().map(|target| SendWrapper::new(target.into())))
    };

    let (position, set_position) = initial_value.into_signal();
    let (start_position, set_start_position) = signal(None::<Position>);

    let filter_event = move |event: &PointerEvent| {
        let ty = event.pointer_type();
        pointer_types.iter().any(|p| p.to_string() == ty)
    };

    let handle_event = move |event: PointerEvent| {
        if prevent_default.get_untracked() {
            event.prevent_default();
        }
        if stop_propagation.get_untracked() {
            event.stop_propagation();
        }
    };

    let on_pointer_down = {
        let filter_event = filter_event.clone();

        move |event: PointerEvent| {
            if !filter_event(&event) {
                return;
            }

            if let Some(target) = target.get_untracked() {
                let target: web_sys::Element = target.into().unchecked_into();

                if exact.get_untracked() && event_target::<web_sys::Element>(&event) != target {
                    return;
                }

                let rect = target.get_bounding_client_rect();
                let position = Position {
                    x: event.client_x() as f64 - rect.left(),
                    y: event.client_y() as f64 - rect.top(),
                };

                #[cfg(debug_assertions)]
                let zone = SpecialNonReactiveZone::enter();

                if !on_start(UseDraggableCallbackArgs {
                    position,
                    event: event.clone(),
                }) {
                    #[cfg(debug_assertions)]
                    drop(zone);
                    return;
                }

                #[cfg(debug_assertions)]
                drop(zone);

                set_start_position.set(Some(position));
                handle_event(event);
            }
        }
    };

    let on_pointer_move = {
        let filter_event = filter_event.clone();

        move |event: PointerEvent| {
            if !filter_event(&event) {
                return;
            }
            if let Some(start_position) = start_position.get_untracked() {
                let position = Position {
                    x: event.client_x() as f64 - start_position.x,
                    y: event.client_y() as f64 - start_position.y,
                };
                set_position.set(position);

                #[cfg(debug_assertions)]
                let zone = SpecialNonReactiveZone::enter();

                on_move(UseDraggableCallbackArgs {
                    position,
                    event: event.clone(),
                });

                #[cfg(debug_assertions)]
                drop(zone);

                handle_event(event);
            }
        }
    };

    let on_pointer_up = move |event: PointerEvent| {
        if !filter_event(&event) {
            return;
        }
        if start_position.get_untracked().is_none() {
            return;
        }
        set_start_position.set(None);

        #[cfg(debug_assertions)]
        let zone = SpecialNonReactiveZone::enter();

        on_end(UseDraggableCallbackArgs {
            position: position.get_untracked(),
            event: event.clone(),
        });

        #[cfg(debug_assertions)]
        drop(zone);

        handle_event(event);
    };

    let listener_options = UseEventListenerOptions::default().capture(true);

    let _ = use_event_listener_with_options(
        dragging_handle,
        pointerdown,
        on_pointer_down,
        listener_options,
    );
    let _ = use_event_listener_with_options(
        dragging_element.clone(),
        pointermove,
        on_pointer_move,
        listener_options,
    );
    let _ = use_event_listener_with_options(
        dragging_element,
        pointerup,
        on_pointer_up,
        listener_options,
    );

    UseDraggableReturn {
        x: Signal::derive(move || position.get().x),
        y: Signal::derive(move || position.get().y),
        position,
        set_position,
        is_dragging: Signal::derive(move || start_position.get().is_some()),
        style: Signal::derive(move || {
            let position = position.get();
            format!("left: {}px; top: {}px;", position.x, position.y)
        }),
    }
}

/// Options for [`use_draggable_with_options`].
#[derive(DefaultBuilder)]
pub struct UseDraggableOptions<DragEl, DragT, HandleEl, HandleT>
where
    DragEl: Into<ElementMaybeSignal<DragT, web_sys::EventTarget>>,
    DragT: Into<web_sys::EventTarget> + Clone + 'static,
    HandleEl: Into<ElementMaybeSignal<HandleT, web_sys::EventTarget>>,
    HandleT: Into<web_sys::EventTarget> + Clone + 'static,
{
    /// Only start the dragging when click on the element directly. Defaults to `false`.
    #[builder(into)]
    exact: MaybeSignal<bool>,

    /// Prevent events defaults. Defaults to `false`.
    #[builder(into)]
    prevent_default: MaybeSignal<bool>,

    /// Prevent events propagation. Defaults to `false`.
    #[builder(into)]
    stop_propagation: MaybeSignal<bool>,

    /// Element to attach `pointermove` and `pointerup` events to. Defaults to `window`.
    dragging_element: DragEl,

    /// Handle that triggers the drag event. Defaults to `target`.
    handle: Option<HandleEl>,

    /// Pointer types that listen to. Defaults to `[Mouse, Touch, Pen]`.
    pointer_types: Vec<PointerType>,

    /// Initial position of the element. Defaults to `{ x: 0, y: 0 }`.
    #[builder(into)]
    initial_value: MaybeRwSignal<Position>,

    /// Callback when the dragging starts. Return `false` to prevent dragging.
    on_start: Arc<dyn Fn(UseDraggableCallbackArgs) -> bool + Send + Sync>,

    /// Callback during dragging.
    on_move: Arc<dyn Fn(UseDraggableCallbackArgs) + Send + Sync>,

    /// Callback when dragging end.
    on_end: Arc<dyn Fn(UseDraggableCallbackArgs) + Send + Sync>,

    #[builder(skip)]
    _marker1: PhantomData<DragT>,
    #[builder(skip)]
    _marker2: PhantomData<HandleT>,
}

impl Default
    for UseDraggableOptions<UseWindow, web_sys::Window, web_sys::EventTarget, web_sys::EventTarget>
{
    fn default() -> Self {
        Self {
            exact: MaybeSignal::default(),
            prevent_default: MaybeSignal::default(),
            stop_propagation: MaybeSignal::default(),
            dragging_element: use_window(),
            handle: None,
            pointer_types: vec![PointerType::Mouse, PointerType::Touch, PointerType::Pen],
            initial_value: MaybeRwSignal::default(),
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

/// Return type of [`use_draggable`].
pub struct UseDraggableReturn {
    /// X coordinate of the element
    pub x: Signal<f64>,
    /// Y coordinate of the element
    pub y: Signal<f64>,
    /// Position of the element
    pub position: Signal<Position>,
    /// Set the position of the element manually
    pub set_position: WriteSignal<Position>,
    /// Whether the element is being dragged
    pub is_dragging: Signal<bool>,
    /// Style attribute "left: {x}px; top: {y}px;"
    pub style: Signal<String>,
}
