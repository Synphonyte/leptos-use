use crate::core::ElementMaybeSignal;
use crate::use_debounce_fn_with_arg;
use crate::utils::{CloneableFn, CloneableFnWithArg, CloneableFnWithReturn};
use leptos::*;

/// We have to check if the scroll amount is close enough to some threshold in order to
/// more accurately calculate arrivedState. This is because scrollTop/scrollLeft are non-rounded
/// numbers, while scrollHeight/scrollWidth and clientHeight/clientWidth are rounded.
/// https://developer.mozilla.org/en-US/docs/Web/API/Element/scrollHeight#determine_if_an_element_has_been_totally_scrolled
const ARRIVED_STATE_THRESHOLD_PIXELS: f64 = 1.0;

#[allow(unused_variables)]
pub fn use_scroll_with_options<El, T>(
    cx: Scope,
    element: El,
    options: UseScrollOptions,
) -> UseScrollReturn
where
    (Scope, El): Into<ElementMaybeSignal<T, web_sys::Element>>,
    T: Into<web_sys::Element> + Clone + 'static,
{
    // TODO : implement

    let (internal_x, set_internal_x) = create_signal(cx, 0.0);
    let (internal_y, set_internal_y) = create_signal(cx, 0.0);

    let signal = (cx, element).into();
    let behavior = options.behavior;

    let scroll_to = move |x: Option<f64>, y: Option<f64>| {
        let element = signal.get_untracked();

        if let Some(element) = element {
            let element = element.into();

            let mut scroll_options = web_sys::ScrollToOptions::new();
            scroll_options.behavior(behavior.into());

            if let Some(x) = x {
                scroll_options.left(x);
            }
            if let Some(y) = y {
                scroll_options.top(y);
            }

            element.scroll_to_with_scroll_to_options(&scroll_options);
        }
    };

    let scroll = scroll_to.clone();
    let set_x = Box::new(move |x| scroll(Some(x), None));

    let scroll = scroll_to.clone();
    let set_y = Box::new(move |y| scroll(None, Some(y)));

    let (is_scrolling, set_is_scrolling) = create_signal(cx, false);
    let (arrived_state, set_arrived_state) = create_signal(
        cx,
        Directions {
            left: true,
            right: false,
            top: true,
            bottom: false,
        },
    );
    let (directions, set_directions) = create_signal(
        cx,
        Directions {
            left: false,
            right: false,
            top: false,
            bottom: false,
        },
    );

    let on_stop = options.on_stop;
    let on_scroll_end = move |e| {
        if !is_scrolling.get() {
            return;
        }

        set_is_scrolling(false);
        set_directions.update(|directions| {
            directions.left = false;
            directions.right = false;
            directions.top = false;
            directions.bottom = false;
            on_stop(e);
        });
    };
    let on_scroll_end_debounced =
        use_debounce_fn_with_arg(on_scroll_end, options.throttle + options.idle);

    UseScrollReturn {
        x: internal_x.into(),
        set_x,
        y: internal_y.into(),
        set_y,
        is_scrolling: is_scrolling.into(),
        arrived_state: arrived_state.into(),
        directions: directions.into(),
    }
}

/// Options for [`use_scroll`].
pub struct UseScrollOptions {
    /// Throttle time in milliseconds for the scroll events. Defaults to 0 (disabled).
    pub throttle: f64,

    /// After scrolling ends we wait idle + throttle milliseconds before we consider scrolling to have stopped.
    /// Defaults to 200.
    pub idle: f64,

    /// Threshold in pixels when we consider a side to have arrived (`UseScrollReturn::arrived_state`).
    pub offset: ScrollOffset,

    /// Callback when scrolling is happening.
    pub on_scroll: Box<dyn CloneableFn>,

    /// Callback when scrolling stops (after `idle` + `throttle` milliseconds have passed).
    pub on_stop: Box<dyn CloneableFnWithArg<web_sys::Event>>,

    /// Options passed to the `addEventListener("scroll", ...)` call
    pub event_listener_options: web_sys::AddEventListenerOptions,

    /// When changing the `x` or `y` signals this specifies the scroll behaviour.
    /// Can be `Auto` (= not smooth) or `Smooth`. Defaults to `Auto`.
    pub behavior: ScrollBehavior,
}

impl Default for UseScrollOptions {
    fn default() -> Self {
        Self {
            throttle: 0.0,
            idle: 200.0,
            offset: Default::default(),
            on_scroll: Default::default(),
            on_stop: Default::default(),
            event_listener_options: Default::default(),
            behavior: Default::default(),
        }
    }
}

#[derive(Default, Copy, Clone)]
pub enum ScrollBehavior {
    #[default]
    Auto,
    Smooth,
}

impl Into<web_sys::ScrollBehavior> for ScrollBehavior {
    fn into(self) -> web_sys::ScrollBehavior {
        match self {
            ScrollBehavior::Auto => web_sys::ScrollBehavior::Auto,
            ScrollBehavior::Smooth => web_sys::ScrollBehavior::Smooth,
        }
    }
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

#[derive(Default, Copy, Clone)]
pub struct ScrollOffset {
    pub left: f64,
    pub top: f64,
    pub right: f64,
    pub bottom: f64,
}
