use crate::{
    IntoElementMaybeSignal, Position, UseDraggableCallbackArgs, UseDraggableOptions,
    UseDraggableReturn, UseEventListenerOptions, use_event_listener_with_options,
};
use leptos::ev::{pointerdown, pointermove, pointerup};
use leptos::prelude::*;
use leptos::reactive::wrappers::read::Signal;
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
/// # use leptos_use_draggable::{use_draggable_with_options, UseDraggableOptions, UseDraggableReturn};
/// # use leptos_use_core::Position;
/// #
/// # #[component]
/// # fn Demo() -> impl IntoView {
/// let el = NodeRef::<Div>::new();
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
pub fn use_draggable<El, M>(target: El) -> UseDraggableReturn
where
    El: IntoElementMaybeSignal<web_sys::EventTarget, M>,
{
    use_draggable_with_options::<El, M, _, _, _, _>(target, UseDraggableOptions::default())
}

/// Version of [`use_draggable`] that takes a `UseDraggableOptions`. See [`use_draggable`] for how to use.
pub fn use_draggable_with_options<El, M, DragEl, DragM, HandleEl, HandleM>(
    target: El,
    options: UseDraggableOptions<DragEl, DragM, HandleEl, HandleM>,
) -> UseDraggableReturn
where
    El: IntoElementMaybeSignal<web_sys::EventTarget, M>,
    DragEl: IntoElementMaybeSignal<web_sys::EventTarget, DragM>,
    HandleEl: IntoElementMaybeSignal<web_sys::EventTarget, HandleM>,
{
    let UseDraggableOptions {
        exact,
        prevent_default,
        stop_propagation,
        dragging_element,
        handle,
        pointer_types,
        initial_value,
        target_offset,
        on_start,
        on_move,
        on_end,
        ..
    } = options;

    let target = target.into_element_maybe_signal();

    let dragging_handle = if let Some(handle) = handle {
        handle.into_element_maybe_signal()
    } else {
        target
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
                let (x, y) = target_offset(target.unchecked_ref::<web_sys::EventTarget>().clone());
                let target: &web_sys::Element = target.unchecked_ref();

                if exact.get_untracked() && event_target::<web_sys::Element>(&event) != *target {
                    return;
                }

                #[allow(clippy::unnecessary_cast)]
                let grab_offset = Position {
                    x: event.client_x() as f64 - x,
                    y: event.client_y() as f64 - y,
                };

                #[cfg(debug_assertions)]
                let zone = leptos::reactive::diagnostics::SpecialNonReactiveZone::enter();

                if !on_start(UseDraggableCallbackArgs {
                    position: Position { x, y },
                    event: event.clone(),
                }) {
                    #[cfg(debug_assertions)]
                    drop(zone);
                    return;
                }

                #[cfg(debug_assertions)]
                drop(zone);

                set_start_position.set(Some(grab_offset));
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
                #[allow(clippy::unnecessary_cast)]
                let position = Position {
                    x: event.client_x() as f64 - start_position.x,
                    y: event.client_y() as f64 - start_position.y,
                };
                set_position.set(position);

                #[cfg(debug_assertions)]
                let zone = leptos::reactive::diagnostics::SpecialNonReactiveZone::enter();

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
        let zone = leptos::reactive::diagnostics::SpecialNonReactiveZone::enter();

        on_end(UseDraggableCallbackArgs {
            position: position.get_untracked(),
            event: event.clone(),
        });

        #[cfg(debug_assertions)]
        drop(zone);

        handle_event(event);
    };

    let dragging_element = dragging_element.into_element_maybe_signal();

    let passive_dragging_handle = Signal::derive(move || {
        if prevent_default.get() {
            None
        } else {
            dragging_handle.get()
        }
    });
    let non_passive_dragging_handle = Signal::derive(move || {
        if prevent_default.get() {
            dragging_handle.get()
        } else {
            None
        }
    });
    let passive_dragging_element = Signal::derive(move || {
        if prevent_default.get() {
            None
        } else {
            dragging_element.get()
        }
    });
    let non_passive_dragging_element = Signal::derive(move || {
        if prevent_default.get() {
            dragging_element.get()
        } else {
            None
        }
    });

    let passive_listener_options = UseEventListenerOptions::default()
        .capture(true)
        .passive(true);
    let non_passive_listener_options = UseEventListenerOptions::default()
        .capture(true)
        .passive(false);

    let _ = use_event_listener_with_options(
        passive_dragging_handle,
        pointerdown,
        on_pointer_down.clone(),
        passive_listener_options,
    );
    let _ = use_event_listener_with_options(
        non_passive_dragging_handle,
        pointerdown,
        on_pointer_down,
        non_passive_listener_options,
    );
    let _ = use_event_listener_with_options(
        passive_dragging_element,
        pointermove,
        on_pointer_move.clone(),
        passive_listener_options,
    );
    let _ = use_event_listener_with_options(
        non_passive_dragging_element,
        pointermove,
        on_pointer_move,
        non_passive_listener_options,
    );
    let _ = use_event_listener_with_options(
        passive_dragging_element,
        pointerup,
        on_pointer_up.clone(),
        passive_listener_options,
    );
    let _ = use_event_listener_with_options(
        non_passive_dragging_element,
        pointerup,
        on_pointer_up,
        non_passive_listener_options,
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
