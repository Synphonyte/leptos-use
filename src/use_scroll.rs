use crate::core::EventTargetMaybeSignal;
use leptos::*;

#[allow(unused_variables)]
pub fn use_scroll<El, T, Fx, Fy>(
    cx: Scope,
    element: El,
    options: UseScrollOptions,
) -> UseScrollReturn
where
    (Scope, El): Into<EventTargetMaybeSignal<T>>,
    T: Into<web_sys::EventTarget> + Clone + 'static,
    Fx: Fn(f64),
    Fy: Fn(f64),
{
    // TODO : implement

    let (x, set_x) = create_signal(cx, 0.0);
    let (y, set_y) = create_signal(cx, 0.0);

    let (is_scrolling, _) = create_signal(cx, false);
    let (arrived_state, _) = create_signal(
        cx,
        Directions {
            left: false,
            right: false,
            top: false,
            bottom: false,
        },
    );
    let (directions, _) = create_signal(
        cx,
        Directions {
            left: false,
            right: false,
            top: false,
            bottom: false,
        },
    );

    UseScrollReturn {
        x: x.into(),
        set_x: Box::new(move |x| set_x.set(x)),
        y: y.into(),
        set_y: Box::new(move |y| set_y.set(y)),
        is_scrolling: is_scrolling.into(),
        arrived_state: arrived_state.into(),
        directions: directions.into(),
    }
}

/// Options for [`use_scroll`].
#[derive(Default)]
pub struct UseScrollOptions {
    /// Throttle time in milliseconds for the scroll events. Defaults to 0 (disabled).
    pub throttle: u32,

    /// After scrolling ends we wait idle + throttle milliseconds before we consider scrolling to have stopped.
    /// Defaults to 200.
    pub idle: u32,

    /// Threshold in pixels when we consider a side to have arrived (`UseScrollReturn::arrived_state`).
    pub offset: ScrollOffset,

    /// Callback when scrolling is happening.
    pub on_scroll: Option<Box<dyn Fn()>>,

    /// Callback when scrolling stops (after `idle` + `throttle` milliseconds have passed).
    pub on_stop: Option<Box<dyn Fn()>>,

    /// Options passed to the `addEventListener("scroll", ...)` call
    pub event_listener_options: Option<web_sys::AddEventListenerOptions>,

    /// When changing the `x` or `y` signals this specifies the scroll behaviour.
    /// Can be `Auto` (= not smooth) or `Smooth`. Defaults to `Auto`.
    pub behavior: ScrollBehavior,
}

#[derive(Default)]
pub enum ScrollBehavior {
    #[default]
    Auto,
    Smooth,
}

pub struct UseScrollReturn {
    pub x: Signal<f64>,
    pub set_x: Box<dyn Fn(f64)>,
    pub y: Signal<f64>,
    pub set_y: Box<dyn Fn(f64)>,
    pub is_scrolling: Signal<bool>,
    pub arrived_state: Signal<Directions>,
    pub directions: Signal<Directions>,
}

#[derive(Copy, Clone)]
pub struct Directions {
    pub left: bool,
    pub right: bool,
    pub top: bool,
    pub bottom: bool,
}

#[derive(Default)]
pub struct ScrollOffset {
    pub left: f64,
    pub top: f64,
    pub right: f64,
    pub bottom: f64,
}
